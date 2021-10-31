pub mod dive_stage {
    use crate::commands::dive_stages::a_b_values::a_b_values::{calculate_a_value, calculate_b_value};
    use crate::commands::dive_stages::ambient_pressures::ambient_pressures::calculate_ambient_pressure;
    use crate::commands::dive_stages::compartment_loads::compartment_loads::calculate_compartment_load;
    use crate::commands::dive_stages::max_surface_pressures::max_surface_pressures::calculate_max_surface_pressure;
    use crate::commands::dive_stages::tissue_pressures::tissue_pressure::{calculate_tissue_pressure_helium, calculate_tissue_pressure_nitrogen, calculate_tissue_pressure_total};
    use crate::commands::dive_stages::tolerated_ambient_pressures::tolerated_ambient_pressures::calculate_tolerated_ambient_pressure;
    use crate::models::dive_model::dive_model::DiveModel;
    use crate::models::dive_profile::dive_profile_model::DiveProfile;
    use crate::models::dive_step::dive_step::DiveStep;
    use crate::models::gas_mixture::gas_mixture::GasMixture;

    pub fn run_dive_profile(mut dive_model: DiveModel, dive_step: DiveStep, gas_mixture: GasMixture) -> DiveProfile {
        dive_model.dive_profile = calculate_ambient_pressure(dive_model.dive_profile, dive_step, gas_mixture);

        for compartment in 0..dive_model.compartment_count {
            dive_model.dive_profile = update_dive_profile_model(compartment, dive_model, dive_step);
        }

        return dive_model.dive_profile;
    }

    fn update_dive_profile_model(compartment: usize, mut dive_model: DiveModel, dive_step: DiveStep) -> DiveProfile {
        dive_model.dive_profile.tissue_pressures_nitrogen[compartment] = calculate_tissue_pressure_nitrogen(compartment, dive_model, dive_step);
        dive_model.dive_profile.tissue_pressures_helium[compartment] = calculate_tissue_pressure_helium(compartment, dive_model, dive_step);
        dive_model.dive_profile.tissue_pressures_total[compartment] = calculate_tissue_pressure_total(compartment, dive_model.dive_profile);
        dive_model.dive_profile.a_values[compartment] = calculate_a_value(compartment, dive_model, dive_model.dive_profile);
        dive_model.dive_profile.b_values[compartment] = calculate_b_value(compartment, dive_model, dive_model.dive_profile);
        dive_model.dive_profile.tolerated_ambient_pressures[compartment] = calculate_tolerated_ambient_pressure(compartment, dive_model.dive_profile);
        dive_model.dive_profile.maximum_surface_pressures[compartment] = calculate_max_surface_pressure(compartment, dive_model.dive_profile);
        dive_model.dive_profile.compartment_load[compartment] = calculate_compartment_load(compartment, dive_model.dive_profile);

        return dive_model.dive_profile;
    }
}