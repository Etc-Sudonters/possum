use strum::Display;

use super::core::*;
pub enum Trigger {
    Base(Node<EventKind>),
    Array(Node<Seq<EventKind>>),
    Events(Node<Seq<Event>>),
}

node!(
    struct Event {
        kind: EventKind,
        branches: Seq<String>,
        branches_ignore: Seq<String>,
        paths: Seq<String>,
        paths_ignore: Seq<String>,
        tags: Seq<String>,
        tags_ignore: Seq<String>,
        inputs: Seq<WorkflowInput>,
        outputs: Seq<WorkflowOutput>,
        secrets: Seq<InheritedSecret>,
    }
);

#[derive(Debug, Eq, PartialEq, Display)]
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

node!(
    struct WorkflowInput {
        name: String,
        description: String,
        default: WorkflowInputDefault,
        required: bool,
        input_type: WorkflowInputType,
        choices: Seq<String>,
    }
);

pub enum WorkflowInputDefault {
    Str(Node<String>),
    Number(Node<i64>),
    Bool(Node<bool>),
}

pub enum WorkflowInputType {
    Str,
    Number,
    Bool,
    Choice,
}

node!(
    struct WorkflowOutput {
        name: String,
        description: String,
        value: String,
    }
);

node!(
    struct InheritedSecret {
        name: String,
        description: String,
        required: bool,
    }
);
