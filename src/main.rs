/*
 * CLI Commands:
 *
 * possum lint/hiss <directory | .>
 * possum search/rummage <directory | .>
 */
#![allow(dead_code, unused_variables)]
mod cli;
mod document;
mod lint;
mod project;
mod scavenge;
mod workflow;

use clap::Parser;
use cli::render::{DebugRender, OneLineRender};
use cli::Arguments;
use project::builder::build;

fn main() {
    let args = Arguments::try_parse().unwrap();
    let project = build(args.directory);

    match project {
        Err(err) => println!("{:#?}", err),
        Ok(proj) => {
            if args.one_line {
                print!("{}", OneLineRender(proj));
            } else {
                print!("{}", DebugRender(proj));
            }
        }
    }
}
