/* Okay new idea. Go for same idea as actionlint: get a hold of a yaml node and just cycle through
 * its fields or whatever. yaml-peg looks like it has everything needed, especially _where_ an item
 * is in the document literally.
 *
 *  pub struct Marked<T> {
 *     v: T,
 *     pos: u64,
 *     annotations: Vec<Annotation>
 *  }
 *
 *  enum AnnotationLevel { ... }
 *
 *  struct Annotation {
 *      level: AnnotationLevel,
 *      msg: String
 *  }
 *
 *  trait WorkflowAnalyzer {
 *      fn analyze(&Self, &mut MarkedWorkflow, &Project) -> Result<(), Error>
 *  }
 *
 *
 *  struct Project {
 *      workflows: WorkflowCache,
 *      actions: ActionCache,
 *      conifg: Config
 *  }
 *
 * Just tag everything all the way down so an error can be properly marked from where ever the code
 * currently is but warning: this could be cumbersome
 *
 * yaml-peg indicated_msg has an example of converting u64 back to (line, col)
 *
 * Also possum _really_ wants to eat trash and ticks so throwing Option on _everything_ is
 * perfectly fine, we'll just hiss about it.
 *
 * DocumentPointer(u64); <- opaque reference used to determine (line, col)
 * DocumentPosition(u64, u64);
 * struct Document {
 *      lines: Vec<Vec<u8>>
 *  }
 *
 *  impl for Document {
 *      fn pos_at(&Self, &DocumentPointer) -> Result<DocumentPosition, Error>
 *  }
 *
 * CLI Commands:
 *
 * possum lint/hiss <directory | .>
 * possum search/rummage <directory | .>
 */
#![allow(dead_code, unused_variables)]
mod cli;
mod scavenge;
mod workflow;
use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

fn main() {
    let mut args = cli::Arguments::parse();
    args.directory.push(".github");
    args.directory.push("workflows");

    println!("looking at dir {}", args.directory.display());

    assert!(args.directory.exists());
    assert!(args.directory.is_dir());

    let mut workflows: HashMap<PathBuf, workflow::Workflow> = HashMap::new();

    for p in args.directory.read_dir().unwrap() {
        let p = p.unwrap().path();

        if !p.is_file() {
            continue;
        }

        if let Some(s) = p.extension() {
            if s == "yaml" || s == "yml" {
                let raw = File::open(p.clone()).expect("whoops, couldn't open that file");
                let mut reader = BufReader::new(raw);
                let mut contents = String::new();
                reader
                    .read_to_string(&mut contents)
                    .expect("whoops couldn't read that file");
                match scavenge::parse_workflow(contents.as_bytes()) {
                    Ok(wf) => {
                        workflows.insert(p, wf);
                    }
                    Err(s) => println!("failure in {} lmao: {:#?}", p.display(), s),
                }
            }
        } else {
            continue;
        }
    }

    println!("{:#?}", workflows);
}
