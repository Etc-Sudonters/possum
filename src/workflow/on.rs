use std::{fmt::Display, str::FromStr};

use crate::scavenge::ast::*;

#[derive(Debug, Default)]
pub struct Trigger(PossumSeq<Event>);

impl Trigger {
    pub fn new() -> Trigger {
        Trigger(PossumSeq::new())
    }

    pub fn push(&mut self, e: PossumNode<Event>) {
        self.0.push(e)
    }
}

impl Into<Trigger> for PossumSeq<Event> {
    fn into(self) -> Trigger {
        Trigger(self)
    }
}

impl Into<Trigger> for PossumNode<Event> {
    fn into(self) -> Trigger {
        let trig = Trigger::new();
        trig.push(self);
        trig
    }
}

impl Into<Event> for PossumNode<EventKind> {
    fn into(self) -> Event {
        Event::new(self)
    }
}

impl Into<Trigger> for PossumNode<EventKind> {
    fn into(self) -> Trigger {
        use PossumNodeKind::Value;
        let loc = self.loc();
        Value(Event::new(self)).at(loc).into()
    }
}

possum_node_type!(
    #[derive(Debug, Default)]
    struct Event {
        kind: EventKind,
        branches: PossumSeq<String>,
        branches_ignore: PossumSeq<String>,
        paths: PossumSeq<String>,
        paths_ignore: PossumSeq<String>,
        tags: PossumSeq<String>,
        tags_ignore: PossumSeq<String>,
        inputs: PossumSeq<WorkflowInput>,
        outputs: PossumSeq<WorkflowOutput>,
        secrets: PossumSeq<InheritedSecret>,
    }
);

impl Event {
    pub fn new(kind: PossumNode<EventKind>) -> Event {
        Event {
            kind: Some(kind),
            ..Default::default()
        }
    }
}

#[derive(Debug, Eq, PartialEq, strum::Display, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum EventKind {
    BranchProtectionRule,
    CheckRun,
    CheckSuite,
    Create,
    Delete,
    Deployment,
    DeploymentStatus,
    Discussion,
    DiscussionComment,
    Fork,
    Gollum,
    IssueComment,
    Issues,
    Label,
    MergeGroup,
    Milestone,
    PageBuild,
    Project,
    ProjectCard,
    ProjectColumn,
    Public,
    PullRequest,
    PullRequestReview,
    PullRequestReviewComment,
    PullRequestTarget,
    Push,
    RegistryPackage,
    Release,
    RepositoryDispatch,
    Schedule,
    Status,
    Watch,
    WorkflowCall,
    WorkflowDispatch,
    WorkflowRun,
}

impl EventKind {
    pub fn what_to_name(raw: &str) -> Result<EventKind, BadEvent> {
        EventKind::from_str(raw).map_err(|_| BadEvent::Unknown(raw.to_owned()))
    }
}

possum_node_type!(
    #[derive(Debug)]
    struct WorkflowInput {
        name: String,
        description: String,
        default: WorkflowInputDefault,
        required: bool,
        input_type: WorkflowInputType,
        choices: PossumSeq<String>,
    }
);

#[derive(Debug)]
pub enum WorkflowInputDefault {
    Str(PossumNode<String>),
    Number(PossumNode<i64>),
    Bool(PossumNode<bool>),
}

#[derive(Debug)]
pub enum WorkflowInputType {
    Str,
    Number,
    Bool,
    Choice,
}

possum_node_type!(
    #[derive(Debug)]
    struct WorkflowOutput {
        name: String,
        description: String,
        value: String,
    }
);

possum_node_type!(
    #[derive(Debug)]
    struct InheritedSecret {
        name: String,
        description: String,
        required: bool,
    }
);

pub enum BadEvent {
    Unknown(String),
}

impl Display for BadEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadEvent::Unknown(s) => write!(f, "unknown event {s}"),
        }
    }
}
