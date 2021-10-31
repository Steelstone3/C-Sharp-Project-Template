pub mod dive_plan {
    use crate::models::dive_profile::dive_profile_model::DiveProfileModel;
    use crate::models::dive_step::dive_step::DiveStep;

    pub fn update_dive_plan(dive_steps: &mut Vec<DiveStep>, dive_profiles: &mut Vec<DiveProfileModel>, dive_profile: DiveProfileModel, dive_step: DiveStep) {
        dive_steps.push(dive_step);
        dive_profiles.push(dive_profile);
    }
}


