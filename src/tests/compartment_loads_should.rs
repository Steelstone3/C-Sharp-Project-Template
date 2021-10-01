#[cfg(test)]
mod compartment_loads_should {
    use crate::commands::compartment_loads::compartment_loads;
    use crate::tests::test_fixtures::test_fixtures::{expected_dive_profile_model, test_fixture_compartment_load_dive_profile_model};

    #[test]
    fn calculate_compartment_load() {
        //Arrange
        let expected_dive_profile = expected_dive_profile_model();
        let actual_dive_profile = test_fixture_compartment_load_dive_profile_model();

        for compartment in 0..16 {
            //Act
            //Assert
            assert_eq!(format!("{:.3}", expected_dive_profile.compartment_load[compartment]), format!("{:.3}", compartment_loads::calculate_compartment_load(compartment, actual_dive_profile)));
        }
    }
}