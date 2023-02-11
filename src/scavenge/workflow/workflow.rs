use crate::document::DocumentPointer;
use std::collections::HashMap;
use std::convert::AsRef;

pub type MarkedMap<'a, T> = HashMap<MString<'a>, T>;
pub type MString<'a> = Marked<&'a str>;
pub type KVP<'a> = (MString<'a>, MString<'a>);

pub struct NamedNode<'a, T> {
    name: MString<'a>,
    children: Vec<T>,
}

pub struct Marked<T> {
    value: T,
    pointer: DocumentPointer,
}

impl<T> Marked<T> {
    pub fn new(value: T, pointer: DocumentPointer) -> Marked<T> {
        Marked { value, pointer }
    }
}

impl<T> AsRef<T> for Marked<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

pub enum Workflow<'a> {
    Name(MString<'a>),
    RunName(MString<'a>),
    On(Event<'a>),
    Env(MarkedMap<'a, MString<'a>>),
    Defaults(Defaults<'a>),
    Concurrency(Concurrency<'a>),
    Jobs(Vec<JobNode<'a>>),
    Permission(Permission<'a>),
}

pub struct JobNode<'a> {
    pub(super) name: MString<'a>,
    pub(super) children: Vec<Job<'a>>,
}

pub enum Job<'a> {
    Concurrency(Concurrency<'a>),
    Container(Vec<JobContainer<'a>>),
    ContinueOnError(MString<'a>),
    Defaults(Defaults<'a>),
    Env(MarkedMap<'a, MString<'a>>),
    Environment(Environment<'a>),
    If(MString<'a>),
    Name(MString<'a>),
    Needs(Vec<MString<'a>>),
    Outputs(MarkedMap<'a, MString<'a>>),
    Permissions(Permission<'a>),
    RunsOn(Vec<MString<'a>>),
    Secrets(PassedSecret<'a>),
    Services(Vec<JobServiceNode<'a>>),
    Steps(Vec<StepNode<'a>>),
    Strategy(Strategy<'a>),
    TimeoutMinutes(MString<'a>),
    Uses(MString<'a>),
    With(MarkedMap<'a, MString<'a>>),
}

pub struct StepNode<'a> {
    children: Vec<Step<'a>>,
    idx: usize,
}

pub struct JobServiceNode<'a> {
    children: Vec<JobService<'a>>,
    idx: usize,
}

pub enum PassedSecret<'a> {
    Inherit,
    Explicit(Vec<KVP<'a>>),
}
pub enum JobService<'a> {
    Image(MString<'a>),
    Credentials(Vec<ContainerCreds<'a>>),
    Env(MarkedMap<'a, MString<'a>>),
    Ports(Vec<MString<'a>>),
    Volumes(Vec<MString<'a>>),
    Options(Vec<MString<'a>>),
}
pub enum JobContainer<'a> {
    Image(MString<'a>),
    Credentials(Vec<ContainerCreds<'a>>),
    Env(MarkedMap<'a, MString<'a>>),
    Ports(Vec<MString<'a>>), // ??
    Volumes(Vec<MString<'a>>),
    Options(Vec<MString<'a>>), //?
}
pub enum ContainerCreds<'a> {
    Username(MString<'a>),
    Password(MString<'a>),
}
pub enum Strategy<'a> {
    Expr(MString<'a>),
    Matrix(Vec<Matrix<'a>>),
    FailFast(MString<'a>),
    MaxParallel(MString<'a>),
}

pub enum Matrix<'a> {
    Entry(KVP<'a>),
    Include(Vec<Vec<KVP<'a>>>),
    Exclude(Vec<Vec<KVP<'a>>>),
}

pub enum Step<'a> {
    Id(MString<'a>),
    Name(MString<'a>),
    If(MString<'a>),
    Uses(MString<'a>),
    Run(MString<'a>),
    Shell(MString<'a>),
    Env(MarkedMap<'a, MString<'a>>),
    With(StepWith<'a>),
}

pub enum StepWith<'a> {
    Envish(MarkedMap<'a, MString<'a>>),
    Container(Vec<ContainerWith<'a>>),
}

pub enum ContainerWith<'a> {
    Args(MString<'a>),
    Entrypoint(MString<'a>),
}

pub enum Environment<'a> {
    Name(MString<'a>),
    Group(EnvironmentGroup<'a>),
}
pub enum EnvironmentGroup<'a> {
    Name(MString<'a>),
    Url(MString<'a>),
}
pub enum Concurrency<'a> {
    Bare(MString<'a>),
    Group(Vec<ConcurrencyGroup<'a>>),
}

pub enum ConcurrencyGroup<'a> {
    Name(MString<'a>),
    CancelInProgress(MString<'a>),
}

pub enum Defaults<'a> {
    Runs(Runs<'a>),
}

pub enum Runs<'a> {
    Shell(MString<'a>),
}

pub enum Permission<'a> {
    GlobalGrant(MString<'a>),
    GlobalRevoke,
    InvididualGrant(MarkedMap<'a, MString<'a>>),
}

pub enum Event<'a> {
    Bare(MString<'a>),
    Array(Vec<MString<'a>>),
    Configured(Vec<NamedNode<'a, EventConfiguration<'a>>>),
}

pub enum EventConfiguration<'a> {
    Branches(Vec<MString<'a>>),
    BranchesIgnore(Vec<MString<'a>>),
    Paths(Vec<MString<'a>>),
    PathsIgnore(Vec<MString<'a>>),
    Tags(Vec<MString<'a>>),
    TagsIgnore(Vec<MString<'a>>),
    Schedule(Vec<Marked<Schedule<'a>>>),
    Inputs(Vec<NamedNode<'a, Input<'a>>>),
    InheritedSecrets(Vec<NamedNode<'a, WorkflowInheritedSecret<'a>>>),
    Outputs(Vec<NamedNode<'a, WorkflowOutput<'a>>>),
    Workflows(Vec<MString<'a>>),
}

pub enum WorkflowInheritedSecret<'a> {
    Required(MString<'a>),
}

pub enum Input<'a> {
    Description(MString<'a>),
    DefaultValue(MString<'a>),
    Required(MString<'a>),
    InputType(MString<'a>),
    Options(Vec<MString<'a>>),
}

pub enum WorkflowOutput<'a> {
    Value(MString<'a>),
    Description(MString<'a>),
}

pub enum Schedule<'a> {
    Cron(MString<'a>),
}
