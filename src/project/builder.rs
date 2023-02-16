use crate::document::Annotations;
use crate::document::Document;
use crate::scavenge::parse_single_document as possum_parse;
use crate::scavenge::ParseFailure;
use crate::workflow::WorkflowParser;
use std::ffi::OsStr;
use yaml_peg::parser::Loader;
use yaml_peg::repr::RcRepr;

use super::{InitFailures, Project, ProjectEntry};
use std::path::PathBuf;

pub fn build(mut root: PathBuf) -> Result<Project, InitFailures> {
    if !root.exists() {
        return Err(InitFailures::DirectoryNotFound(root));
    }

    if !root.is_dir() {
        return Err(InitFailures::NotADirectory(root));
    }

    let mut workflow_dir = PathBuf::from(".github");
    workflow_dir.push("workflows");

    let mut orig_root: Option<PathBuf> = None;

    if !root.ends_with(&workflow_dir) {
        let mut possible = root.clone();
        possible.push(workflow_dir);

        if possible.exists() {
            orig_root = Some(root);
            root = possible;
        } else {
            return Err(InitFailures::NoWorkflowDirectoryFound(root));
        }
    }

    let workflows = get_all_workflows(&root);
    let mut project = Project::new(root);
    project.orig_root = orig_root;

    for p in workflows {
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

fn get_all_workflows(root: &PathBuf) -> Vec<PathBuf> {
    std::fs::read_dir(root)
        .unwrap()
        .map(|d| d.unwrap().path())
        .filter(|p| {
            p.extension()
                .and_then(OsStr::to_str)
                .and_then(|ext| Some(ext.ends_with("yaml") || ext.ends_with("yml")))
                .unwrap()
        })
        .collect()
}
