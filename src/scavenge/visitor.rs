use super::workflow::*;

pub trait WorkflowVisitor:
    ConcurrencyVisitor + EnvVisitor + DefaultsVisitor + PermissionVisitor
{
    fn visit_name<'a, 'b>(&mut self, name: &'b MString<'a>);
    fn visit_run_name<'a, 'b>(&mut self, run_name: &'b MString<'a>);
    fn visit_on<'a, 'b>(&mut self, event: &'b Event<'a>);
    fn visit_jobs<'a, 'b>(&mut self, jobs: Vec<(MString<'a>, Vec<Job<'a>>)>);
    fn visit_invalid<'a, 'b>(&mut self, invalid: Marked<Invalid>);

    fn visit_workflow<'a, 'b>(&mut self, node: &'b Workflow<'a>) {
        match node {
            Workflow::Concurrency(c) => self.visit_concurrency(c),
            Workflow::Defaults(d) => self.visit_defaults(d),
            Workflow::Env(e) => self.visit_env(e),
            Workflow::Invalid(i) => self.visit_invalid(i),
            Workflow::Jobs(jobs) => self.visit_jobs(jobs),
            Workflow::Name(n) => self.visit_name(n),
            Workflow::On(on) => self.visit_on(on),
            Workflow::RunName(rn) => self.visit_run_name(rn),
            Workflow::Permission(p) => self.visit_permissions(p),
        }
    }
}

pub trait EventVisitor {
    fn visit_bare<'a, 'b>(&mut self, bare: &'b MString<'a>);
    fn visit_configured<'a, 'b>(
        &mut self,
        name: &'b MString<'a>,
        configuration: &'b Vec<EventConfiguration<'a>>,
    );
    fn visit_array<'a, 'b>(&mut self, names: &'b Vec<MString<'a>>) {
        for name in names.iter() {
            self.visit_bare(name)
        }
    }
    fn visit_event<'a, 'b>(&mut self, node: &'b Event<'a>) {
        match node {
            Event::Bare(b) => self.visit_bare(b),
            Event::Array(a) => self.visit_array(a),
            Event::Configured(c) => self.visit_configured(c),
        }
    }
}

pub trait ConcurrencyVisitor {
    fn visit_bare<'a, 'b>(&mut self, group: &'b MString<'a>);
    fn visit_name<'a, 'b>(&mut self, name: &'b MString<'a>);
    fn visit_cancel_in_progress<'a, 'b>(&mut self, cancel_in_progress: &'b MString<'a>);

    fn visit_concurrency<'a, 'b>(&mut self, concurrency: &'b Concurrency<'a>) {
        match concurrency {
            Concurrency::Bare(s) => self.visit_bare(s),
            Concurrency::Group(g) => self.visit_concurrency_group(g),
        }
    }

    fn visit_concurrency_group<'a, 'b>(&mut self, group: &'b ConcurrencyGroup) {
        match group {
            ConcurrencyGroup::Name(s) => self.visit_name(s),
            ConcurrencyGroup::CancelInProgress(s) => self.visit_cancel_in_progress(s),
        }
    }
}
pub trait EnvVisitor {
    fn visit_kvp<'a, 'b>(&mut self, kvp: &'b KVP<'a>);
    fn visit_env<'a, 'b>(&mut self, env: &'b Vec<KVP<'a>>) {
        for kvp in env.iter() {
            self.visit_kvp(kvp)
        }
    }
}
pub trait DefaultsVisitor {
    fn visit_shell<'a, 'b>(&mut self, shell: &'b MString<'a>);
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
    fn visit_global_grant<'a, 'b>(&mut self, grant: &'b MString<'a>);
    fn visit_global_revoke<'a, 'b>(&mut self);
    fn visit_individual_grant<'a, 'b>(&mut self, grant: KVP<'a>);

    fn visit_all_individual_grants<'a, 'b>(&mut self, grants: Vec<KVP<'a>>) {
        for grant in grants.iter() {
            self.visit_individual_grant(grant)
        }
    }

    fn visit_permissions<'a, 'b>(&mut self, permissions: &'b Permission) {
        match permissions {
            Permission::GlobalGrant(gg) => self.visit_global_grant(gg),
            Permission::GlobalRevoke => self.visit_global_revoke,
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
            Schedule(schedule) => self.visit_schedule(schedule),
            Inputs(inputs) => self.visit_inputs(inputs),
            InheritedSecrets(secrets) => self.visit_inherited_secrets(secrets),
            Outputs(outputs) => self.visit_outputs(outputs),
            Workflows(workflows) => self.visit_workflows(workflows),
        }
    }
    fn visit_branches<'a, 'b>(&mut self, branches: &'b Vec<MString<'a>>);
    fn visit_branches_ignore<'a, 'b>(&mut self, ignored_branches: &'b Vec<MString<'a>>);
    fn visit_paths<'a, 'b>(&mut self, paths: &'b Vec<MString<'a>>);
    fn visit_ignored_paths<'a, 'b>(&mut self, ignored_paths: &'b Vec<MString<'a>>);
    fn visit_tags<'a, 'b>(&mut self, tags: &'b Vec<MString<'a>>);
    fn visit_ignored_tags<'a, 'b>(&mut self, ignored_tags: &'b Vec<MString<'a>>);
    fn visit_schedule<'a, 'b>(&mut self, schedule: &'b Vec<Marked<Schedule>>);
    fn visit_inputs<'a, 'b>(&mut self, inputs: &'b Vec<(MString<'a>, Vec<Input<'a>>)>);
    fn visit_inherited_secrets<'a, 'b>(
        &mut self,
        secrets: &'b Vec<(MString<'a>, Vec<InheritedSecret<'a>>)>,
    );
    fn visit_outputs<'a, 'b>(&mut self, outputs: &'b Vec<(MString<'a>, Vec<WorkflowOutput>)>);
    fn visit_workflows<'a, 'b>(&mut self, workflows: &'b Vec<MString<'a>>);
}
