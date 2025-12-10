use crate::models::CdResult;
use tabled::{
    Table, Tabled,
    settings::{Color, Settings, Style, Width, object::Rows, peaker::Priority},
};
use terminal_size::{Height as TerminalHeight, Width as TerminalWidth, terminal_size};

#[derive(Tabled)]
#[tabled(rename_all = "UPPERCASE")]
struct Row {
    artist: String,
    cd_name: String,
    file_name: String,
    quality: String,
    accurate_rip_v1_result: String,
    accurate_rip_v2_result: String,
    status: String,
}

pub fn print_results(cd_results: Vec<CdResult>) {
    let fallback = "unknown";
    let mut rows = vec![];

    // Build data
    for res in cd_results {
        for track_res in res.track_results {
            rows.push(Row {
                artist: res.artist.clone().unwrap_or(fallback.into()),
                cd_name: res.title.clone().unwrap_or(fallback.into()),
                status: track_res.status.unwrap_or(fallback.into()),
                quality: track_res.quality.unwrap_or(fallback.into()),
                file_name: track_res.song_name.unwrap_or(fallback.into()),
                accurate_rip_v1_result: track_res.accurate_rip_v1_result.unwrap_or(fallback.into()),
                accurate_rip_v2_result: track_res.accurate_rip_v2_result.unwrap_or(fallback.into()),
            });
        }
    }

    // Get terminal size
    let (width, _height) = get_terminal_size();

    // Build general table style
    let settings = Settings::default()
        .with(Style::modern_rounded())
        .with(Width::wrap(width).priority(Priority::max(true)))
        .with(Width::increase(width));
    let mut table = Table::new(rows);
    table.with(settings);
    table.modify(Rows::first(), Color::FG_GREEN);

    // Print
    print!("{}", table);
}

fn get_terminal_size() -> (usize, usize) {
    let (TerminalWidth(width), TerminalHeight(height)) =
        terminal_size().expect("failed to obtain a terminal size");

    (width as usize, height as usize)
}
