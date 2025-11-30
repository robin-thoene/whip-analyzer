use std::path::Path;

use walkdir::WalkDir;

use crate::models::{CdResult, TrackResult};

pub fn get_cd_rip_results(root_path: &Path) -> Vec<CdResult> {
    log::debug!("getting CD rip results in {} ...", root_path.display());
    let mut cd_results = vec![];
    for entry in WalkDir::new(root_path) {
        if let Ok(e) = entry {
            if e.file_type().is_file() {
                if let Some(file_name) = e.file_name().to_str() {
                    if file_name.ends_with(".log") {
                        log::info!("found log file {}", file_name);
                        let cd_result = parse_log_file(e.path());
                        cd_results.push(cd_result);
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
        } else {
            log::error!("error occured while getting directory entry");
        }
    }
    log::debug!("done");
    log::info!("found {} CD rip results", cd_results.len());
    cd_results
}

fn parse_log_file(path: &Path) -> CdResult {
    log::debug!("starting to parse log file at {}", path.display());
    // TODO: do the parsing
    let test = TrackResult {
        status: "Ok".to_string(),
        quality: "100%".to_string(),
        file_name: "test.flac".to_string(),
        accurate_rip_v1_result: "yo".to_string(),
        accurate_rip_v2_result: "yo".to_string(),
    };
    log::debug!("done");
    log::info!("parsed log file {}", path.display());
    CdResult {
        log_file_path: path.display().to_string(),
        track_results: vec![test], // TODO: impl
    }
}
