pub mod render;
use crate::project::ProjectRoot;
use clap::Parser;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    #[arg(
        short, long,
        default_value_t = ProjectRoot::WorkingDirectory, 
        value_parser = parse_project_dir

    )]
    pub directory: ProjectRoot,
    #[arg(short, long, default_value = "false")]
    pub one_line: bool,
}

fn parse_project_dir(s: &str) -> Result<ProjectRoot, Box<dyn Error + Send + Sync + 'static>> {
    Ok(ProjectRoot::Explicit(s.into()))
}
