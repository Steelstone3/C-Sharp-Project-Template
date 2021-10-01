#[cfg(test)]
mod a_b_values_should {
    use crate::commands::a_b_values::a_b_values;
    use crate::models::dive_model::dive_model::create_zhl16_dive_model;
    use crate::tests::test_fixtures::test_fixtures::{expected_dive_profile_model, test_fixture_a_b_values_dive_profile_model};

    #[test]
    fn calculate_a_value() {
        //Arrange
        let zhl16 = create_zhl16_dive_model();
        let expected_dive_profile_model = expected_dive_profile_model();
        let actual_dive_profile_model = test_fixture_a_b_values_dive_profile_model();

        for compartment in 0..16 {
            //Act
            //Assert
            assert_eq!(format!("{:.3}", expected_dive_profile_model.a_values[compartment]), format!("{:.3}", a_b_values::calculate_a_value(compartment, zhl16, actual_dive_profile_model)));
        }
    }

    #[test]
    fn calculate_b_value() {
        //Arrange
        let zhl16 = create_zhl16_dive_model();
        let expected_dive_profile_model = expected_dive_profile_model();
        let actual_dive_profile_model = test_fixture_a_b_values_dive_profile_model();

        for compartment in 0..16 {
            //Act
            //Assert
            assert_eq!(format!("{:.3}", expected_dive_profile_model.b_values[compartment]), format!("{:.3}", a_b_values::calculate_b_value(compartment, zhl16, actual_dive_profile_model)));
        }
    }
}