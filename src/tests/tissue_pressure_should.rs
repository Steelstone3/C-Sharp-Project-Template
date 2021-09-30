#[cfg(test)]
mod tissue_pressure_should {
    use crate::commands::tissue_pressures::tissue_pressure;
    use crate::models::dive_model::dive_model::create_zhl16_dive_model;
    use crate::tests::test_fixtures::test_fixtures::{expected_dive_profile_model, test_fixture_dive_step};

    #[test]
    #[ignore]
    fn calculate_tissue_pressure_nitrogen() {
        let zhl16 = create_zhl16_dive_model();
        let expected_dive_profile_model = expected_dive_profile_model();
        let dive_step = test_fixture_dive_step();

        for compartment in 0..16 {
            assert_eq!(expected_dive_profile_model.tissue_pressures_nitrogen[compartment].to_string(), format!("{:.3}", tissue_pressure::calculate_tissue_pressure_nitrogen(compartment, zhl16, expected_dive_profile_model, dive_step)));
        }
    }

    #[test]
    #[ignore]
    fn calculate_tissue_pressure_helium() {}

    #[test]
    #[ignore]
    fn calculate_tissue_pressure_total() {}
}