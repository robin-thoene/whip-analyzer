use clap::Parser;
use env_logger::{Builder, Env};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "The path to the root directory of ripped CD data")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    setup_logger();
    let cd_results = get_cd_rip_results(&args.path);
    print_results(cd_results);
}

fn setup_logger() {
    let env = Env::default().filter_or("RUST_LOG", "info");
    Builder::from_env(env).init();
    log::debug!("configured logger");
}

fn get_cd_rip_results(root_path: &Path) -> Vec<CdResult> {
    log::debug!("getting CD rip results in {} ...", root_path.display());
    let mut cd_results = vec![];
    for entry in WalkDir::new(root_path) {
        if let Ok(e) = entry {
            if e.file_type().is_file() {
                if let Some(file_name) = e.file_name().to_str() {
                    if file_name.ends_with(".log") {
                        log::debug!("found log file {}", file_name);
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
    log::debug!("done, got {} results", cd_results.len());
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
            println!("{}", track_res);
        }
    }
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

impl Display for TrackResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: improve
        write!(
            f,
            "
file: {}
is good?: {}
quality: {}
v1: {}
v2: {}
status: {}
            ",
            self.file_name,
            self.is_rip_good(),
            self.quality,
            self.accurate_rip_v1_result,
            self.accurate_rip_v2_result,
            self.status
        )
    }
}
