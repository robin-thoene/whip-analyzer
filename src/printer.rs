use crate::models::CdResult;

pub fn print_results(cd_results: Vec<CdResult>) {
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
