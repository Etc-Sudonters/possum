use super::workflow::*;

pub trait WorkflowVisitor:
    ConcurrencyVisitor + EnvVisitor + DefaultsVisitor + PermissionVisitor
{
    fn visit_workflow_name<'a, 'b>(&mut self, name: &'b MString<'a>) {}
    fn visit_run_name<'a, 'b>(&mut self, run_name: &'b MString<'a>) {}
    fn visit_on<'a, 'b>(&mut self, event: &'b Event<'a>) {}
    fn visit_job<'a, 'b>(&mut self, jobs: &'b JobNode<'a>) {}

    fn visit_workflow<'a, 'b>(&mut self, node: &'b Workflow<'a>) {
        match node {
            Workflow::Concurrency(c) => self.visit_concurrency(c),
            Workflow::Defaults(d) => self.visit_defaults(d),
            Workflow::Env(e) => self.visit_env(e),
            Workflow::Jobs(jobs) => self.visit_jobs(jobs),
            Workflow::Name(n) => self.visit_workflow_name(n),
            Workflow::On(on) => self.visit_on(on),
            Workflow::RunName(rn) => self.visit_run_name(rn),
            Workflow::Permission(p) => self.visit_permissions(p),
        }
    }
    fn visit_jobs<'a, 'b>(&mut self, jobs: &'b Vec<JobNode<'a>>) {
        for j in jobs.iter() {
            self.visit_job(j)
        }
    }
}

