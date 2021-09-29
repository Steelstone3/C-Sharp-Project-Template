pub mod tissue_pressure {
    use crate::models::dive_model::dive_model::DiveModel;
    use crate::models::dive_profile::dive_profile_model::DiveProfileModel;
    use crate::models::dive_step::dive_step::DiveStep;

    pub fn calculate_tissue_pressure_nitrogen(compartment: usize, dive_model: DiveModel, dive_profile_model: DiveProfileModel, dive_step: DiveStep) -> f32 {
        return dive_profile_model.tissue_pressures_nitrogen[compartment] + ((dive_profile_model.nitrogen_at_pressure - dive_profile_model.tissue_pressures_nitrogen[compartment]) * (1.0 - f32::powf(2.0, -(dive_step.time as f32 / dive_model.nitrogen_half_time[compartment]))));
    }

    pub fn calculate_tissue_pressure_helium(compartment: usize, dive_model: DiveModel, dive_profile_model: DiveProfileModel, dive_step: DiveStep) -> f32 {
        return dive_profile_model.tissue_pressures_helium[compartment] + ((dive_profile_model.helium_at_pressure - dive_profile_model.tissue_pressures_helium[compartment]) * (1.0 - f32::powf(2.0, -(dive_step.time as f32 / dive_model.helium_half_time[compartment]))));
    }

    pub fn calculate_tissue_pressure_total(compartment: usize, dive_profile_model: DiveProfileModel) -> f32 {
        return dive_profile_model.tissue_pressures_helium[compartment] + dive_profile_model.tissue_pressures_nitrogen[compartment];
    }
}

#[cfg(test)]
mod tissue_pressure_should {
    #[test]
    #[ignore]
    fn calculate_tissue_pressure_nitrogen() {}

    #[test]
    #[ignore]
    fn calculate_tissue_pressure_helium() {}

    #[test]
    #[ignore]
    fn calculate_tissue_pressure_total() {}
}