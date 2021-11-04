#[cfg(test)]
mod commands_tolerated_ambient_pressures_should {
    use crate::commands::dive_stages::tolerated_ambient_pressures::tolerated_ambient_pressures;
    use crate::tests::test_fixtures_dive_plan::test_fixtures_dive_stage::{expected_dive_profile_model, test_fixture_tolerated_ambient_pressures_dive_profile_model};

    #[test]
    fn calculate_tolerated_ambient_pressure() {
        //Arrange
        let expected_dive_profile_model = expected_dive_profile_model();
        let actual_dive_profile_model = test_fixture_tolerated_ambient_pressures_dive_profile_model();

        for compartment in 0..16 {
            //Act
            //Assert
            assert_eq!(format!("{:.3}", expected_dive_profile_model.tolerated_ambient_pressures[compartment]), format!("{:.3}", tolerated_ambient_pressures::calculate_tolerated_ambient_pressure(compartment, actual_dive_profile_model)));
        }
    }
}