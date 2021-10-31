pub mod ambient_pressures {
    use crate::models::dive_profile::dive_profile_model::DiveProfileModel;
    use crate::models::dive_step::dive_step::DiveStep;
    use crate::models::gas_mixture::gas_mixture::GasMixture;

    pub fn calculate_ambient_pressure(mut dive_profile_model: DiveProfileModel, dive_step: DiveStep, gas_mixture: GasMixture) -> DiveProfileModel {
        let ambient_pressure = 1.0 + dive_step.depth as f32 / 10.0;
        dive_profile_model.nitrogen_at_pressure = gas_mixture.nitrogen as f32 / 100.0 * ambient_pressure;
        dive_profile_model.oxygen_at_pressure = gas_mixture.oxygen as f32 / 100.0 * ambient_pressure;
        dive_profile_model.helium_at_pressure = gas_mixture.helium as f32 / 100.0 * ambient_pressure;

        return dive_profile_model;
    }
}