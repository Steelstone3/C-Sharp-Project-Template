use presenters::load_file;
use crate::commands::files::dive_profile::dive_profile::{read_dive_profile_file, upsert_dive_profile_file};
use crate::commands::files::dive_step::dive_step::{read_dive_step_file, upsert_dive_step_file};
use crate::commands::states::dive_plan::dive_plan::update_dive_plan;
use crate::controllers::dive_stage::dive_stage::run_dive_profile;
use crate::controllers::gas_management::gas_management::update_gas_management;
use crate::models::cylinder::cylinder::Cylinder;
use crate::models::dive_model::dive_model::DiveModel;
use crate::models::dive_profile::dive_profile_model::DiveProfile;
use crate::models::dive_step::dive_step::DiveStep;
use crate::presenters::cylinder::cylinder::display_gas_management;
use crate::presenters::cylinders::cylinder::{create_cylinders, select_cylinder};
use crate::presenters::dive_model::dive_model::select_dive_model;
use crate::presenters::dive_results::dive_results::display_results;
use crate::presenters::dive_step::dive_step::enter_dive_step;
use crate::presenters::load_file::load_file::load_from_default_file;
use crate::presenters::presenter::presenters::write_message;

mod presenters;
mod models;
mod controllers;
mod commands;
mod factories;

#[cfg(debug_assertions)]
mod tests;

fn main() -> std::io::Result<()> {
    let mut dive_steps: Vec<DiveStep> = Vec::new();
    let mut dive_profiles: Vec<DiveProfile> = Vec::new();

    //load dive plan
    let loaded_dive_application_data = load_from_default_file();
    dive_steps = loaded_dive_application_data.0;
    dive_profiles = loaded_dive_application_data.1;

    //new dive plan
    write_message(String::from("Welcome to Bubbles Dive Planner Console Rust"));

    let mut dive_model = select_dive_model();
    let mut cylinders = create_cylinders();
    let mut dive_step = DiveStep { depth: 0, time: 0 };

    loop {
        dive_step = enter_dive_step();
        let cylinder_selection = select_cylinder(&mut cylinders);
        dive_model.dive_profile = run_dive_profile(dive_model, dive_step, cylinders[cylinder_selection].gas_mixture);
        display_results(dive_model.dive_profile);
        display_gas_management(update_gas_management(cylinders[cylinder_selection].gas_management, dive_step));

        update_dive_plan(&mut dive_steps, &mut dive_profiles, dive_model.dive_profile, dive_step);
        upsert_dive_step_file(&dive_steps).expect("Failed to upsert dive_step.json file");
        upsert_dive_profile_file(&mut dive_profiles).expect("Failed to upsert dive_profile.json file");
    }
}