pub mod execute_dive_plan {
    use crate::controllers::dive_stage::dive_stage::run_dive_profile;
    use crate::controllers::gas_management::gas_management::update_gas_management;
    use crate::{DiveProfile, DiveStep};
    use crate::models::cylinder::cylinder::Cylinder;
    use crate::models::dive_model::dive_model::DiveModel;
    use crate::presenters::dive_data::cylinder::cylinder::display_gas_management;
    use crate::presenters::dive_data::cylinders::cylinder::select_cylinder;
    use crate::presenters::dive_data::dive_results::dive_results::display_results;
    use crate::presenters::dive_data::dive_step::dive_step::enter_dive_step;

    pub fn execute_dive_plan(mut dive_model: DiveModel, mut cylinders: &mut Vec<Cylinder>) -> (DiveProfile, DiveStep) {
        let dive_step = enter_dive_step();
        let cylinder_selection = select_cylinder(&mut cylinders);
        dive_model.dive_profile = run_dive_profile(dive_model, dive_step, cylinders[cylinder_selection].gas_mixture);
        display_results(dive_model.dive_profile);
        display_gas_management(update_gas_management(cylinders[cylinder_selection].gas_management, dive_step));

        return (dive_model.dive_profile, dive_step);
    }
}