use crate::models::dive_profile::dive_profile_model::DiveProfile;
use crate::models::dive_step::dive_step::DiveStep;
use crate::presenters::dive_plan::execute_dive_plan::execute_dive_plan::execute_dive_plan;
use crate::presenters::dive_plan::load_dive_plan::load_file::{load_from_default_file};
use crate::presenters::dive_plan::new_dive_plan::new_dive_plan::new_dive_plan;
use crate::presenters::dive_plan::update_files::update_files::update_files;

mod presenters;
mod models;
mod controllers;
mod commands;
mod factories;

#[cfg(debug_assertions)]
mod tests;

fn main() -> std::io::Result<()> {
    let (mut dive_steps, mut dive_profiles) = load_from_default_file();
    let (dive_model, mut cylinders, dive_step) = new_dive_plan();

    loop {
        let dive_plan_data:(DiveProfile, DiveStep) = execute_dive_plan(dive_model, &mut cylinders, dive_step);
        update_files(&mut dive_steps, &mut dive_profiles, dive_plan_data);
    }
}