pub mod tissue_pressure {
    use crate::models::dive_model::dive_model::DiveModel;
    use crate::models::dive_profile::dive_profile_model::DiveProfile;
    use crate::models::dive_step::dive_step::DiveStep;

    pub fn calculate_tissue_pressure_nitrogen(compartment: usize, dive_model: DiveModel, dive_step: DiveStep) -> f32 {
        return dive_model.dive_profile.tissue_pressures_nitrogen[compartment] + ((dive_model.dive_profile.nitrogen_at_pressure - dive_model.dive_profile.tissue_pressures_nitrogen[compartment]) * (1.0 - f32::powf(2.0, -(dive_step.time as f32 / dive_model.nitrogen_half_time[compartment]))));
    }

    pub fn calculate_tissue_pressure_helium(compartment: usize, dive_model: DiveModel, dive_step: DiveStep) -> f32 {
        return dive_model.dive_profile.tissue_pressures_helium[compartment] + ((dive_model.dive_profile.helium_at_pressure - dive_model.dive_profile.tissue_pressures_helium[compartment]) * (1.0 - f32::powf(2.0, -(dive_step.time as f32 / dive_model.helium_half_time[compartment]))));
    }

    pub fn calculate_tissue_pressure_total(compartment: usize, dive_profile: DiveProfile) -> f32 {
        return dive_profile.tissue_pressures_helium[compartment] + dive_profile.tissue_pressures_nitrogen[compartment];
    }
}

