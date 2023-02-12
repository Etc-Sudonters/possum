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
mod action;
mod cli;
use std::collections::HashMap;
mod document;
mod project;
use yaml_peg::parser::Loader;
mod scavenge;
mod workflow;
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use yaml_peg::repr::RcRepr;

use std::path::PathBuf;

use crate::document::Annotations;

fn main() {
    let mut args = cli::Arguments::parse();
    args.directory.push(".github");
    args.directory.push("workflows");

    println!("looking at dir {}", args.directory.display());

    assert!(args.directory.exists());
    assert!(args.directory.is_dir());

    let mut project: HashMap<PathBuf, Annotations> = HashMap::new();

    for p in args.directory.read_dir().unwrap() {
        let p = p.unwrap().path();

        if !p.is_file() {
            continue;
        }

        if let Some(s) = p.extension() {
            if s == "yaml" || s == "yml" {
                if let Ok(raw) = std::fs::read(p) {
                    let raw = raw.as_slice();
                    let mut annotations = Annotations::new();
                    let mut loader: Loader<'_, RcRepr> = yaml_peg::parser::Loader::new(raw);
                    match crate::scavenge::parse(&mut loader, &mut annotations) {
                        Ok(_) => {}
                        Err(e) => println!("Failed to parse {}", p.display()),
                    }
                } else {
                    println!("failed to read {}", p.display())
                }
            }
        } else {
            continue;
        }
    }
}
