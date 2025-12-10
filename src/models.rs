//
// TODO: improve encapsulation
//

use std::fmt::Display;

#[derive(Debug)]
pub struct CdResult {
    pub artist: Option<String>,
    pub title: Option<String>,
    pub log_file_path: String,
    pub track_results: Vec<TrackResult>,
}

impl CdResult {
    pub fn is_rip_good(&self) -> bool {
        self.track_results.iter().all(|x| x.is_rip_good())
    }
}

#[derive(Debug, Default, Clone)]
pub struct TrackResult {
    pub song_name: Option<String>,
    pub quality: Option<String>,
    pub accurate_rip_v1_result: Option<String>,
    pub accurate_rip_v2_result: Option<String>,
    pub status: Option<String>,
}

impl TrackResult {
    pub fn is_rip_good(&self) -> bool {
        // TODO: implement
        false
    }
}

impl Display for TrackResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fallback = "unknown";
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
            self.song_name.as_deref().unwrap_or(fallback),
            self.is_rip_good(),
            self.quality.as_deref().unwrap_or(fallback),
            self.accurate_rip_v1_result.as_deref().unwrap_or(fallback),
            self.accurate_rip_v2_result.as_deref().unwrap_or(fallback),
            self.status.as_deref().unwrap_or(fallback)
        )
    }
}
