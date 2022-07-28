use crate::DiveProfile;

pub fn calculate_tolerated_ambient_pressure(compartment: usize, dive_profile_model: DiveProfile) -> f32 {
    return (dive_profile_model.tissue_pressures_total[compartment] - dive_profile_model.a_values[compartment]) * dive_profile_model.b_values[compartment];
}

#[cfg(test)]
mod commands_tolerated_ambient_pressures_should {
    use crate::tests::test_fixtures_dive_plan::{expected_dive_profile_model, test_fixture_tolerated_ambient_pressures_dive_profile_model};

    #[test]
    fn calculate_tolerated_ambient_pressure() {
        //Arrange
        let expected_dive_profile_model = expected_dive_profile_model();
        let actual_dive_profile_model = test_fixture_tolerated_ambient_pressures_dive_profile_model();

        for compartment in 0..16 {
            //Act
            //Assert
            assert_eq!(format!("{:.3}", expected_dive_profile_model.tolerated_ambient_pressures[compartment]), format!("{:.3}", super::calculate_tolerated_ambient_pressure(compartment, actual_dive_profile_model)));
        }
    }
}