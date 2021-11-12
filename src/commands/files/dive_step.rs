pub mod dive_step {
    use std::fs::File;
    use std::io::{Read, Write};
    use crate::commands::files::error_messages::error_messages::CAN_NOT_PARSE_FILE_TO_APP_DATA_ERROR;
    use crate::commands::files::file_names::file_names::DIVE_STEP_FILE_NAME;
    use crate::controllers::file_controller::file_factory::{create_dive_step_file, get_file_contents, open_the_file};
    use crate::models::dive_step::dive_step::DiveStep;

    pub fn upsert_dive_step_file(dive_steps: &Vec<DiveStep>) -> std::io::Result<()> {
        let mut json_dive_step_file = create_dive_step_file();
        let json_dive_step = serde_json::ser::to_string_pretty(&dive_steps).expect("Can't parse dive_steps to string");
        write!(json_dive_step_file, "{}", json_dive_step).expect("Can't update dive_step.json file");
        Ok(())
    }

    pub fn read_dive_step_file() -> Vec<DiveStep> {
        let mut file = open_the_file(DIVE_STEP_FILE_NAME);
        let contents = get_file_contents(&mut file);
        return parse_to_application_data(&contents);
    }

    fn parse_to_application_data(contents: &String) -> Vec<DiveStep> {
        if contents.is_empty()
        {
            let empty_dive_steps: Vec<DiveStep> = Vec::new();
            return empty_dive_steps;
        }

        return serde_json::from_str(&contents).expect(CAN_NOT_PARSE_FILE_TO_APP_DATA_ERROR );
    }
}