use crate::{DiveProfile, DiveStep};
use crate::commands::dive_stages::a_b_values::{calculate_a_value, calculate_b_value};
use crate::commands::dive_stages::ambient_pressures::calculate_ambient_pressure;
use crate::commands::dive_stages::compartment_loads::calculate_compartment_load;
use crate::commands::dive_stages::max_surface_pressures::calculate_max_surface_pressure;
use crate::commands::dive_stages::tissue_pressures::{calculate_tissue_pressure_helium, calculate_tissue_pressure_nitrogen, calculate_tissue_pressure_total};
use crate::commands::dive_stages::tolerated_ambient_pressures::calculate_tolerated_ambient_pressure;
use crate::models::dive_model::DiveModel;
use crate::models::gas_mixture::GasMixture;

pub fn run_dive_profile(mut dive_model: DiveModel, dive_step: DiveStep, gas_mixture: GasMixture) -> DiveProfile {
    dive_model.dive_profile = calculate_ambient_pressure(dive_model.dive_profile, dive_step, gas_mixture);

    for compartment in 0..dive_model.compartment_count {
        dive_model.dive_profile = update_dive_profile_model(compartment, dive_model, dive_step);
    }

    dive_model.dive_profile
}

fn update_dive_profile_model(compartment: usize, mut dive_model: DiveModel, dive_step: DiveStep) -> DiveProfile {
    dive_model.dive_profile.tissue_pressures_nitrogen[compartment] = calculate_tissue_pressure_nitrogen(compartment, dive_model, dive_step);
    dive_model.dive_profile.tissue_pressures_helium[compartment] = calculate_tissue_pressure_helium(compartment, dive_model, dive_step);
    dive_model.dive_profile.tissue_pressures_total[compartment] = calculate_tissue_pressure_total(compartment, dive_model.dive_profile);
    dive_model.dive_profile.a_values[compartment] = calculate_a_value(compartment, dive_model);
    dive_model.dive_profile.b_values[compartment] = calculate_b_value(compartment, dive_model);
    dive_model.dive_profile.tolerated_ambient_pressures[compartment] = calculate_tolerated_ambient_pressure(compartment, dive_model.dive_profile);
    dive_model.dive_profile.maximum_surface_pressures[compartment] = calculate_max_surface_pressure(compartment, dive_model.dive_profile);
    dive_model.dive_profile.compartment_load[compartment] = calculate_compartment_load(compartment, dive_model.dive_profile);

    dive_model.dive_profile
}

#[cfg(test)]
mod controllers_dive_stage_should {
    use crate::factories::zhl16_dive_model::create_zhl16_dive_model;
    use crate::tests::test_fixtures_dive_plan::{expected_dive_profile_model, test_fixture_cylinder, test_fixture_dive_step_default};

    #[test]
    fn run_dive_profile() {
        //Arrange
        let zhl16 = create_zhl16_dive_model();
        let dive_step = test_fixture_dive_step_default();
        let gas_mixture = test_fixture_cylinder().gas_mixture;
        let expected_dive_profile = expected_dive_profile_model();

        //Act
        let result = super::run_dive_profile(zhl16, dive_step, gas_mixture);

        //Assert
        assert_eq!(format!("{:.2}", expected_dive_profile.oxygen_at_pressure), format!("{:.2}", result.oxygen_at_pressure));
        assert_eq!(format!("{:.2}", expected_dive_profile.helium_at_pressure), format!("{:.2}", result.helium_at_pressure));
        assert_eq!(format!("{:.2}", expected_dive_profile.nitrogen_at_pressure), format!("{:.2}", result.nitrogen_at_pressure));

        for compartment in 0..16 {
            assert_eq!(format!("{:.1}", expected_dive_profile.tissue_pressures_nitrogen[compartment]), format!("{:.1}", result.tissue_pressures_nitrogen[compartment]));
            assert_eq!(format!("{:.3}", expected_dive_profile.tissue_pressures_helium[compartment]), format!("{:.3}", result.tissue_pressures_helium[compartment]));
            assert_eq!(format!("{:.2}", expected_dive_profile.tissue_pressures_total[compartment]), format!("{:.2}", result.tissue_pressures_total[compartment]));
            assert_eq!(format!("{:.1}", expected_dive_profile.a_values[compartment]), format!("{:.1}", result.a_values[compartment]));
            assert_eq!(format!("{:.2}", expected_dive_profile.b_values[compartment]), format!("{:.2}", result.b_values[compartment]));
            assert_eq!(format!("{:.2}", expected_dive_profile.tolerated_ambient_pressures[compartment]), format!("{:.2}", result.tolerated_ambient_pressures[compartment]));
            assert_eq!(format!("{:.2}", expected_dive_profile.maximum_surface_pressures[compartment]), format!("{:.2}", result.maximum_surface_pressures[compartment]));
            assert_eq!(format!("{:.0}", expected_dive_profile.compartment_load[compartment]), format!("{:.0}", result.compartment_load[compartment]));
        }
    }
}