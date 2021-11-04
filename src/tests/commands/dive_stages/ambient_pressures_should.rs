#[cfg(test)]
mod commands_ambient_pressures_should {
    use crate::commands::dive_stages::ambient_pressures::ambient_pressures;
    use crate::tests::test_fixtures_dive_plan::test_fixtures_dive_stage::{expected_dive_profile_model, test_fixture_cylinder, test_fixture_dive_step_default};

    #[test]
    fn calculate_ambient_pressure() {
        let expected_dive_profile_model = expected_dive_profile_model();
        let dive_step = test_fixture_dive_step_default();
        let gas_mixture = test_fixture_cylinder().gas_mixture;

        let actual_dive_profile_model = ambient_pressures::calculate_ambient_pressure(expected_dive_profile_model, dive_step, gas_mixture);
        assert_eq!(format!("{:.3}", expected_dive_profile_model.oxygen_at_pressure), format!("{:.3}", actual_dive_profile_model.oxygen_at_pressure));
        assert_eq!(format!("{:.3}", expected_dive_profile_model.nitrogen_at_pressure), format!("{:.3}", actual_dive_profile_model.nitrogen_at_pressure));
        assert_eq!(format!("{:.3}", expected_dive_profile_model.helium_at_pressure), format!("{:.3}", actual_dive_profile_model.helium_at_pressure));
    }
}