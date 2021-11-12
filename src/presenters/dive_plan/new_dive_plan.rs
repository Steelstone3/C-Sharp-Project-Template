pub mod new_dive_plan {
    use crate::DiveStep;
    use crate::models::cylinder::cylinder::Cylinder;
    use crate::models::dive_model::dive_model::DiveModel;
    use crate::presenters::dive_data::cylinders::cylinder::create_cylinders;
    use crate::presenters::dive_data::dive_model::dive_model::select_dive_model;
    use crate::presenters::presenter::presenters::write_message;

    pub fn new_dive_plan() -> (DiveModel, Vec<Cylinder>, DiveStep) {
        write_message(String::from("Welcome to Bubbles Dive Planner Console Rust"));

        let mut dive_model = select_dive_model();
        let mut cylinders = create_cylinders();
        let mut dive_step = DiveStep { depth: 0, time: 0 };
        (dive_model, cylinders, dive_step)
    }
}

