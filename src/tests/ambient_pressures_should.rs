#[cfg(test)]
mod ambient_pressures_should {
    use crate::commands::ambient_pressures::ambient_pressures;
    use crate::tests::test_fixtures::test_fixtures::{expected_dive_profile_model, test_fixture_dive_step, test_fixture_gas_mixture};

    #[test]
    fn calculate_ambient_pressure() {
        let expected_dive_profile_model = expected_dive_profile_model();
        let dive_step = test_fixture_dive_step();
        let gas_mixture = test_fixture_gas_mixture();

        let actual_dive_profile_model = ambient_pressures::calculate_ambient_pressure(expected_dive_profile_model, dive_step, gas_mixture);
        assert_eq!(expected_dive_profile_model.oxygen_at_pressure, actual_dive_profile_model.oxygen_at_pressure);
        assert_eq!(expected_dive_profile_model.nitrogen_at_pressure, actual_dive_profile_model.nitrogen_at_pressure);
        assert_eq!(expected_dive_profile_model.helium_at_pressure, actual_dive_profile_model.helium_at_pressure.round());
    }
}