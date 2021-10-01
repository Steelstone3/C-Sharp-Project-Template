use presenters::presenter::write_message;
use presenters::cylinder::cylinder::enter_gas_mixture;
use presenters::dive_model::dive_model::select_dive_model;
use presenters::dive_profile::dive_profile::create_dive_profile;
use presenters::dive_step::dive_step::enter_dive_step;
use crate::controllers::dive_stage::dive_stage::run_dive_profile;
use crate::presenters::dive_results::dive_results::display_results;

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

    loop {
        let dive_step = enter_dive_step();
        let gas_mixture = enter_gas_mixture();
        dive_profile = run_dive_profile(dive_model, dive_profile, dive_step, gas_mixture);
        display_results(dive_profile);
    }
}

