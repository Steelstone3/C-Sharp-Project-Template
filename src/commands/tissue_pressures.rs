use crate::{DiveProfile, DiveStep};
use crate::models::dive_model::DiveModel;

pub fn calculate_tissue_pressure_nitrogen(compartment: usize, dive_model: DiveModel, dive_step: DiveStep) -> f32 {
    dive_model.dive_profile.tissue_pressures_nitrogen[compartment] + ((dive_model.dive_profile.nitrogen_at_pressure - dive_model.dive_profile.tissue_pressures_nitrogen[compartment]) * (1.0 - f32::powf(2.0, -(dive_step.time as f32 / dive_model.nitrogen_half_time[compartment]))))
}

pub fn calculate_tissue_pressure_helium(compartment: usize, dive_model: DiveModel, dive_step: DiveStep) -> f32 {
    dive_model.dive_profile.tissue_pressures_helium[compartment] + ((dive_model.dive_profile.helium_at_pressure - dive_model.dive_profile.tissue_pressures_helium[compartment]) * (1.0 - f32::powf(2.0, -(dive_step.time as f32 / dive_model.helium_half_time[compartment]))))
}

pub fn calculate_tissue_pressure_total(compartment: usize, dive_profile: DiveProfile) -> f32 {
    dive_profile.tissue_pressures_helium[compartment] + dive_profile.tissue_pressures_nitrogen[compartment]
}

#[cfg(test)]
mod commands_tissue_pressure_should {
    use crate::{tests::test_fixtures_dive_plan::{expected_dive_profile_model, test_fixture_dive_step_default, test_fixture_tissue_pressures_dive_profile_model, test_fixture_tissue_pressures_total_dive_profile_model}, models::dive_model::DiveModel};

    #[test]
    fn calculate_tissue_pressure_nitrogen() {
        //Arrange
        let mut zhl16 = DiveModel::create_zhl16_dive_model();
        let dive_step = test_fixture_dive_step_default();
        let expected_dive_profile_model = expected_dive_profile_model();
        zhl16.dive_profile = test_fixture_tissue_pressures_dive_profile_model();

        for compartment in 0..16 {
            //Act
            //Assert
            assert_eq!(format!("{:.3}", expected_dive_profile_model.tissue_pressures_nitrogen[compartment]), format!("{:.3}", super::calculate_tissue_pressure_nitrogen(compartment, zhl16, dive_step)));
        }
    }

    #[test]
    fn calculate_tissue_pressure_helium() {
        //Arrange
        let mut zhl16 = DiveModel::create_zhl16_dive_model();
        let dive_step = test_fixture_dive_step_default();
        let expected_dive_profile_model = expected_dive_profile_model();
        zhl16.dive_profile = test_fixture_tissue_pressures_dive_profile_model();

        for compartment in 0..16 {
            //Act
            //Assert
            assert_eq!(format!("{:.3}", expected_dive_profile_model.tissue_pressures_helium[compartment]), format!("{:.3}", super::calculate_tissue_pressure_helium(compartment, zhl16, dive_step)));
        }
    }

    #[test]
    fn calculate_tissue_pressure_total() {
        //Arrange
        let expected_dive_profile_model = expected_dive_profile_model();
        let actual_dive_profile = test_fixture_tissue_pressures_total_dive_profile_model();

        for compartment in 0..16 {
            //Act
            //Assert
            assert_eq!(format!("{:.3}", expected_dive_profile_model.tissue_pressures_total[compartment]), format!("{:.3}", super::calculate_tissue_pressure_total(compartment, actual_dive_profile)));
        }
    }
}

