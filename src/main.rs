use std::fs::File;
use std::io::Write;
use crate::presenters::dive_model::dive_model::select_dive_model;
use crate::presenters::dive_step::dive_step::enter_dive_step;
use crate::controllers::dive_stage::dive_stage::run_dive_profile;
use crate::controllers::gas_management::gas_management::update_gas_management;
use crate::factories::zhl16_dive_model::zhl16_dive_model::create_zhl16_dive_profile;
use crate::models::dive_profile::dive_profile_model::DiveProfileModel;
use crate::models::dive_step::dive_step::DiveStep;
use crate::presenters::cylinder::cylinder::display_gas_management;
use crate::presenters::cylinders::cylinder::{create_cylinders, select_cylinder};
use crate::presenters::dive_results::dive_results::display_results;
use crate::presenters::presenter::presenters::write_message;

mod presenters;
mod models;
mod controllers;
mod commands;
mod factories;

#[cfg(debug_assertions)]
mod tests;

fn main() -> std::io::Result<()> {
    write_message(String::from("Welcome to Bubbles Dive Planner Console Rust"));
    let dive_model = select_dive_model();
    let mut dive_profile = create_zhl16_dive_profile();
    let mut cylinders = create_cylinders();

    loop {
        let dive_step = enter_dive_step();
        let cylinder_selection = select_cylinder(&mut cylinders);
        dive_profile = run_dive_profile(dive_model, dive_profile, dive_step, cylinders[cylinder_selection].gas_mixture);
        display_results(dive_profile);
        display_gas_management(update_gas_management(cylinders[cylinder_selection].gas_management, dive_step));

        create_dive_step_file(&dive_step);
        create_dive_profile_file(&mut dive_profile);
    }
}

fn create_dive_profile_file(dive_profile: &mut DiveProfileModel) -> std::io::Result<()> {
    let mut json_dive_profile_file = File::create("dive_profile.json")?;
    let json_dive_profile = serde_json::ser::to_string_pretty(&dive_profile)?;
    write!(json_dive_profile_file, "{}", json_dive_profile)?;
    Ok(())
}

fn create_dive_step_file(dive_step: &DiveStep) -> std::io::Result<()> {
    let mut json_dive_step_file = File::create("dive_step.json")?;
    let json_dive_step = serde_json::ser::to_string_pretty(&dive_step)?;
    write!(json_dive_step_file, "{}", json_dive_step)?;
    Ok(())
}