use crate::models::{CdResult, TrackResult};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
use walkdir::WalkDir;

pub fn parse_cd_rip_results(root_path: &Path) -> Vec<CdResult> {
    log::debug!("getting CD rip results in {} ...", root_path.display());
    let mut cd_results = vec![];
    for entry in WalkDir::new(root_path) {
        match entry {
            Ok(e) => {
                if e.file_type().is_file() {
                    if let Some(file_name) = e.file_name().to_str() {
                        if file_name.ends_with(".log") {
                            log::info!("found log file {}", file_name);
                            let cd_result = parse_cd_rip_log_file(e.path());
                            if let Some(res) = cd_result {
                                cd_results.push(res);
                            }
                        } else {
                            log::debug!(
                                "file {} is not a log file, skipping",
                                e.file_name().display()
                            );
                        }
                    } else {
                        log::error!(
                            "could not get string value for file name {}",
                            e.file_name().display()
                        );
                    }
                } else {
                    log::debug!("{} is not a file, skipping", e.file_name().display());
                }
            }
            Err(err) => {
                log::error!("error occured while getting directory entry: {}", err);
            }
        }
    }
    log::debug!("done");
    log::info!("found {} CD rip results", cd_results.len());
    cd_results
}

fn parse_cd_rip_log_file(path: &Path) -> Option<CdResult> {
    log::debug!("starting to parse log file at {}", path.display());
    let file = File::open(path);
    match file {
        Ok(file) => {
            // keep track of what to parse at the moment
            let mut mode = LogFileParsingMode::default();
            // create result variables
            let mut track_results = vec![];
            let mut title: Option<String> = None;
            let mut artist: Option<String> = None;
            let mut is_new_track = true;
            // create temporary placeholder for building track results based on multiple lines
            let mut tmp_track_result = TrackResult::default();

            for line in BufReader::new(file).lines() {
                match line {
                    Ok(line) => {
                        log::debug!("mode is {:?}", mode);
                        log::debug!("handling line: {}", line);
                        match mode {
                            LogFileParsingMode::Start => {
                                if line.contains("CD metadata:") {
                                    mode = LogFileParsingMode::ReadCdMetadata;
                                }
                            }
                            LogFileParsingMode::ReadCdMetadata => {
                                if let Some(("Artist", value)) = line.trim().split_once(':') {
                                    artist = Some(value.trim().to_string());
                                }
                                if let Some(("Title", value)) = line.trim().split_once(':') {
                                    title = Some(value.trim().to_string());
                                }
                                if line.contains("Tracks:") {
                                    mode = LogFileParsingMode::ReadTracks;
                                }
                            }
                            LogFileParsingMode::ReadTracks => {
                                if is_new_track {
                                    is_new_track = false;
                                    log::debug!("starting to process track number {} now", line);
                                }
                                if line.trim().is_empty() {
                                    // save the track
                                    track_results.push(tmp_track_result.clone());
                                    // ensure the tmp var is reset
                                    tmp_track_result = TrackResult::default();
                                    // signal to the parser that the next line is a new track
                                    is_new_track = true;
                                    log::debug!(
                                        "finished processing track: {:?}",
                                        tmp_track_result
                                    );
                                    continue;
                                }
                                if let Some(("Filename", value)) = line.trim().split_once(':') {
                                    let song_name =
                                        value.split('-').next_back().unwrap_or_default().trim();
                                    tmp_track_result.song_name = Some(song_name.to_string());
                                }
                                if let Some(("Extraction quality", value)) =
                                    line.trim().split_once(':')
                                {
                                    tmp_track_result.quality = Some(value.trim().to_string());
                                }
                                if let Some(("Status", value)) = line.trim().split_once(':') {
                                    tmp_track_result.status = Some(value.trim().to_string());
                                }
                                if line.contains("Result: Found, exact match") {
                                    tmp_track_result.is_accurate_rip = true;
                                }
                                if line.contains("Conclusive status report:") {
                                    mode = LogFileParsingMode::End;
                                }
                            }
                            LogFileParsingMode::End => {
                                log::debug!("reached end mode, not reading any furhter lines");
                                break;
                            }
                        }
                    }
                    Err(err) => {
                        log::error!("error reading line: {}", err);
                    }
                }
            }

            log::debug!("done");
            log::info!("parsed log file {}", path.display());

            Some(CdResult {
                artist,
                title,
                log_file_path: path.display().to_string(),
                track_results,
            })
        }
        Err(err) => {
            log::error!(
                "error occurred while opening log file {}: {}",
                path.display(),
                err
            );
            None
        }
    }
}

#[derive(Default, Debug, PartialEq)]
enum LogFileParsingMode {
    #[default]
    Start,
    ReadCdMetadata,
    ReadTracks,
    End,
}
