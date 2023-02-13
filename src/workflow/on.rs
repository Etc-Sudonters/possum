use strum::Display;

use crate::scavenge::ast::*;

#[derive(Debug)]
pub enum Trigger {
    Base(PossumNode<EventKind>),
    Array(PossumNode<Seq<EventKind>>),
    Events(PossumNode<Seq<Event>>),
}

possum_node!(
    #[derive(Debug)]
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

possum_node!(
    #[derive(Debug)]
    struct WorkflowInput {
        name: String,
        description: String,
        default: WorkflowInputDefault,
        required: bool,
        input_type: WorkflowInputType,
        choices: Seq<String>,
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

possum_node!(
    #[derive(Debug)]
    struct WorkflowOutput {
        name: String,
        description: String,
        value: String,
    }
);

possum_node!(
    #[derive(Debug)]
    struct InheritedSecret {
        name: String,
        description: String,
        required: bool,
    }
);
