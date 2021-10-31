pub mod a_b_values {
    use crate::models::dive_model::dive_model::DiveModel;

    pub fn calculate_a_value(compartment: usize, dive_model: DiveModel) -> f32 {
        return (dive_model.a_values_nitrogen[compartment] * dive_model.dive_profile.tissue_pressures_nitrogen[compartment] + dive_model.a_values_helium[compartment] * dive_model.dive_profile.tissue_pressures_helium[compartment]) / dive_model.dive_profile.tissue_pressures_total[compartment];
    }

    pub fn calculate_b_value(compartment: usize, dive_model: DiveModel) -> f32 {
        return (dive_model.b_values_nitrogen[compartment] * dive_model.dive_profile.tissue_pressures_nitrogen[compartment] + dive_model.b_values_helium[compartment] * dive_model.dive_profile.tissue_pressures_helium[compartment]) / dive_model.dive_profile.tissue_pressures_total[compartment];
    }
}