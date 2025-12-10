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
    pub is_accurate_rip: bool,
    pub status: Option<String>,
}

impl TrackResult {
    pub fn is_rip_good(&self) -> bool {
        self.is_accurate_rip
            && self.status.as_deref() == Some("Copy OK")
            && self.quality.as_deref() == Some("100.00 %")
    }
}
