use controllers::gas_mixture::gas_mixture_controller;
use models::dive_step::dive_step::DiveStep;
use models::gas_mixture::gas_mixture::GasMixture;
use models::dive_model::dive_model::DiveModel;
use presenters::presenter::write;
use presenters::cylinder::cylinder::enter_gas_mixture;
use presenters::dive_model::dive_model::select_dive_model;
use presenters::dive_step::dive_step::enter_dive_step;
use crate::controllers::dive_stage::dive_stage::run_dive_profile;
use crate::presenters::dive_results::dive_results::display_results;

mod presenters;
mod models;
mod controllers;

fn main() {
    write(String::from("Welcome to Bubbles Dive Planner Console Rust"));

    let dive_model = select_dive_model();
    let dive_step = enter_dive_step();
    let gas_mixture = enter_gas_mixture();
    let results = run_dive_profile(dive_model, dive_step, gas_mixture);
    display_results(results); //results as a parameter when you figure out what you want
}