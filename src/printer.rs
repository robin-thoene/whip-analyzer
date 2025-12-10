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
    is_accurate_rip: bool,
    status: String,
}

pub fn print_results(cd_results: Vec<CdResult>, only_errors: bool) {
    let fallback = "unknown";
    let mut rows = vec![];

    // Build data
    for res in cd_results {
        for track_res in res.track_results {
            if only_errors && track_res.is_rip_good() {
                continue;
            }
            rows.push(Row {
                artist: res.artist.clone().unwrap_or(fallback.into()),
                cd_name: res.title.clone().unwrap_or(fallback.into()),
                status: track_res.status.unwrap_or(fallback.into()),
                quality: track_res.quality.unwrap_or(fallback.into()),
                file_name: track_res.song_name.unwrap_or(fallback.into()),
                is_accurate_rip: track_res.is_accurate_rip,
            });
        }
    }

    // Get terminal size
    let (width, _height) = get_terminal_size();

    // Build general table style
    let header_color = if only_errors {
        Color::FG_RED
    } else {
        Color::FG_GREEN
    };
    let settings = Settings::default()
        .with(Style::modern_rounded())
        .with(Width::wrap(width).priority(Priority::max(true)))
        .with(Width::increase(width));
    let mut table = Table::new(rows);
    table.with(settings);
    table.modify(Rows::first(), header_color);

    // Print
    print!("{}", table);
}

fn get_terminal_size() -> (usize, usize) {
    let (TerminalWidth(width), TerminalHeight(height)) =
        terminal_size().expect("failed to obtain a terminal size");

    (width as usize, height as usize)
}
