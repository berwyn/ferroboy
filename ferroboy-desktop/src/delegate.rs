use druid::{AppDelegate, Target};

pub struct TopLevelDelegate;

impl AppDelegate<crate::state::State> for TopLevelDelegate {
    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        data: &mut crate::state::State,
        _env: &druid::Env,
    ) -> druid::Handled {
        if cmd.is(crate::selectors::SELECTOR_STEP) {
            ferroboy::tick(&mut data.0).expect("Unable to step state!");

            let handle = cmd.get_unchecked(crate::selectors::SELECTOR_STEP);
            handle
                .submit_command(crate::selectors::SELECTOR_STEP_COMPLETE, (), Target::Auto)
                .expect("Unable to mark step as completed!");

            return druid::Handled::Yes;
        }

        druid::Handled::No
    }
}
