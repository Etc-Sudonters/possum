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
    root: PathBuf,
    // this is set if we needed to rut around to find the workflows directory
    orig_root: Option<PathBuf>,
    entries: Vec<ProjectEntry>,
}

impl Project {
    pub fn from<P, E>(root: P, entries: E) -> Self
    where
        P: Into<PathBuf>,
        E: Into<Vec<ProjectEntry>>,
    {
        Project {
            root: root.into(),
            entries: entries.into(),
            orig_root: None,
        }
    }

    pub fn new<P>(root: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Project {
            root: root.into(),
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

    pub fn root(&self) -> PathBuf {
        self.root.clone()
    }
}

#[derive(Debug)]
pub enum ProjectEntry {
    Workflow {
        source: PathBuf,
        document: super::document::Document,
        annotations: super::document::Annotations,
        workflow: super::scavenge::workflow::Workflow,
    },
    ParseFailure(PathBuf, ParseFailure),
}
