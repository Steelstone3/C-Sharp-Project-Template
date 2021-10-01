#[cfg(test)]
mod tolerated_ambient_pressures_should {
    use crate::commands::tolerated_ambient_pressures::tolerated_ambient_pressures;
    use crate::tests::test_fixtures::test_fixtures::{expected_dive_profile_model, test_fixture_tolerated_ambient_pressures_dive_profile_model};

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