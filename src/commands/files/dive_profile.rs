pub mod dive_profile {
    use std::io::{Write};
    use crate::commands::files::error_messages::error_messages::{CAN_NOT_PARSE_FILE_TO_APP_DATA_ERROR, CAN_NOT_UPDATE_FILE_ERROR, PARSE_APP_DATA_ERROR};
    use crate::commands::files::file_names::file_names::DIVE_PROFILE_FILE_NAME;
    use crate::controllers::file_controller::file_controller::{create_dive_profile_file, get_file_contents, open_the_file};
    use crate::models::dive_profile::dive_profile_model::DiveProfile;

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
}