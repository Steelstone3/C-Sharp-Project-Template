pub mod new_dive_plan {
    use crate::models::cylinder::cylinder::Cylinder;
    use crate::models::dive_model::dive_model::DiveModel;
    use crate::presenters::dive_data::cylinders::cylinder::create_cylinders;
    use crate::presenters::dive_data::dive_model::dive_model::select_dive_model;
    use crate::presenters::presenter::presenters::write_message;

    pub fn new_dive_plan() -> (DiveModel, Vec<Cylinder>) {
        write_message(String::from("Welcome to Bubbles Dive Planner Console Rust"));

        let dive_model = select_dive_model();
        let cylinders = create_cylinders();
        (dive_model, cylinders)
    }
}

