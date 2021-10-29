use presenters::dive_model::dive_model::select_dive_model;
use presenters::dive_step::dive_step::enter_dive_step;

use crate::controllers::dive_stage::dive_stage::run_dive_profile;
use crate::controllers::gas_management::gas_management::update_gas_management;
use crate::factories::zhl16_dive_model::zhl16_dive_model::create_zhl16_dive_profile;
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

fn main() {
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
    }
}