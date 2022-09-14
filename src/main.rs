use controllers::dive_stage::run_dive_profile;
use presenters::presenter::select_cylinder;
use presenters::{
    dive_setup::{create_cylinders, create_dive_step, welcome_message},
    presenter::select_dive_model,
};

use crate::controllers::dive_stage::update_cylinder_gas_usage;

mod commands;
mod controllers;
mod models;
mod presenters;

fn main() {
    welcome_message();
    let mut dive_model = select_dive_model();
    let cylinders = create_cylinders();

    //loop {
    let dive_step = create_dive_step();
    let mut cylinder = select_cylinder(cylinders);
    dive_model.dive_profile = run_dive_profile(dive_model, dive_step, cylinder);
    cylinder = update_cylinder_gas_usage(cylinder, dive_step);
    println!("{}", dive_model.dive_profile);
    println!("{}", cylinder);
    // }
}
