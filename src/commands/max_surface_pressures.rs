pub mod max_surface_pressures {
    use crate::models::dive_profile::dive_profile_model::DiveProfileModel;

    pub fn calculate_max_surface_pressure(compartment: usize, dive_profile_model: DiveProfileModel) -> f32 {
        return (1.0 / dive_profile_model.b_values[compartment]) + dive_profile_model.a_values[compartment];
    }
}

#[cfg(test)]
mod max_surface_pressures_should {
    #[test]
    #[ignore]
    fn calculate_max_surface_pressure() {}
}