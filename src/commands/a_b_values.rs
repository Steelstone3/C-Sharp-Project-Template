use crate::models::dive_model::DiveModel;

pub fn calculate_a_value(compartment: usize, dive_model: DiveModel) -> f32 {
    (dive_model.a_values_nitrogen[compartment]
        * dive_model.dive_profile.tissue_pressures_nitrogen[compartment]
        + dive_model.a_values_helium[compartment]
            * dive_model.dive_profile.tissue_pressures_helium[compartment])
        / dive_model.dive_profile.tissue_pressures_total[compartment]
}

pub fn calculate_b_value(compartment: usize, dive_model: DiveModel) -> f32 {
    (dive_model.b_values_nitrogen[compartment]
        * dive_model.dive_profile.tissue_pressures_nitrogen[compartment]
        + dive_model.b_values_helium[compartment]
            * dive_model.dive_profile.tissue_pressures_helium[compartment])
        / dive_model.dive_profile.tissue_pressures_total[compartment]
}

#[cfg(test)]
mod commands_a_b_values_should {
    use crate::models::dive_model::DiveModel;
    use crate::tests::test_fixtures_dive_plan::{
        expected_dive_profile_model, test_fixture_a_b_values_dive_profile_model,
    };

    #[test]
    fn calculate_a_value() {
        //Arrange
        let mut zhl16 = DiveModel::create_zhl16_dive_model();
        let expected_dive_profile_model = expected_dive_profile_model();
        zhl16.dive_profile = test_fixture_a_b_values_dive_profile_model();

        for compartment in 0..16 {
            //Act
            //Assert
            assert_eq!(
                format!("{:.3}", expected_dive_profile_model.a_values[compartment]),
                format!("{:.3}", super::calculate_a_value(compartment, zhl16))
            );
        }
    }

    #[test]
    fn calculate_b_value() {
        //Arrange
        let mut zhl16 = DiveModel::create_zhl16_dive_model();
        let expected_dive_profile_model = expected_dive_profile_model();
        zhl16.dive_profile = test_fixture_a_b_values_dive_profile_model();

        for compartment in 0..16 {
            //Act
            //Assert
            assert_eq!(
                format!("{:.3}", expected_dive_profile_model.b_values[compartment]),
                format!("{:.3}", super::calculate_b_value(compartment, zhl16))
            );
        }
    }
}
