use presenters::dive_model::dive_model::select_dive_model;
use presenters::dive_profile::dive_profile::create_dive_profile;
use presenters::dive_step::dive_step::enter_dive_step;
use crate::controllers::dive_stage::dive_stage::run_dive_profile;
use crate::controllers::gas_management::gas_management::update_gas_management;
use crate::presenters::cylinder::cylinder::{create_cylinder, display_cylinder};
use crate::presenters::dive_results::dive_results::display_results;
use crate::presenters::presenter::presenters::write_message;

mod presenters;
mod models;
mod controllers;
mod commands;

#[cfg(debug_assertions)]
mod tests;

fn main() {
    write_message(String::from("Welcome to Bubbles Dive Planner Console Rust"));
    let dive_model = select_dive_model();
    let mut dive_profile = create_dive_profile();
    let mut cylinder = create_cylinder();

    loop {
        let dive_step = enter_dive_step();
        dive_profile = run_dive_profile(dive_model, dive_profile, dive_step, cylinder.gas_mixture);
        display_results(dive_profile);
        cylinder.gas_management = update_gas_management(cylinder.gas_management,dive_step);
        display_cylinder(cylinder.gas_management);
    }
}