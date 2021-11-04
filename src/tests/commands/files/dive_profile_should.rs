#[cfg(test)]
mod dive_profile_should {
    use crate::commands::files::dive_profile::dive_profile::{read_dive_profile_file, upsert_dive_profile_file};
    use crate::tests::test_fixtures_dive_plan::test_fixtures_dive_stage::expected_dive_profile_model;

    #[test]
    fn dive_profile_file_io() {
        let dive_profiles = vec![expected_dive_profile_model(), expected_dive_profile_model()];

        upsert_dive_profile_file(&dive_profiles).expect("integration test dive_profile.json file didn't upsert");
        let actual_dive_profiles = read_dive_profile_file();

        for dive_profile in 0..2 {
            assert_eq!(dive_profiles[dive_profile].oxygen_at_pressure, actual_dive_profiles[dive_profile].oxygen_at_pressure);
            assert_eq!(dive_profiles[dive_profile].helium_at_pressure, actual_dive_profiles[dive_profile].helium_at_pressure);
            assert_eq!(dive_profiles[dive_profile].nitrogen_at_pressure, actual_dive_profiles[dive_profile].nitrogen_at_pressure);

            for compartment in 0..16 {
                assert_eq!(dive_profiles[dive_profile].tissue_pressures_nitrogen[compartment], actual_dive_profiles[dive_profile].tissue_pressures_nitrogen[compartment]);
                assert_eq!(dive_profiles[dive_profile].tissue_pressures_helium[compartment], actual_dive_profiles[dive_profile].tissue_pressures_helium[compartment]);
                assert_eq!(dive_profiles[dive_profile].tissue_pressures_total[compartment], actual_dive_profiles[dive_profile].tissue_pressures_total[compartment]);
                assert_eq!(dive_profiles[dive_profile].a_values[compartment], actual_dive_profiles[dive_profile].a_values[compartment]);
                assert_eq!(dive_profiles[dive_profile].b_values[compartment], actual_dive_profiles[dive_profile].b_values[compartment]);
                assert_eq!(dive_profiles[dive_profile].tolerated_ambient_pressures[compartment], actual_dive_profiles[dive_profile].tolerated_ambient_pressures[compartment]);
                assert_eq!(dive_profiles[dive_profile].maximum_surface_pressures[compartment], actual_dive_profiles[dive_profile].maximum_surface_pressures[compartment]);
                assert_eq!(dive_profiles[dive_profile].compartment_load[compartment], actual_dive_profiles[dive_profile].compartment_load[compartment]);
            }
        }
    }
}