pub trait EventVisitor {
    fn visit_bare<'a, 'b>(&mut self, bare: &'b MString<'a>) {}
    fn visit_configured<'a, 'b>(&mut self, event: &'b NamedNode<'a, EventConfiguration<'a>>) {}

    fn visit_array<'a, 'b>(&mut self, names: &'b Vec<MString<'a>>) {
        for name in names.iter() {
            self.visit_bare(name)
        }
    }

    fn visit_all_configured<'a, 'b>(
        &mut self,
        configurations: &'b Vec<NamedNode<'a, EventConfiguration<'a>>>,
    ) {
        for c in configurations.iter() {
            self.visit_configured(c)
        }
    }
    fn visit_event<'a, 'b>(&mut self, node: &'b Event<'a>) {
        match node {
            Event::Bare(b) => self.visit_bare(b),
            Event::Array(a) => self.visit_array(a),
            Event::Configured(c) => self.visit_all_configured(c),
        }
    }
}

pub trait JobVisitor:
    ConcurrencyVisitor + DefaultsVisitor + EnvVisitor + PermissionVisitor
{
    fn visit_jobs<'a, 'b>(&mut self, nodes: &'b Vec<JobNode>) {
        for job in nodes {
            self.visit_job(job)
        }
    }

    fn visit_job<'a, 'b>(&mut self, node: &'b JobNode<'a>) {
        for job in node.children.iter() {
            use Job::*;
            match job {
                Concurrency(c) => self.visit_concurrency(c),
                Env(e) => self.visit_env(e),
                Permissions(p) => self.visit_permissions(p),
                Defaults(d) => self.visit_defaults(d),
                Container(c) => self.visit_containers(c),
                ContinueOnError(c) => self.visit_continue_on_error(c),
                Environment(e) => self.visit_environment(e),
                If(cond) => self.visit_condition(cond),
                Name(name) => self.visit_job_name(name),
                Needs(dependencies) => self.visit_dependencies(dependencies),
                Outputs(o) => self.visit_outputs(o),
                RunsOn(r) => self.visit_runs_on(r),
                Secrets(s) => self.visit_job_secrets(s),
                Services(s) => self.visit_services(s),
                Steps(s) => self.visit_steps(s),
                Strategy(s) => self.visit_strategy(s),
                TimeoutMinutes(t) => self.visit_timeout_minutes(t),
                Uses(u) => self.visit_uses(u),
                With(w) => self.visit_job_with(w),
            }
        }
    }

    fn visit_continue_on_error<'a, 'b>(&mut self, cont_on_err: &'b MString<'a>) {}
    fn visit_containers<'a, 'b>(&mut self, containers: &'b Vec<JobContainer<'a>>) {}
    fn visit_environment<'a, 'b>(&mut self, environment: &'b Environment<'a>) {}
    fn visit_condition<'a, 'b>(&mut self, cond: &'b MString<'a>) {}
    fn visit_job_name<'a, 'b>(&mut self, name: &'b MString<'a>) {}
    fn visit_outputs<'a, 'b>(&mut self, outputs: &'b MarkedMap<'a, MString<'a>>) {}
    fn visit_runs_on<'a, 'b>(&mut self, runs_on: &'b Vec<MString<'a>>) {}
    fn visit_job_secrets<'a, 'b>(&mut self, secrets: &'b PassedSecret<'a>) {}
    fn visit_services<'a, 'b>(&mut self, services: &'b Vec<JobServiceNode<'a>>) {}
    fn visit_steps<'a, 'b>(&mut self, steps: &'b Vec<StepNode<'a>>) {}
    fn visit_strategy<'a, 'b>(&mut self, strategy: &'b Strategy<'a>) {}
    fn visit_timeout_minutes<'a, 'b>(&mut self, timeout: &'b MString<'a>) {}
    fn visit_uses<'a, 'b>(&mut self, uses: &'b MString<'a>) {}
    fn visit_job_with<'a, 'b>(&mut self, with: &'b MarkedMap<'a, MString<'a>>) {}
    fn visit_dependencies<'a, 'b>(&mut self, dependencies: &'b Vec<MString<'a>>) {}
}

pub trait ConcurrencyVisitor {
    fn visit_bare<'a, 'b>(&mut self, group: &'b MString<'a>) {}
    fn visit_concurrency_name<'a, 'b>(&mut self, name: &'b MString<'a>) {}
    fn visit_cancel_in_progress<'a, 'b>(&mut self, cancel_in_progress: &'b MString<'a>) {}

    fn visit_concurrency<'a, 'b>(&mut self, concurrency: &'b Concurrency<'a>) {
        match concurrency {
            Concurrency::Bare(s) => self.visit_bare(s),
            Concurrency::Group(g) => self.visit_all_concurrency_groups(g),
        }
    }

    fn visit_all_concurrency_groups<'a, 'b>(&mut self, group: &'b Vec<ConcurrencyGroup>) {
        for c in group.iter() {
            self.visit_concurrency_group(c)
        }
    }

    fn visit_concurrency_group<'a, 'b>(&mut self, group: &'b ConcurrencyGroup) {
        match group {
            ConcurrencyGroup::Name(s) => self.visit_concurrency_name(s),
            ConcurrencyGroup::CancelInProgress(s) => self.visit_cancel_in_progress(s),
        }
    }
}
pub trait EnvVisitor {
    fn visit_key_value<'a, 'b>(&mut self, key: &'b MString<'a>, value: &'b MString<'a>) {}
    fn visit_env<'a, 'b>(&mut self, env: &'b MarkedMap<'a, MString<'a>>) {
        for (k, v) in env.iter() {
            self.visit_key_value(k, v)
        }
    }
}
pub trait DefaultsVisitor {
    fn visit_shell<'a, 'b>(&mut self, shell: &'b MString<'a>) {}
    fn visit_runs<'a, 'b>(&mut self, runs: &'b Runs<'a>) {
        match runs {
            Runs::Shell(s) => self.visit_shell(s),
        }
    }
    fn visit_defaults<'a, 'b>(&mut self, defaults: &'b Defaults<'a>) {
        match defaults {
            Defaults::Runs(r) => self.visit_runs(r),
        }
    }
}

pub trait PermissionVisitor {
    fn visit_global_grant<'a, 'b>(&mut self, grant: &'b MString<'a>) {}
    fn visit_global_revoke<'a, 'b>(&mut self) {}
    fn visit_individual_grant<'a, 'b>(&mut self, key: &'b MString<'a>, value: &'b MString<'a>) {}

    fn visit_all_individual_grants<'a, 'b>(&mut self, grants: &'b MarkedMap<'a, MString<'a>>) {
        for (k, v) in grants.iter() {
            self.visit_individual_grant(k, v)
        }
    }

    fn visit_permissions<'a, 'b>(&mut self, permissions: &'b Permission) {
        match permissions {
            Permission::GlobalGrant(gg) => self.visit_global_grant(gg),
            Permission::GlobalRevoke => self.visit_global_revoke(),
            Permission::InvididualGrant(grants) => self.visit_all_individual_grants(grants),
        }
    }
}

pub trait EventConfigurationVisitor {
    fn visit_event_configuration<'a, 'b>(&mut self, configuration: &'b EventConfiguration<'a>) {
        use EventConfiguration::*;
        match configuration {
            Branches(b) => self.visit_branches(b),
            BranchesIgnore(branches) => self.visit_branches_ignore(branches),
            Paths(paths) => self.visit_paths(paths),
            PathsIgnore(paths) => self.visit_ignored_paths(paths),
            Tags(tags) => self.visit_tags(tags),
            TagsIgnore(tags) => self.visit_ignored_tags(tags),
            Schedule(schedule) => self.visit_schedules(schedule),
            Inputs(inputs) => self.visit_inputs(inputs),
            InheritedSecrets(secrets) => self.visit_inherited_secrets(secrets),
            Outputs(outputs) => self.visit_outputs(outputs),
            Workflows(workflows) => self.visit_dependent_workflows(workflows),
        }
    }
    fn visit_branches<'a, 'b>(&mut self, branches: &'b Vec<MString<'a>>) {}
    fn visit_branches_ignore<'a, 'b>(&mut self, ignored_branches: &'b Vec<MString<'a>>) {}
    fn visit_paths<'a, 'b>(&mut self, paths: &'b Vec<MString<'a>>) {}
    fn visit_ignored_paths<'a, 'b>(&mut self, ignored_paths: &'b Vec<MString<'a>>) {}
    fn visit_tags<'a, 'b>(&mut self, tags: &'b Vec<MString<'a>>) {}
    fn visit_ignored_tags<'a, 'b>(&mut self, ignored_tags: &'b Vec<MString<'a>>) {}
    fn visit_schedules<'a, 'b>(&mut self, schedules: &'b Vec<Marked<Schedule>>) {
        for s in schedules.iter() {
            self.visit_schedule(s)
        }
    }
    fn visit_schedule<'a, 'b>(&mut self, schedule: &'b Marked<Schedule>) {}
    fn visit_inputs<'a, 'b>(&mut self, inputs: &'b Vec<NamedNode<'a, Input<'a>>>) {
        for input in inputs.iter() {
            self.visit_input(input)
        }
    }
    fn visit_input<'a, 'b>(&mut self, input: &'b NamedNode<'a, Input<'a>>) {}
    fn visit_inherited_secrets<'a, 'b>(
        &mut self,
        secrets: &'b Vec<NamedNode<'a, WorkflowInheritedSecret<'a>>>,
    ) {
        for secret in secrets.iter() {
            self.visit_inherited_secret(secret)
        }
    }
    fn visit_inherited_secret<'a, 'b>(
        &mut self,
        secret: &'b NamedNode<'a, WorkflowInheritedSecret<'a>>,
    ) {
    }
    fn visit_outputs<'a, 'b>(&mut self, outputs: &'b Vec<NamedNode<'a, WorkflowOutput<'a>>>) {
        for output in outputs {
            self.visit_output(output)
        }
    }
    fn visit_output<'a, 'b>(&mut self, output: &'b NamedNode<'a, WorkflowOutput<'a>>) {}
    fn visit_dependent_workflows<'a, 'b>(&mut self, dependencies: &'b Vec<MString<'a>>) {
        for dependency in dependencies.iter() {
            self.visit_depedendent_workflow(dependency)
        }
    }
    fn visit_depedendent_workflow<'a, 'b>(&mut self, workflow: &'a MString<'a>) {}
}

pub trait WorkflowInputVisitor {
    fn visit_workflow_input<'a, 'b>(&mut self, input: &'b Input<'a>) {
        use Input::*;
        match input {
            Description(description) => self.visit_description(description),
            DefaultValue(default) => self.visit_default_value(default),
            Required(required) => self.visit_required(required),
            InputType(t) => self.visit_input_type(t),
            Options(opts) => self.visit_options(opts),
        }
    }

    fn visit_description<'a, 'b>(&mut self, description: &'b MString<'a>) {}
    fn visit_default_value<'a, 'b>(&mut self, default: &'b MString<'a>) {}
    fn visit_required<'a, 'b>(&mut self, required: &'b MString<'a>) {}
    fn visit_input_type<'a, 'b>(&mut self, input_type: &'b MString<'a>) {}
    fn visit_options<'a, 'b>(&mut self, options: &'b Vec<MString<'a>>) {}
    fn visit_dependencies<'a, 'b>(&mut self, dependencies: &'b Vec<MString<'a>>) {}
}
pub trait StepVisitor: EnvVisitor {
    fn visit_all_step<'a, 'b>(&mut self, step: &'b StepNode<'a>) {}
    fn visit_step<'a, 'b>(&mut self, step: &'b Step<'a>) {
        use Step::*;
        match step {
            Id(i) => self.visit_step_id(i),
            Name(n) => self.visit_step_name(n),
            If(cond) => self.visit_step_condition(cond),
            Uses(u) => self.visit_step_uses(u),
            Run(r) => self.visit_step_run(r),
            Shell(s) => self.visit_step_shell(s),
            Env(e) => self.visit_env(e),
            With(w) => self.visit_step_with(w),
        }
    }
    fn visit_step_id<'a, 'b>(&mut self, id: &'b MString<'a>) {}
    fn visit_step_name<'a, 'b>(&mut self, name: &'b MString<'a>) {}
    fn visit_step_condition<'a, 'b>(&mut self, cond: &'b MString<'a>) {}
    fn visit_step_uses<'a, 'b>(&mut self, uses: &'b MString<'a>) {}
    fn visit_step_run<'a, 'b>(&mut self, run: &'b MString<'a>) {}
    fn visit_step_shell<'a, 'b>(&mut self, shell: &'b MString<'a>) {}
    fn visit_step_with<'a, 'b>(&mut self, with: &'b StepWith<'a>) {}
}

pub trait JobContainerVisitor {}
pub trait JobOutputsVisitor {}
pub trait PassedSecretVisitor {}
pub trait ServicesVisitor {}
pub trait StrategyVisitor {}
