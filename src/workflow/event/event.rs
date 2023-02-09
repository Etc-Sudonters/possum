use std::convert::Into;
use std::default::Default;
use std::str::FromStr;
use std::string::ToString;

use super::bodies::*;
use super::{ActivityType, ActivityTypes, Error, EventName};

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
    PullRequestReview(ActivityTypes),
    PullRequestReviewComment(ActivityTypes),
    PullRequestTarget(PullRequest),
    Push(Push),
    RegistryPackage(ActivityTypes),
    Release(ActivityTypes),
    RepositoryDispatch(ActivityTypes),
    Schedule(Vec<Schedule>),
    Status,
    Watch(ActivityTypes),
    WorkflowCall(WorkflowCall),
    WorkflowDispatch(WorkflowDispatch),
    WorkflowRun(WorkflowRun),
    Unknown(String),
}

impl ToString for Event {
    fn to_string(&self) -> String {
        self.to_event_name().to_string()
    }
}

impl Event {
    pub(crate) fn default_activity_types(s: &str) -> ActivityTypes {
        match s.to_lowercase().as_str() {
            s @ "branch_protection_rule" => vec!["created", "edited", "deleted"],
            s @ "check_run" => vec!["created", "requested", "completed", "requested_action"],
            s @ "check_suite" => vec!["completed"],
            s @ "discussion" => vec![
                "created",
                "edited",
                "deleted",
                "transferred",
                "pinned",
                "unpinned",
                "labeled",
                "unlabeled",
                "locked",
                "unlocked",
                "category_changed",
                "answered",
                "unanswered",
            ],
            s @ "discussion_comment" => vec!["created", "edited", "deleted"],
            s @ "issue_comment" => vec!["created", "edited", "deleted"],
            s @ "issues" => vec![
                "opened",
                "edited",
                "deleted",
                "transferred",
                "pinned",
                "unpinned",
                "closed",
                "reopened",
                "assigned",
                "unassigned",
                "labeled",
                "unlabeled",
                "locked",
                "unlocked",
                "milestoned",
                "demilestoned",
            ],
            s @ "label" => vec!["created", "edited", "deleted"],
            s @ "merge_group" => vec!["checks_requested"],
            s @ "milestone" => vec!["created", "closed", "opened", "edited", "deleted"],
            s @ "project" => vec!["created", "closed", "reopened", "edited", "deleted"],
            s @ "project_card" => vec!["created", "moved", "converted", "edited", "deleted"],
            s @ "project_column" => vec!["created", "updated", "moved", "deleted"],
            s @ "pull_request" | s @ "pull_request_target" => vec![
                "assigned",
                "unassigned",
                "labeled",
                "unlabeled",
                "opened",
                "edited",
                "closed",
                "reopened",
                "synchronize",
                "converted_to_draft",
                "ready_for_review",
                "locked",
                "unlocked",
                "review_requested",
                "review_request_removed",
                "auto_merge_enabled",
                "auto_merge_disabled",
            ],
            s @ "pull_request_review" => vec!["submitted", "edited", "dismissed"],
            s @ "pull_request_review_comment" => vec!["created", "edited", "deleted"],
            s @ "registry_package" => vec!["published", "updated"],
            s @ "release" => vec![
                "published",
                "unpublished",
                "created",
                "edited",
                "deleted",
                "prereleased",
                "released",
            ],
            s @ "watch" => vec!["started"],
            s @ "workflow_run" => vec!["completed", "requested", "in_progress"],
            _ => vec![],
        }
        .iter()
        .map(|s| ActivityType::new(s.to_string()))
        .collect()
    }

