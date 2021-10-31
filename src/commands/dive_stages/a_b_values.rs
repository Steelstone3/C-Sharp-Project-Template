pub mod a_b_values {
    use crate::models::dive_model::dive_model::DiveModel;
    use crate::models::dive_profile::dive_profile_model::DiveProfile;

    pub fn calculate_a_value(compartment: usize, dive_model: DiveModel, dive_profile_model: DiveProfile) -> f32 {
        return (dive_model.a_values_nitrogen[compartment] * dive_profile_model.tissue_pressures_nitrogen[compartment] + dive_model.a_values_helium[compartment] * dive_profile_model.tissue_pressures_helium[compartment]) / dive_profile_model.tissue_pressures_total[compartment];
    }

    pub fn calculate_b_value(compartment: usize, dive_model: DiveModel, dive_profile_model: DiveProfile) -> f32 {
        return (dive_model.b_values_nitrogen[compartment] * dive_profile_model.tissue_pressures_nitrogen[compartment] + dive_model.b_values_helium[compartment] * dive_profile_model.tissue_pressures_helium[compartment]) / dive_profile_model.tissue_pressures_total[compartment];
    }
}