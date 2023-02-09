use std::convert::AsRef;

pub type MString<'a> = Marked<&'a str>;
pub type KVP<'a> = (MString<'a>, MString<'a>);
pub struct Marked<T> {
    value: T,
    raw_document_pointer: u64,
}

impl<T> Marked<T> {
    pub fn new(inner: T, document_pointer: u64) -> Marked<T> {
        Marked {
            value: inner,
            raw_document_pointer: document_pointer,
        }
    }
}

impl<T> AsRef<T> for Marked<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

pub struct Invalid {}
pub enum Workflow<'a> {
    Name(MString<'a>),
    RunName(MString<'a>),
    On(Event<'a>),
    Env(Vec<KVP<'a>>),
    Defaults(Defaults<'a>),
    Concurrency(Concurrency<'a>),
    Jobs(Vec<(MString<'a>, Vec<Job<'a>>)>),
    Invalid(Marked<Invalid>),
    Permission(Permission<'a>),
}

pub enum Job<'a> {
    Concurrency(Concurrency<'a>),
    Container(Vec<JobContainer<'a>>),
    ContinueOnError(MString<'a>),
    Defaults(Defaults<'a>),
    Env(Vec<KVP<'a>>),
    Environment(Environment<'a>),
    If(MString<'a>),
    Name(MString<'a>),
    Needs(Vec<MString<'a>>),
    Outputs(Vec<KVP<'a>>),
    Permissions(Permission<'a>),
    RunsOn(Vec<MString<'a>>),
    Secrets(PassedSecret<'a>),
    Services(Vec<Vec<JobService<'a>>>),
    Steps(Vec<Vec<Step<'a>>>),
    Strategy(Strategy<'a>),
    TimeoutMinutes(MString<'a>),
    Uses(MString<'a>),
    With(Vec<KVP<'a>>),
}
pub enum PassedSecret<'a> {
    Inherit,
    Explicit(Vec<KVP<'a>>),
}
pub enum JobService<'a> {
    Image(MString<'a>),
    Credentials(Vec<ContainerCreds<'a>>),
    Env(Vec<KVP<'a>>),
    Ports(Vec<MString<'a>>),
    Volumes(Vec<MString<'a>>),
    Options(Vec<MString<'a>>),
}
pub enum JobContainer<'a> {
    Image(MString<'a>),
    Credentials(Vec<ContainerCreds<'a>>),
    Env(Vec<KVP<'a>>),
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
    Env(Vec<KVP<'a>>),
    With(StepWith<'a>),
}

pub enum StepWith<'a> {
    Envish(Vec<KVP<'a>>),
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
    InvididualGrant(Vec<(MString<'a>, MString<'a>)>),
}

pub enum Event<'a> {
    Bare(MString<'a>),
    Array(Vec<MString<'a>>),
    Configured(Vec<(MString<'a>, Vec<EventConfiguration<'a>>)>),
}

pub enum EventConfiguration<'a> {
    Branches(Vec<MString<'a>>),
    BranchesIgnore(Vec<MString<'a>>),
    Paths(Vec<MString<'a>>),
    PathsIgnore(Vec<MString<'a>>),
    Tags(Vec<MString<'a>>),
    TagsIgnore(Vec<MString<'a>>),
    Schedule(Vec<Marked<Schedule<'a>>>),
    Inputs(Vec<(MString<'a>, Vec<Input<'a>>)>),
    InheritedSecrets(Vec<(MString<'a>, Vec<InheritedSecret<'a>>)>),
    Outputs(Vec<(MString<'a>, Vec<WorkflowOutput<'a>>)>),
    Workflows(Vec<MString<'a>>),
}

pub enum InheritedSecret<'a> {
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
