use std::io::{Write};
use crate::commands::files::error_messages::{CAN_NOT_PARSE_FILE_TO_APP_DATA_ERROR, CAN_NOT_UPDATE_FILE_ERROR, PARSE_APP_DATA_ERROR};
use crate::commands::files::file_names::DIVE_PROFILE_FILE_NAME;
use crate::controllers::file_controller::{create_dive_profile_file, get_file_contents, open_the_file};
use crate::DiveProfile;

pub fn upsert_dive_profile_file(dive_profiles: &Vec<DiveProfile>) -> std::io::Result<()> {
    let mut json_dive_profile_file = create_dive_profile_file();
    let json_dive_profile = serde_json::ser::to_string_pretty(&dive_profiles).expect(PARSE_APP_DATA_ERROR);
    write!(json_dive_profile_file, "{}", json_dive_profile).expect(CAN_NOT_UPDATE_FILE_ERROR);

    Ok(())
}

pub fn read_dive_profile_file() -> Vec<DiveProfile> {
    let mut file = open_the_file(DIVE_PROFILE_FILE_NAME);
    let contents = get_file_contents(&mut file);
    return parse_to_application_data(&contents);
}

fn parse_to_application_data(contents: &String) -> Vec<DiveProfile> {
    if contents.is_empty()
    {
        let empty_dive_profiles: Vec<DiveProfile> = Vec::new();
        return empty_dive_profiles;
    }

    return serde_json::from_str(&contents).expect(CAN_NOT_PARSE_FILE_TO_APP_DATA_ERROR);
}

#[cfg(test)]
mod dive_profile_should {
    use crate::commands::files::dive_profile::{read_dive_profile_file, upsert_dive_profile_file};
    use crate::tests::test_fixtures_dive_plan::expected_dive_profile_model;

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