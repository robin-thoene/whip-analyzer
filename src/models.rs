//
// TODO: improve encapsulation
//

use std::fmt::Display;

#[derive(Debug)]
pub struct CdResult {
    pub log_file_path: String,
    pub track_results: Vec<TrackResult>,
}

impl CdResult {
    pub fn is_rip_good(&self) -> bool {
        self.track_results.iter().all(|x| x.is_rip_good())
    }
}

#[derive(Debug)]
pub struct TrackResult {
    pub file_name: String,
    pub quality: String,
    pub accurate_rip_v1_result: String,
    pub accurate_rip_v2_result: String,
    pub status: String,
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
