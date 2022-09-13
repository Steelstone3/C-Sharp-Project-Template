use controllers::dive_stage::run_dive_profile;
use presenters::presenter::select_cylinder;
use presenters::{
    dive_setup::{create_cylinders, create_dive_step, welcome_message},
    presenter::select_dive_model,
};

mod commands;
mod controllers;
mod models;
mod presenters;

fn main() {
    welcome_message();
    let dive_model = select_dive_model();
    let cylinders = create_cylinders();

    //loop {
    let dive_step = create_dive_step();
    let cylinder = select_cylinder(cylinders);
    run_dive_profile(dive_model, dive_step, cylinder.gas_mixture);
    println!("{}", dive_model.dive_profile);
    // }
}
