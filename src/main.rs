use clap::Parser;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "The path to the root directory of ripped CD data")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let cd_results = get_cd_rip_results(&args.path);
    print_results(cd_results);
}

fn get_cd_rip_results(root_path: &Path) -> Vec<CdResult> {
    let mut cd_results = vec![];
    for entry in WalkDir::new(root_path) {
        if let Ok(e) = entry {
            if e.file_type().is_file() {
                if let Some(file_name) = e.file_name().to_str() {
                    if file_name.ends_with(".log") {
                        // TODO: handle
                        let cd_result = parse_log_file(e.path());
                        cd_results.push(cd_result);
                    }
                } else {
                    // TODO: log entry
                }
            }
        } else {
            // TODO: log entry
        }
    }
    cd_results
}

fn print_results(cd_results: Vec<CdResult>) {
    // TODO: improve visual display of results
    for res in cd_results {
        println!("-------------------------");
        println!(
            "
            log file: {}
            overall status: {}
            ",
            res.log_file_path,
            res.is_rip_good()
        );
        println!("\nTracks:");

        for track_res in res.track_results {
            println!(
                "
                file: {}
                is good?: {}
                quality: {}
                v1: {}
                v2: {}
                status: {}
                ",
                track_res.file_name,
                track_res.is_rip_good(),
                track_res.quality,
                track_res.accurate_rip_v1_result,
                track_res.accurate_rip_v2_result,
                track_res.status
            );
        }
    }
}

fn parse_log_file(path: &Path) -> CdResult {
    let test = TrackResult {
        status: "Ok".to_string(),
        quality: "100%".to_string(),
        file_name: "test.flac".to_string(),
        accurate_rip_v1_result: "yo".to_string(),
        accurate_rip_v2_result: "yo".to_string(),
    };
    CdResult {
        log_file_path: path.display().to_string(),
        track_results: vec![test], // TODO: impl
    }
}

#[derive(Debug)]
struct CdResult {
    log_file_path: String,
    track_results: Vec<TrackResult>,
}

impl CdResult {
    pub fn is_rip_good(&self) -> bool {
        self.track_results.iter().all(|x| x.is_rip_good())
    }
}

#[derive(Debug)]
struct TrackResult {
    file_name: String,
    quality: String,
    accurate_rip_v1_result: String,
    accurate_rip_v2_result: String,
    status: String,
}

impl TrackResult {
    pub fn is_rip_good(&self) -> bool {
        // TODO: implement
        false
    }
}
