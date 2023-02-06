mod cli;
mod workflow;
use serde_yaml;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use clap::Parser;

fn main() {
    let args: cli::Arguments = cli::Arguments::parse();
    let mut path = args.directory;
    path.push(".github");
    path.push("workflows");

    assert!(path.exists());
    assert!(path.is_dir());

    let mut workflows: HashMap<PathBuf, workflow::Workflow> = HashMap::new();

    for p in path.read_dir().unwrap() {
        let p = p.unwrap().path();
        if !p.is_file() {
            continue;
        }

        let ext = p.extension().unwrap();

        if ext == "yaml" || ext == "yml" {
            let fh = fs::File::open(p.clone()).unwrap();
            let workflow: workflow::Workflow =
                serde_yaml::from_reader(fh).expect("Failed to deserialize");
            workflows.insert(p, workflow);
        }
    }

    assert!(!workflows.is_empty());

    println!("{:#?}", workflows);
}
