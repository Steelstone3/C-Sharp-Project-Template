pub mod compartment_loads {
    use crate::models::dive_profile::dive_profile_model::DiveProfileModel;

    pub fn calculate_compartment_load(compartment: usize, dive_profile_model: DiveProfileModel) -> f32 {
        return dive_profile_model.tissue_pressures_total[compartment] / dive_profile_model.maximum_surface_pressures[compartment] * 100.0;
    }
}