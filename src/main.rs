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
    // let mut state = DivePlanState::default();

    welcome_message();
    
    //TODO option to load from file here which populates the models
    let mut dive_model = select_dive_model();
    let cylinders = create_cylinders();

    loop {
        let dive_step = create_dive_step();
        //TODO to_owned() means that cylinder won't update properly
        let mut cylinder = select_cylinder(cylinders.to_owned());
        dive_model.dive_profile = run_dive_profile(dive_model, dive_step, cylinder);
        cylinder = update_cylinder_gas_usage(cylinder, dive_step);
        println!("{}", dive_model.dive_profile);
        println!("{}", cylinder);

        //TODO write dive_model.dive_profile and cylinder/s to file
    }
}