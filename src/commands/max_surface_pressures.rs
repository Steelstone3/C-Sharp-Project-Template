use crate::DiveProfile;

pub fn calculate_max_surface_pressure(compartment: usize, dive_profile_model: DiveProfile) -> f32 {
    (1.0 / dive_profile_model.b_values[compartment]) + dive_profile_model.a_values[compartment]
}

#[cfg(test)]
mod commands_max_surface_pressures_should {
    use crate::tests::test_fixtures_dive_plan::{expected_dive_profile_model, test_fixture_max_surface_pressure_dive_profile_model};

    #[test]
    fn calculate_max_surface_pressure() {
        //Arrange
        let expected_dive_profile = expected_dive_profile_model();
        let actual_dive_profile = test_fixture_max_surface_pressure_dive_profile_model();

        for compartment in 0..16 {
            //Act
            //Assert
            assert_eq!(format!("{:.3}", expected_dive_profile.maximum_surface_pressures[compartment]), format!("{:.3}", super::calculate_max_surface_pressure(compartment, actual_dive_profile)));
        }
    }
}