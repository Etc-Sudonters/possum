use std::cmp::{Eq, PartialEq};
use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;
use std::string::ToString;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct ActivityType(&'static str);
type ActivityTypes = HashSet<ActivityType>;

#[derive(Debug)]
pub enum Schedule {
    Cron(String),
}

#[derive(Debug)]
pub struct PullRequest {
    branches: Vec<String>,
    ignored_branches: Vec<String>,
    paths: Vec<String>,
    ignored_paths: Vec<String>,
    types: ActivityTypes,
}

#[derive(Debug)]
pub struct Push {
    branches: Vec<String>,
    ignored_branches: Vec<String>,
    paths: Vec<String>,
    ignored_paths: Vec<String>,
    tags: Vec<String>,
    ignored_tags: Vec<String>,
}
#[derive(Debug)]
pub struct WorkflowCall {}
#[derive(Debug)]
pub struct WorkflowDispatch {}
#[derive(Debug)]
pub struct WorkflowRun {}

trait TryDefault: Sized {
    type Error;

    fn try_default() -> Result<Self, Self::Error>;
}

#[derive(Debug)]
pub enum Event {
    BranchProtectionRule(ActivityTypes),
    CheckRun(ActivityTypes),
    CheckSuite(ActivityTypes),
    Create,
    Delete,
    Deployment,
    DeploymentStatus,
    Discussion(ActivityTypes),
    DiscussionComment(ActivityTypes),
    Fork,
    Gollum,
    IssueComment(ActivityTypes),
    Issues(ActivityTypes),
    Label(ActivityTypes),
    MergeGroup(ActivityTypes),
    Milestone(ActivityTypes),
    PageBuild,
    Project(ActivityTypes),
    ProjectCard(ActivityTypes),
    ProjectColumn(ActivityTypes),
    Public,
    PullRequest(PullRequest),
    PullRequestComment,
    PullRequestReview(ActivityTypes),
    PullRequestReviewComment(ActivityTypes),
    PullRequestTarget(PullRequest),
    Push(Push),
    RegistryPackage(ActivityTypes),
    Release(ActivityTypes),
    RepositoryDispatch(ActivityTypes),
    Schedule,
    Status,
    Watch(ActivityTypes),
    WorkflowCall(WorkflowCall),
    WorkflowDispatch(WorkflowDispatch),
    WorkflowRun(WorkflowRun),
    Unknown(String),
}

impl Event {
    pub fn accept_activity(&mut self, activity: ActivityType) -> Result<(), WorkflowEventError> {
        if self.accepted_activities().contains(&activity) {
            return Ok(());
        }
        return Err(WorkflowEventError);
    }

    fn accepted_activities(&self) -> HashSet<ActivityType> {
        match self {
            _ => HashSet::new(),
        }
    }
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
