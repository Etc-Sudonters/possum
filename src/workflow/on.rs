use std::{fmt::Display, str::FromStr};

use crate::scavenge::ast::*;

#[derive(Debug)]
pub struct Trigger(PossumMap<EventKind, Event>);

impl Trigger {
    pub fn empty() -> Trigger {
        Trigger(PossumMap::empty())
    }

    pub fn add_event(&mut self, kind: PossumNode<EventKind>, event: PossumNode<Event>) {
        self.0.insert(kind, event);
    }

    pub fn add_event_name(&mut self, kind: PossumNode<EventKind>) {
        let loc = kind.loc();
        self.add_event(kind, PossumNodeKind::Empty.at(loc));
    }
}

impl Default for Trigger {
    fn default() -> Self {
        Self::empty()
    }
}

impl Into<Trigger> for PossumNode<EventKind> {
    fn into(self) -> Trigger {
        let mut trig = Trigger::empty();
        let loc = self.loc();
        trig.add_event_name(self);
        trig
    }
}

impl Into<Trigger> for PossumSeq<EventKind> {
    fn into(self) -> Trigger {
        self.into_iter().collect()
    }
}

impl FromIterator<PossumNode<EventKind>> for Trigger {
    fn from_iter<T: IntoIterator<Item = PossumNode<EventKind>>>(iter: T) -> Self {
        let mut trig = Trigger::empty();

        for ek in iter.into_iter() {
            trig.add_event_name(ek);
        }

        trig
    }
}

impl FromIterator<(PossumNode<EventKind>, PossumNode<Event>)> for Trigger {
    fn from_iter<T: IntoIterator<Item = (PossumNode<EventKind>, PossumNode<Event>)>>(
        iter: T,
    ) -> Self {
        let mut trig = Trigger::default();

        for (kind, event) in iter.into_iter() {
            trig.add_event(kind, event)
        }

        trig
    }
}

impl Into<Trigger> for PossumMap<EventKind, Event> {
    fn into(self) -> Trigger {
        Trigger(self)
    }
}

#[derive(Debug, Default)]
pub struct Globbed(String);

impl Globbed {
    pub fn new<S>(s: S) -> Globbed
    where
        S: Into<String>,
    {
        Globbed(s.into())
    }
}

possum_node_type!(
    #[derive(Debug, Default)]
    struct Event {
        branches: PossumSeq<Globbed>,
        branches_ignore: PossumSeq<Globbed>,
        paths: PossumSeq<Globbed>,
        paths_ignore: PossumSeq<Globbed>,
        tags: PossumSeq<Globbed>,
        tags_ignore: PossumSeq<Globbed>,
        inputs: PossumMap<String, WorkflowInput>,
        outputs: PossumMap<String, WorkflowOutput>,
        secrets: PossumMap<String, InheritedSecret>,
    }
);

impl Event {
    pub fn new() -> Event {
        Default::default()
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
    pub fn fromstr(raw: &str) -> Result<EventKind, BadEvent> {
        EventKind::from_str(raw).map_err(|_| BadEvent::Unknown(raw.to_owned()))
    }
}

possum_node_type!(
    #[derive(Debug, Default)]
    struct WorkflowInput {
        description: String,
        default: WorkflowInputDefault,
        required: bool,
        input_type: WorkflowInputType,
        choices: PossumSeq<String>,
    }
);

#[derive(Debug)]
pub enum WorkflowInputDefault {
    Str(String),
    Number(String),
    Bool(bool),
}

#[derive(Debug, strum::EnumString)]
pub enum WorkflowInputType {
    Str,
    Number,
    Bool,
    Choice,
}

possum_node_type!(
    #[derive(Debug, Default)]
    struct WorkflowOutput {
        description: String,
        value: String,
    }
);

possum_node_type!(
    #[derive(Debug, Default)]
    struct InheritedSecret {
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
