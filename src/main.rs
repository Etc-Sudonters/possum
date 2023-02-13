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
mod document;
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
