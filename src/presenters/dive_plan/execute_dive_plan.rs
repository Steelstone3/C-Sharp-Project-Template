use crate::{DiveProfile, DiveStep};
use crate::controllers::dive_stage::run_dive_profile;
use crate::controllers::gas_management::update_gas_management;
use crate::models::cylinder::Cylinder;
use crate::models::dive_model::DiveModel;
use crate::presenters::dive_data::cylinder::display_gas_management;
use crate::presenters::dive_data::cylinders::select_cylinder;
use crate::presenters::dive_data::dive_results::display_results;
use crate::presenters::dive_data::dive_step::enter_dive_step;

pub fn execute_dive_plan(mut dive_model: DiveModel, cylinders: &mut Vec<Cylinder>) -> (DiveProfile, DiveStep) {
    let dive_step = enter_dive_step();
    let cylinder_selection = select_cylinder(cylinders);
    dive_model.dive_profile = run_dive_profile(dive_model, dive_step, cylinders[cylinder_selection].gas_mixture);
    display_results(dive_model.dive_profile);
    display_gas_management(update_gas_management(cylinders[cylinder_selection].gas_management, dive_step));

    (dive_model.dive_profile, dive_step)
}