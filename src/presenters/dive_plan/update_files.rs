pub mod update_files {
    use crate::commands::files::dive_profile::dive_profile::upsert_dive_profile_file;
    use crate::commands::files::dive_step::dive_step::upsert_dive_step_file;
    use crate::commands::states::dive_plan::dive_plan::update_dive_plan;
    use crate::{DiveProfile, DiveStep};

    pub fn update_files(mut dive_steps: &mut Vec<DiveStep>, mut dive_profiles: &mut Vec<DiveProfile>, dive_plan_data: (DiveProfile, DiveStep)) {
        update_dive_plan(&mut dive_steps, &mut dive_profiles, dive_plan_data.0, dive_plan_data.1);
        upsert_dive_step_file(&dive_steps).expect("Failed to upsert dive_step.json file");
        upsert_dive_profile_file(&mut dive_profiles).expect("Failed to upsert dive_profile.json file");
    }
}
