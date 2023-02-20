pub mod builder;
use crate::scavenge::ParseFailure;
use std::path::PathBuf;

#[derive(Debug)]
pub enum InitFailures {
    NoWorkflows(PathBuf),
    DirectoryNotFound(PathBuf),
    NoWorkflowDirectoryFound(PathBuf),
    NotADirectory(PathBuf),
}

#[derive(Debug)]
pub struct Project {
    _root: PathBuf,
    // this is set if we needed to rut around to find the workflows directory
    orig_root: Option<PathBuf>,
    entries: Vec<ProjectEntry>,
}

impl Project {
    pub fn new<P>(root: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Project {
            _root: root.into(),
            entries: Vec::with_capacity(8),
            orig_root: None,
        }
    }

    pub fn push(&mut self, p: ProjectEntry) {
        self.entries.push(p)
    }

    pub fn entries<'a>(&'a self) -> std::slice::Iter<ProjectEntry> {
        self.entries.iter()
    }
}

#[derive(Debug)]
pub enum ProjectEntry {
    Workflow {
        source: PathBuf,
        document: super::document::Document,
        annotations: super::document::Annotations,
        workflow: crate::scavenge::ast::PossumNode<super::workflow::Workflow>,
    },
    ParseFailure(PathBuf, ParseFailure),
}
