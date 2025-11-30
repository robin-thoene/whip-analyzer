//
// TODO: improve encapsulation
//

use clap::{self, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, help = "The path to the root directory of ripped CD data")]
    pub path: PathBuf,
}

pub fn get_cli_args() -> Args {
    Args::parse()
}
