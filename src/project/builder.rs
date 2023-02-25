use crate::document::Annotations;
use crate::document::Document;
use crate::scavenge::parse_single_document as possum_parse;
use crate::scavenge::ParseFailure;
use crate::workflow::WorkflowParser;
use std::ffi::OsStr;
use std::fmt::Display;
use yaml_peg::parser::Loader;
use yaml_peg::repr::RcRepr;

use super::{InitFailures, Project, ProjectEntry};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum ProjectRoot {
    Explicit(PathBuf),
    WorkingDirectory,
}

impl ProjectRoot {
    fn path(&self) -> PathBuf {
        match self {
            ProjectRoot::Explicit(d) => d.to_owned(),
            ProjectRoot::WorkingDirectory => std::env::current_dir().unwrap(),
        }
        .join(".github")
        .join("workflows")
    }

    fn exists(&self) -> bool {
        self.path().exists()
    }

    fn is_dir(&self) -> bool {
        self.path().is_dir()
    }
}

impl Into<PathBuf> for ProjectRoot {
    fn into(self) -> PathBuf {
        self.path().clone()
    }
}

impl Display for ProjectRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ProjectRoot::WorkingDirectory => write!(f, "."),
            ProjectRoot::Explicit(dir) => write!(f, "{}", dir.display()),
        }
    }
}

pub fn build(root: ProjectRoot) -> Result<Project, InitFailures> {
    if !root.exists() {
        return Err(InitFailures::DirectoryNotFound(root.path()));
    }

    if !root.is_dir() {
        return Err(InitFailures::NotADirectory(root.path()));
    }

    let workflows = get_all_workflows(root.path());

    if workflows.is_empty() {
        Err(InitFailures::NoWorkflows(root.path()))?;
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
            Err(_) => {
                project.push(ProjectEntry::ParseFailure(p, ParseFailure::CouldntOpen));
            }
        }
    }

    Ok(project)
}

fn get_all_workflows(root: PathBuf) -> Vec<PathBuf> {
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
