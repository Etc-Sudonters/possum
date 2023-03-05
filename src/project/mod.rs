pub mod builder;
use crate::scavenge::ParseFailure;
use std::path::PathBuf;
use std::fmt::Display;

#[derive(Debug)]
pub struct InitFailure {
    reason: InitFailureReason,
    path: PathBuf
}

impl InitFailure {
    fn new(p: impl Into<PathBuf>, reason: InitFailureReason) -> InitFailure {
        InitFailure { reason, path: p.into().clone() }
    }

    pub fn no_workflows(p: impl Into<PathBuf>) -> InitFailure {
        Self::new(p, InitFailureReason::NoWorkflows)
    }

    pub fn dir_not_found(p: impl Into<PathBuf>) -> InitFailure {
        Self::new(p, InitFailureReason::DirectoryNotFound)
    }

    pub fn not_dir(p: impl Into<PathBuf>) -> InitFailure {
        Self::new(p, InitFailureReason::NotADirectory)
    }
}

impl Display for InitFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.path.display(), self.reason)
    }
}

impl std::error::Error for InitFailure{}

#[derive(Debug)]
 enum InitFailureReason {
    NoWorkflows,
    DirectoryNotFound,
    NotADirectory,
}


impl Display for InitFailureReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use InitFailureReason::*;
        let msg = match self {
            NoWorkflows => "no workflows found in directory",
            DirectoryNotFound => "directory not found",
            NotADirectory => "not a directory"
        };

        write!(f, "{}", msg)
    }
}

#[derive(Debug)]
pub struct Project {
    _root: ProjectRoot,
    // this is set if we needed to rut around to find the workflows directory
    entries: Vec<ProjectEntry>,
}

impl Project {
    pub fn new(root: ProjectRoot) -> Self {
        Project {
            _root: root.into(),
            entries: Vec::with_capacity(8),
        }
    }

    pub fn push(&mut self, p: ProjectEntry) {
        self.entries.push(p)
    }

    pub fn entries(&self) -> std::slice::Iter<ProjectEntry> {
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

impl Into<PathBuf> for &ProjectRoot {
    fn into(self) -> PathBuf {
        self.path().clone()
    }
}

impl Into<PathBuf> for ProjectRoot {
    fn into(self) -> PathBuf {
        Into::<PathBuf>::into(&self)
    }
}

impl Display for ProjectRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ProjectRoot::WorkingDirectory => write!(f, "{}", PathBuf::from(".").display()),
            ProjectRoot::Explicit(dir) => write!(f, "{}", dir.display()),
        }
    }
}
