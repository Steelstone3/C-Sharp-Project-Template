use crate::DiveProfile;

pub fn calculate_compartment_load(compartment: usize, dive_profile_model: DiveProfile) -> f32 {
    dive_profile_model.tissue_pressures_total[compartment] / dive_profile_model.maximum_surface_pressures[compartment] * 100.0
}

#[cfg(test)]
mod commands_compartment_loads_should {
    use crate::tests::test_fixtures_dive_plan::{expected_dive_profile_model, test_fixture_compartment_load_dive_profile_model};

    #[test]
    fn calculate_compartment_load() {
        //Arrange
        let expected_dive_profile = expected_dive_profile_model();
        let actual_dive_profile = test_fixture_compartment_load_dive_profile_model();

        for compartment in 0..16 {
            //Act
            //Assert
            assert_eq!(format!("{:.3}", expected_dive_profile.compartment_load[compartment]), format!("{:.3}", super::calculate_compartment_load(compartment, actual_dive_profile)));
        }
    }
}