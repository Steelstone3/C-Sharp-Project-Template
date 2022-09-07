use crate::{DiveProfile, DiveStep};
use crate::models::gas_mixture::GasMixture;

pub fn calculate_ambient_pressure(mut dive_profile_model: DiveProfile, dive_step: DiveStep, gas_mixture: GasMixture) -> DiveProfile {
    let ambient_pressure = 1.0 + dive_step.depth as f32 / 10.0;
    dive_profile_model.nitrogen_at_pressure = gas_mixture.nitrogen as f32 / 100.0 * ambient_pressure;
    dive_profile_model.oxygen_at_pressure = gas_mixture.oxygen as f32 / 100.0 * ambient_pressure;
    dive_profile_model.helium_at_pressure = gas_mixture.helium as f32 / 100.0 * ambient_pressure;

    dive_profile_model
}

#[cfg(test)]
mod commands_ambient_pressures_should {
    use crate::tests::test_fixtures_dive_plan::{expected_dive_profile_model, test_fixture_cylinder, test_fixture_dive_step_default};

    #[test]
    fn calculate_ambient_pressure() {
        let expected_dive_profile_model = expected_dive_profile_model();
        let dive_step = test_fixture_dive_step_default();
        let gas_mixture = test_fixture_cylinder().gas_mixture;

        let actual_dive_profile_model = super::calculate_ambient_pressure(expected_dive_profile_model, dive_step, gas_mixture);
        assert_eq!(format!("{:.3}", expected_dive_profile_model.oxygen_at_pressure), format!("{:.3}", actual_dive_profile_model.oxygen_at_pressure));
        assert_eq!(format!("{:.3}", expected_dive_profile_model.nitrogen_at_pressure), format!("{:.3}", actual_dive_profile_model.nitrogen_at_pressure));
        assert_eq!(format!("{:.3}", expected_dive_profile_model.helium_at_pressure), format!("{:.3}", actual_dive_profile_model.helium_at_pressure));
    }
}