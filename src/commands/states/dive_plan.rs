use crate::{DiveProfile, DiveStep};

pub fn update_dive_plan(dive_steps: &mut Vec<DiveStep>, dive_profiles: &mut Vec<DiveProfile>, dive_profile: DiveProfile, dive_step: DiveStep) {
    dive_steps.push(dive_step);
    dive_profiles.push(dive_profile);
}

#[cfg(test)]
mod dive_plan_should {
    #[test]
    #[ignore]
    fn update_current_application_state() {}

    #[test]
    #[ignore]
    fn undo_last_dive_step() {}

    #[test]
    #[ignore]
    fn redo_last_dive_step() {}
}