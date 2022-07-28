use crate::{DiveProfile, DiveStep};
use crate::commands::files::dive_profile::upsert_dive_profile_file;
use crate::commands::files::dive_step::upsert_dive_step_file;
use crate::commands::states::dive_plan::update_dive_plan;
use crate::commands::files::error_messages::CAN_NOT_UPDATE_FILE_ERROR;

pub fn update_files(mut dive_profiles: &mut Vec<DiveProfile>, mut dive_steps: &mut Vec<DiveStep>, dive_plan_data: (DiveProfile, DiveStep)) {
    update_dive_plan(&mut dive_steps, &mut dive_profiles, dive_plan_data.0, dive_plan_data.1);
    upsert_dive_step_file(&dive_steps).expect(CAN_NOT_UPDATE_FILE_ERROR);
    upsert_dive_profile_file(&mut dive_profiles).expect(CAN_NOT_UPDATE_FILE_ERROR);
}
