use self::core::*;

mod core {
    pub enum Node<T> {
        Absent,
        Invalid { location: usize, msg: String },
        N { location: usize, value: T },
    }

    pub struct Map<K, V> {
        keys: Seq<K>,
        values: Seq<V>,
    }

    pub struct Seq<T> {
        entries: Vec<Node<T>>,
    }
}

pub struct Workflow {
    name: Node<String>,
    run_name: Node<String>,
    on: Node<on::Trigger>,
    jobs: Seq<job::Job>,
}

mod job {
    use super::core::*;
    use super::Concurrency;
    use super::Permission;
    use super::Step;

    pub struct Job {
        id: Node<String>,
        name: Node<String>,
        permissions: Node<Permission>,
        needs: Node<Seq<String>>,
        cond: Node<String>,
        runs_on: Node<String>,
        environment: Node<Environment>,
        concurrency: Node<Concurrency>,
        outputs: Node<Map<String, Output>>,
        env: Node<Map<String, String>>,
        steps: Node<Seq<Step>>,
        timeout_minutes: Node<u64>,
        strategy: Node<Strategy>,
        continue_on_error: Node<bool>,
        container: Node<Container>,
        services: Node<Map<String, Service>>,
        uses: Node<String>,
        with: Node<Map<String, String>>,
    }

    pub enum Environment {
        Bare(Node<String>),
        Env {
            name: Node<String>,
            url: Node<String>,
        },
    }

    pub struct Output {}
    pub struct Strategy;
    pub struct Container;
    pub struct Service;
}

use permission::Permission;
mod permission {
    use super::{Map, Node};
    pub enum Permission {
        GlobalGrant(Node<Grant>),
        GlobalRevoke,
        IndividualGrants(Node<Map<String, Grant>>),
    }

    pub enum Grant {
        Read,
        Write,
    }
}

mod on {
    use super::core::*;
    pub enum Trigger {
        Base(Node<EventKind>),
        Array(Node<Seq<EventKind>>),
        Events(Node<Seq<Event>>),
    }

    pub struct Event {
        kind: Node<EventKind>,
        branches: Node<Seq<String>>,
        branches_ignore: Node<Seq<String>>,
        paths: Node<Seq<String>>,
        paths_ignore: Node<Seq<String>>,
        tags: Node<Seq<String>>,
        tags_ignore: Node<Seq<String>>,
        inputs: Node<Seq<WorkflowInput>>,
        outputs: Node<Seq<WorkflowOutput>>,
        secrets: Node<Seq<InheritedSecret>>,
    }

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

    pub struct WorkflowInput {
        name: Node<String>,
        description: Node<String>,
        default: Node<WorkflowInputDefault>,
        required: Node<bool>,
        input_type: Node<WorkflowInputType>,
        choices: Node<Seq<String>>,
    }

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

    pub struct WorkflowOutput {
        name: Node<String>,
        description: Node<String>,
        value: Node<String>,
    }

    pub struct InheritedSecret {
        name: Node<String>,
        description: Node<String>,
        required: Node<bool>,
    }
}

use step::Step;
mod step {
    use super::{Map, Node};
    pub struct Step {
        id: Node<String>,
        cond: Node<String>,
        name: Node<String>,
        uses: Node<String>,
        run: Node<String>,
        shell: Node<String>,
        with: Node<StepWith>,
    }

    pub enum StepWith {
        Container {
            args: Node<String>,
            entrypoint: Node<String>,
        },
        Args(Map<String, String>),
    }
}
use trigger::Trigger;
mod trigger {
    use super::{Node, Seq};
    pub enum Trigger {
        Base(Node<EventKind>),
        Array(Node<Seq<EventKind>>),
        Events(Node<Seq<Event>>),
    }

    pub struct Event {
        kind: Node<EventKind>,
        branches: Node<Seq<String>>,
        branches_ignore: Node<Seq<String>>,
        paths: Node<Seq<String>>,
        paths_ignore: Node<Seq<String>>,
        tags: Node<Seq<String>>,
        tags_ignore: Node<Seq<String>>,
        inputs: Node<Seq<Input>>,
        outputs: Node<Seq<Output>>,
        secrets: Node<Seq<InheritedSecret>>,
    }

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

    pub struct Input {
        name: Node<String>,
        description: Node<String>,
        default: Node<DefaultValue>,
        required: Node<bool>,
        input_type: Node<InputType>,
        choices: Node<Seq<String>>,
    }

    pub enum DefaultValue {
        Str(Node<String>),
        Number(Node<i64>),
        Bool(Node<bool>),
    }

    pub enum InputType {
        Str,
        Number,
        Bool,
        Choice,
    }

    pub struct Output {
        name: Node<String>,
        description: Node<String>,
        value: Node<String>,
    }

    pub struct InheritedSecret {
        name: Node<String>,
        description: Node<String>,
        required: Node<bool>,
    }
}
