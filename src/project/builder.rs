use super::{Project, ProjectEntry, InitFailure};
use crate::document::{Annotations, Document};
use crate::scavenge::{parse_single_document as possum_parse, ParseFailure};
use crate::workflow::WorkflowParser;
use std::ffi::OsStr;
use std::path::PathBuf;
use yaml_peg::parser::Loader;
use yaml_peg::repr::RcRepr;
use super::ProjectRoot;

pub fn build(root: ProjectRoot) -> Result<Project, InitFailure> {
    if !root.exists() {
        Err(InitFailure::dir_not_found(&root))?;
    }

    if !root.is_dir() {
        Err(InitFailure::not_dir(&root))?;
    }

    let workflows = get_all_workflows(&root);

    if workflows.is_empty() {
        Err(InitFailure::no_workflows(&root))?;
    }

    let mut project = Project::new(root);

    for p in workflows {
        match std::fs::read(&p) {
            Ok(raw) => {
                let mut annotations = Annotations::new();
                let loader: Loader<'_, RcRepr> = yaml_peg::parser::Loader::new(&raw);
                let mut parser = WorkflowParser::new(&mut annotations);
                match possum_parse(loader, &mut parser) {
                    Ok(workflow) => {
                        project.push(ProjectEntry::Workflow {
                            source: p,
                            annotations,
                            workflow,
                            document: Document::new(raw),
                        });
                    }
                    Err(pf) => {
                        project.push(ProjectEntry::ParseFailure(p, pf));
                    }
                }
            }
            Err(e) => {
                project.push(ProjectEntry::ParseFailure(p, ParseFailure::CouldntOpen(e)));
            }
        }
    }

    Ok(project)
}

fn get_all_workflows(root: impl Into<PathBuf>) -> Vec<PathBuf> {
    std::fs::read_dir(root.into())
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
