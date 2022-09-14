use crate::commands::files::error_messages::{
    CAN_NOT_PARSE_FILE_TO_APP_DATA_ERROR, CAN_NOT_UPDATE_FILE_ERROR, PARSE_APP_DATA_ERROR,
};
use crate::commands::files::file_names::DIVE_STEP_FILE_NAME;
use crate::controllers::file_controller::{
    create_dive_step_file, get_file_contents, open_the_file,
};
use crate::models::dive_step::DiveStep;
use std::io::Write;

pub fn upsert_dive_step_file(dive_steps: &Vec<DiveStep>) -> std::io::Result<()> {
    let mut json_dive_step_file = create_dive_step_file();
    let json_dive_step =
        serde_json::ser::to_string_pretty(&dive_steps).expect(PARSE_APP_DATA_ERROR);
    write!(json_dive_step_file, "{}", json_dive_step).expect(CAN_NOT_UPDATE_FILE_ERROR);
    Ok(())
}

#[allow(dead_code)]
pub fn read_dive_step_file() -> Vec<DiveStep> {
    let mut file = open_the_file(DIVE_STEP_FILE_NAME);
    let contents = get_file_contents(&mut file);
    parse_to_application_data(&contents)
}

fn parse_to_application_data(contents: &String) -> Vec<DiveStep> {
    if contents.is_empty() {
        let empty_dive_steps: Vec<DiveStep> = Vec::new();
        return empty_dive_steps;
    }

    serde_json::from_str(contents).expect(CAN_NOT_PARSE_FILE_TO_APP_DATA_ERROR)
}

#[cfg(test)]
mod dive_step_should {
    use super::*;
    
    #[test]
    fn read_from_the_file_and_parse_to_the_dive_step_model() {
        let expected_dive_steps = dive_step_test_fixture();

        upsert_dive_step_file(&expected_dive_steps)
            .expect("integration test dive_step.json file didn't upsert");
        let dive_steps = read_dive_step_file();

        assert_eq!(expected_dive_steps, dive_steps);
    }

    fn dive_step_test_fixture() -> Vec<DiveStep> {
        vec![
            DiveStep {
                depth: 50,
                time: 10,
            },
            DiveStep {
                depth: 25,
                time: 20
            },
        ]
    }
}
