pub mod dive_stage {
    use crate::commands::a_b_values::a_b_values::{calculate_a_value, calculate_b_value};
    use crate::commands::ambient_pressures::ambient_pressures::calculate_ambient_pressure;
    use crate::commands::compartment_loads::compartment_loads::calculate_compartment_load;
    use crate::commands::max_surface_pressures::max_surface_pressures::calculate_max_surface_pressure;
    use crate::commands::tissue_pressures::tissue_pressure::{calculate_tissue_pressure_helium, calculate_tissue_pressure_nitrogen, calculate_tissue_pressure_total};
    use crate::commands::tolerated_ambient_pressures::tolerated_ambient_pressures::calculate_tolerated_ambient_pressure;
    use crate::models::dive_model::dive_model::DiveModel;
    use crate::models::dive_profile::dive_profile_model::DiveProfileModel;
    use crate::models::dive_step::dive_step::DiveStep;
    use crate::models::gas_mixture::gas_mixture::GasMixture;

    pub fn run_dive_profile(dive_model: DiveModel, mut dive_profile_model: DiveProfileModel, dive_step: DiveStep, gas_mixture: GasMixture) -> DiveProfileModel {
        dive_profile_model = calculate_ambient_pressure(dive_profile_model, dive_step, gas_mixture);

        for compartment in 0..dive_model.compartment_count {
            dive_profile_model = update_dive_profile_model(compartment, dive_model, dive_profile_model, dive_step);
        }

        return dive_profile_model;
    }

    fn update_dive_profile_model(compartment: usize, dive_model: DiveModel, mut dive_profile_model: DiveProfileModel, dive_step: DiveStep) -> DiveProfileModel {
        dive_profile_model.tissue_pressures_nitrogen[compartment] = calculate_tissue_pressure_nitrogen(compartment, dive_model, dive_profile_model, dive_step);
        dive_profile_model.tissue_pressures_helium[compartment] = calculate_tissue_pressure_helium(compartment, dive_model, dive_profile_model, dive_step);
        dive_profile_model.tissue_pressures_total[compartment] = calculate_tissue_pressure_total(compartment, dive_profile_model);
        dive_profile_model.a_values[compartment] = calculate_a_value(compartment, dive_model, dive_profile_model);
        dive_profile_model.b_values[compartment] = calculate_b_value(compartment, dive_model, dive_profile_model);
        dive_profile_model.tolerated_ambient_pressures[compartment] = calculate_tolerated_ambient_pressure(compartment, dive_profile_model);
        dive_profile_model.max_surface_pressures[compartment] = calculate_max_surface_pressure(compartment, dive_profile_model);
        dive_profile_model.compartment_load[compartment] = calculate_compartment_load(compartment, dive_profile_model);

        return dive_profile_model;
    }
}