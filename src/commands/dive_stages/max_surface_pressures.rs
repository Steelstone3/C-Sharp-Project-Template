pub mod max_surface_pressures {
    use crate::models::dive_profile::dive_profile_model::DiveProfile;

    pub fn calculate_max_surface_pressure(compartment: usize, dive_profile_model: DiveProfile) -> f32 {
        return (1.0 / dive_profile_model.b_values[compartment]) + dive_profile_model.a_values[compartment];
    }
}