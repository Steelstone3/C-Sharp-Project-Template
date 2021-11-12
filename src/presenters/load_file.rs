use crate::commands::files::dive_profile::dive_profile::read_dive_profile_file;
use crate::commands::files::dive_step::dive_step::read_dive_step_file;
use crate::models::dive_profile::dive_profile_model::DiveProfile;
use crate::models::dive_step::dive_step::DiveStep;

pub mod load_file {
    use crate::{DiveProfile, DiveStep, read_dive_profile_file, read_dive_step_file};
    use crate::presenters::presenter::presenters::read_boolean;

    pub fn load_from_default_file() -> (Vec<DiveStep>, Vec<DiveProfile>) {
        let mut dive_steps: Vec<DiveStep> = Vec::new();
        let mut dive_profiles: Vec<DiveProfile> = Vec::new();

        if read_boolean("\nDo you wish to load the current dive plan [y/N]:") {
            dive_steps = read_dive_step_file();
            dive_profiles = read_dive_profile_file();
        }

        return (dive_steps, dive_profiles)
    }
}


