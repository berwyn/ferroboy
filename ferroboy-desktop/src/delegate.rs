use druid::AppDelegate;

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
            let state = data.clone();
            let mut state = state
                .write()
                .expect("Unable to get handle on state to step!");

            ferroboy::tick(&mut state).expect("Unable to step state!");

            return druid::Handled::Yes;
        }

        druid::Handled::No
    }
}
