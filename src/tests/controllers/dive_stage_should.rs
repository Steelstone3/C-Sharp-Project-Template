#[cfg(test)]
mod controllers_dive_stage_should {
    use crate::controllers::dive_stage::dive_stage;
    use crate::factories::zhl16_dive_model::zhl16_dive_model::create_zhl16_dive_model;
    use crate::tests::test_fixtures_dive_stage::test_fixtures_dive_stage::{expected_dive_profile_model, test_fixture_cylinder, test_fixture_dive_step};

    #[test]
    fn run_dive_profile() {
        //Arrange
        let zhl16 = create_zhl16_dive_model();
        let dive_step = test_fixture_dive_step();
        let gas_mixture = test_fixture_cylinder().gas_mixture;
        let expected_dive_profile = expected_dive_profile_model();

        let result = dive_stage::run_dive_profile(zhl16, dive_step, gas_mixture);

        assert_eq!(format!("{:.2}", expected_dive_profile.oxygen_at_pressure), format!("{:.2}", result.oxygen_at_pressure));
        assert_eq!(format!("{:.2}", expected_dive_profile.helium_at_pressure), format!("{:.2}", result.helium_at_pressure));
        assert_eq!(format!("{:.2}", expected_dive_profile.nitrogen_at_pressure), format!("{:.2}", result.nitrogen_at_pressure));

        for compartment in 0..16 {
            assert_eq!(format!("{:.1}", expected_dive_profile.tissue_pressures_nitrogen[compartment]), format!("{:.1}", result.tissue_pressures_nitrogen[compartment]));
            assert_eq!(format!("{:.3}", expected_dive_profile.tissue_pressures_helium[compartment]), format!("{:.3}", result.tissue_pressures_helium[compartment]));
            assert_eq!(format!("{:.2}",expected_dive_profile.tissue_pressures_total[compartment]), format!("{:.2}",result.tissue_pressures_total[compartment]));
            assert_eq!(format!("{:.1}",expected_dive_profile.a_values[compartment]), format!("{:.1}",result.a_values[compartment]));
            assert_eq!(format!("{:.2}",expected_dive_profile.b_values[compartment]), format!("{:.2}",result.b_values[compartment]));
            assert_eq!(format!("{:.2}",expected_dive_profile.tolerated_ambient_pressures[compartment]), format!("{:.2}",result.tolerated_ambient_pressures[compartment]));
            assert_eq!(format!("{:.2}",expected_dive_profile.maximum_surface_pressures[compartment]), format!("{:.2}",result.maximum_surface_pressures[compartment]));
            assert_eq!(format!("{:.0}",expected_dive_profile.compartment_load[compartment]), format!("{:.0}",result.compartment_load[compartment]));
        }
    }
}