use crate::document::Annotations;
use crate::document::Document;
use crate::scavenge::parse as possum_parse;
use crate::scavenge::workflow::WorkflowParser;
use crate::scavenge::ParseFailure;
use yaml_peg::parser::Loader;
use yaml_peg::repr::RcRepr;

use super::{InitFailures, Project, ProjectEntry};
use std::path::PathBuf;

pub fn build(mut root: PathBuf) -> Result<Project, InitFailures> {
    if !root.exists() {
        return Err(InitFailures::DirectoryNotFound);
    }

    if !root.is_dir() {
        return Err(InitFailures::NotADirectory);
    }

    let mut workflow_dir = PathBuf::from(".github");
    workflow_dir.push("workflow");

    let mut orig_root: Option<PathBuf> = None;

    if !root.ends_with(&workflow_dir) {
        let mut possible = root.clone();
        possible.push(workflow_dir);

        if possible.exists() {
            orig_root = Some(root);
            root = possible;
        } else {
            return Err(InitFailures::NoWorkflowDirectoryFound);
        }
    }

    let workflows = get_all_workflows(root.clone());
    let mut project = Project::new(root);
    project.orig_root = orig_root;

    for p in workflows {
        let p = p.unwrap();

        match std::fs::read(&p) {
            Ok(raw) => {
                let mut annotations = Annotations::new();
                let loader: Loader<'_, RcRepr> = yaml_peg::parser::Loader::new(&raw);
                let parser = WorkflowParser::new(&mut annotations);
                match possum_parse(loader, parser) {
                    Ok(workflow) => {
                        project.push(ProjectEntry::Workflow {
                            source: p,
                            annotations,
                            workflow,
                            document: Document::new(raw),
                        });
                    }
                    Err(pf) => {
                        project.push(ProjectEntry::ParseFailure(p, ParseFailure::CouldntOpen));
                    }
                }
            }
            Err(e) => {
                project.push(ProjectEntry::ParseFailure(p, ParseFailure::CouldntOpen));
            }
        }
    }

    Ok(project)
}

fn get_all_workflows(mut root: PathBuf) -> glob::Paths {
    root.push("*.ya?ml");
    let pat = root.into_os_string().into_string().unwrap();
    println!("using pattern {}", pat);
    glob::glob_with(
        &pat,
        glob::MatchOptions {
            case_sensitive: false,
            require_literal_separator: true,
            require_literal_leading_dot: true,
        },
    )
    .unwrap()
}
