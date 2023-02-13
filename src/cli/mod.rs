pub mod render;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    #[arg(short, long, default_value = ".")]
    pub directory: PathBuf,
    #[arg(short, long, default_value = "false")]
    pub one_line: bool,
}
