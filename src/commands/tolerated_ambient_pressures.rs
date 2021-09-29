pub mod tolerated_ambient_pressures {
    use crate::models::dive_profile::dive_profile_model::DiveProfileModel;

    pub fn calculate_tolerated_ambient_pressure(compartment:usize, dive_profile_model:DiveProfileModel) -> f32 {
        return (dive_profile_model.tissue_pressures_total[compartment] - dive_profile_model.a_values[compartment]) * dive_profile_model.b_values[compartment];
    }
}

#[cfg(test)]
mod tolerated_ambient_pressures_should {
    #[test]
    #[ignore]
    fn calculate_tolerated_ambient_pressure() {}
}