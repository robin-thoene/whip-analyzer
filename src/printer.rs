use crate::models::CdResult;

pub fn print_results(cd_results: Vec<CdResult>) {
    let fallback = "unknown";
    // TODO: improve visual display of results
    for res in cd_results {
        println!("-------------------------");
        println!(
            "
artist: {}
CD name: {}
log file: {}
overall status: {}
            ",
            res.artist.as_deref().unwrap_or(fallback),
            res.title.as_deref().unwrap_or(fallback),
            res.log_file_path,
            res.is_rip_good()
        );
        println!("\nTracks:");

        for track_res in res.track_results {
            println!("{}", track_res);
        }
    }
}