    pub fn to_event_name(&self) -> EventName {
        match self {
            Event::BranchProtectionRule(_) => "branch_protection_rule".into(),
            Event::CheckRun(_) => "check_run".into(),
            Event::CheckSuite(_) => "check_suite".into(),
            Event::Create => "create".into(),
            Event::Delete => "delete".into(),
            Event::Deployment => "deployment".into(),
            Event::DeploymentStatus => "deployment_status".into(),
            Event::Discussion(_) => "discussion".into(),
            Event::DiscussionComment(_) => "discussion_comment".into(),
            Event::Fork => "fork".into(),
            Event::Gollum => "gollum".into(),
            Event::IssueComment(_) => "issue_comment".into(),
            Event::Issues(_) => "issues".into(),
            Event::Label(_) => "label".into(),
            Event::MergeGroup(_) => "merge_group".into(),
            Event::Milestone(_) => "milestone".into(),
            Event::PageBuild => "page_build".into(),
            Event::Project(_) => "project".into(),
            Event::ProjectCard(_) => "project_card".into(),
            Event::ProjectColumn(_) => "project_column".into(),
            Event::Public => "public".into(),
            Event::PullRequest(_) => "pull_request".into(),
            Event::PullRequestReview(_) => "pull_request_review".into(),
            Event::PullRequestReviewComment(_) => "pull_request_review_comment".into(),
            Event::PullRequestTarget(_) => "pull_request_target".into(),
            Event::Push(_) => "push".into(),
            Event::RegistryPackage(_) => "registry_package".into(),
            Event::Release(_) => "release".into(),
            Event::RepositoryDispatch(_) => "repository_dispatch".into(),
            Event::Schedule(_) => "schedule".into(),
            Event::Status => "status".into(),
            Event::Watch(_) => "watch".into(),
            Event::WorkflowCall(_) => "workflow_call".into(),
            Event::WorkflowDispatch(_) => "workflow_dispatch".into(),
            Event::WorkflowRun(_) => "workflow_run".into(),
            Event::Unknown(s) => s.into(),
        }
    }
}

impl FromStr for Event {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower_evt = s.to_lowercase();
        let default_activity_types = Event::default_activity_types(&lower_evt);
        match lower_evt.as_str() {
            s @ "branch_protection_rule" => Ok(Event::BranchProtectionRule(default_activity_types)),
            s @ "check_run" => Ok(Event::CheckRun(default_activity_types)),
            s @ "check_suite" => Ok(Event::CheckSuite(default_activity_types)),
            s @ "create" => Ok(Event::Create),
            s @ "delete" => Ok(Event::Delete),
            s @ "deployment" => Ok(Event::Deployment),
            s @ "deployment_status" => Ok(Event::DeploymentStatus),
            s @ "discussion" => Ok(Event::Discussion(default_activity_types)),
            s @ "discussion_comment" => Ok(Event::DiscussionComment(default_activity_types)),
            s @ "fork" => Ok(Event::Fork),
            s @ "gollum" => Ok(Event::Gollum),
            s @ "issue_comment" => Ok(Event::IssueComment(default_activity_types)),
            s @ "issues" => Ok(Event::Issues(default_activity_types)),
            s @ "label" => Ok(Event::Label(default_activity_types)),
            s @ "merge_group" => Ok(Event::MergeGroup(default_activity_types)),
            s @ "milestone" => Ok(Event::Milestone(default_activity_types)),
            s @ "page_build" => Ok(Event::PageBuild),
            s @ "project" => Ok(Event::Project(default_activity_types)),
            s @ "project_card" => Ok(Event::ProjectCard(default_activity_types)),
            s @ "project_column" => Ok(Event::ProjectColumn(default_activity_types)),
            s @ "public" => Ok(Event::Public),
            s @ "pull_request" => Ok(Event::PullRequest(PullRequest::default())),
            s @ "pull_request_review" => Ok(Event::PullRequestReview(default_activity_types)),
            s @ "pull_request_review_comment" => {
                Ok(Event::PullRequestReviewComment(default_activity_types))
            }
            s @ "pull_request_target" => Ok(Event::PullRequestTarget(PullRequest::default())),
            s @ "push" => Ok(Event::Push(Push::default())),
            s @ "registry_package" => Ok(Event::RegistryPackage(default_activity_types)),
            s @ "release" => Ok(Event::Release(default_activity_types)),
            s @ "repository_dispatch" => Ok(Event::RepositoryDispatch(default_activity_types)),
            s @ "schedule" => Err(Error::CannotUseStringDeclaration(s.into())),
            s @ "status" => Ok(Event::Status),
            s @ "watch" => Ok(Event::Watch(default_activity_types)),
            s @ "workflow_call" => Ok(Event::WorkflowCall(WorkflowCall::default())),
            s @ "workflow_dispatch" => Ok(Event::WorkflowDispatch(WorkflowDispatch::default())),
            s @ "workflow_run" => Err(Error::CannotUseStringDeclaration(s.into())),
            s @ _ => Err(Error::UnknownEvent(s.into())),
        }
    }
}
