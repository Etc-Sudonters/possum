use std::str::FromStr;
use std::string::ToString;

#[derive(Debug)]
pub enum Event {
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
    PullRequestComment,
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
    Unknown(String),
}

pub struct WorkflowEventError;

impl FromStr for Event {
    type Err = WorkflowEventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Event::*;
        let lower_evt = s.to_lowercase();
        match lower_evt.as_str() {
            "branch_protection_rule" => Ok(BranchProtectionRule),
            "check_run" => Ok(CheckRun),
            "check_suite" => Ok(CheckSuite),
            "create" => Ok(Create),
            "delete" => Ok(Delete),
            "deployment" => Ok(Deployment),
            "deployment_status" => Ok(DeploymentStatus),
            "discussion" => Ok(Discussion),
            "discussion_comment" => Ok(DiscussionComment),
            "fork" => Ok(Fork),
            "gollum" => Ok(Gollum),
            "issue_comment" => Ok(IssueComment),
            "issues" => Ok(Issues),
            "label" => Ok(Label),
            "merge_group" => Ok(MergeGroup),
            "milestone" => Ok(Milestone),
            "page_build" => Ok(PageBuild),
            "project" => Ok(Project),
            "project_card" => Ok(ProjectCard),
            "project_column" => Ok(ProjectColumn),
            "public" => Ok(Public),
            "pull_request" => Ok(PullRequest),
            "pull_request_comment" => Ok(PullRequestComment),
            "pull_request_review" => Ok(PullRequestReview),
            "pull_request_review_comment" => Ok(PullRequestReviewComment),
            "pull_request_target" => Ok(PullRequestTarget),
            "push" => Ok(Push),
            "registry_package" => Ok(RegistryPackage),
            "release" => Ok(Release),
            "repository_dispatch" => Ok(RepositoryDispatch),
            "schedule" => Ok(Schedule),
            "status" => Ok(Status),
            "watch" => Ok(Watch),
            "workflow_call" => Ok(WorkflowCall),
            "workflow_dispatch" => Ok(WorkflowDispatch),
            "workflow_run" => Ok(WorkflowRun),
            _ => Ok(Unknown(lower_evt.to_owned())),
        }
    }
}

impl ToString for Event {
    fn to_string(&self) -> String {
        use Event::*;
        match *self {
            BranchProtectionRule => "branch_protection_rule",
            CheckRun => "check_run",
            CheckSuite => "check_suite",
            Create => "create",
            Delete => "delete",
            Deployment => "deployment",
            DeploymentStatus => "deployment_status",
            Discussion => "discussion",
            DiscussionComment => "discussion_comment",
            Fork => "fork",
            Gollum => "gollum",
            IssueComment => "issue_comment",
            Issues => "issues",
            Label => "label",
            MergeGroup => "merge_group",
            Milestone => "milestone",
            PageBuild => "page_build",
            Project => "project",
            ProjectCard => "project_card",
            ProjectColumn => "project_column",
            Public => "public",
            PullRequest => "pull_request",
            PullRequestComment => "pull_request_comment",
            PullRequestReview => "pull_request_review",
            PullRequestReviewComment => "pull_request_review_comment",
            PullRequestTarget => "pull_request_target",
            Push => "push",
            RegistryPackage => "registry_package",
            Release => "release",
            RepositoryDispatch => "repository_dispatch",
            Schedule => "schedule",
            Status => "status",
            Watch => "watch",
            WorkflowCall => "workflow_call",
            WorkflowDispatch => "workflow_dispatch",
            WorkflowRun => "workflow_run",
            Unknown(_) => "unknown",
        }
        .to_owned()
    }
}
