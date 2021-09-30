pub mod test_fixtures {
    use crate::models::dive_profile::dive_profile_model::DiveProfileModel;
    use crate::models::dive_step::dive_step::DiveStep;
    use crate::models::gas_mixture::gas_mixture::GasMixture;

    #[test]
    pub fn expected_dive_profile_model() -> DiveProfileModel {
        return DiveProfileModel {
            max_surface_pressures: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            compartment_load: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            tissue_pressures_nitrogen: [3.408, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            tissue_pressures_helium: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            tissue_pressures_total: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            tolerated_ambient_pressures: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            a_values: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            b_values: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            oxygen_at_pressure: 1.26,
            helium_at_pressure: 1.0,
            nitrogen_at_pressure: 4.14,
        };
    }

    #[test]
    pub fn test_fixture_dive_step() -> DiveStep {
        return DiveStep {
            depth: 50,
            time: 10,
        };
    }

    #[test]
    pub fn test_fixture_gas_mixture() -> GasMixture {
        return GasMixture {
            oxygen: 21,
            helium: 10,
            nitrogen: 69,
        };
    }
}