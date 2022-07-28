use crate::models::cylinder::Cylinder;
use crate::models::dive_model::DiveModel;
use crate::presenters::dive_data::cylinders::create_cylinders;
use crate::presenters::dive_data::dive_model::select_dive_model;
use crate::presenters::presenter::write_message;

pub fn new_dive_plan() -> (DiveModel, Vec<Cylinder>) {
    write_message(String::from("Welcome to Bubbles Dive Planner Console Rust"));

    let dive_model = select_dive_model();
    let cylinders = create_cylinders();
    (dive_model, cylinders)
}

