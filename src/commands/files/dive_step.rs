use std::io::{Write};
use crate::commands::files::error_messages::{CAN_NOT_PARSE_FILE_TO_APP_DATA_ERROR, CAN_NOT_UPDATE_FILE_ERROR, PARSE_APP_DATA_ERROR};
use crate::commands::files::file_names::DIVE_STEP_FILE_NAME;
use crate::controllers::file_controller::{create_dive_step_file, get_file_contents, open_the_file};
use crate::DiveStep;

pub fn upsert_dive_step_file(dive_steps: &Vec<DiveStep>) -> std::io::Result<()> {
    let mut json_dive_step_file = create_dive_step_file();
    let json_dive_step = serde_json::ser::to_string_pretty(&dive_steps).expect(PARSE_APP_DATA_ERROR);
    write!(json_dive_step_file, "{}", json_dive_step).expect(CAN_NOT_UPDATE_FILE_ERROR);
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

    return serde_json::from_str(&contents).expect(CAN_NOT_PARSE_FILE_TO_APP_DATA_ERROR);
}

#[cfg(test)]
mod dive_step_should {
    use crate::commands::files::dive_step::{read_dive_step_file, upsert_dive_step_file};
    use crate::tests::test_fixtures_dive_plan::{test_fixture_dive_step_alternative, test_fixture_dive_step_default};

    #[test]
    fn dive_step_file_io() {
        let dive_steps = vec![test_fixture_dive_step_default(), test_fixture_dive_step_alternative()];

        upsert_dive_step_file(&dive_steps).expect("integration test dive_step.json file didn't upsert");
        let actual_dive_steps = read_dive_step_file();

        assert_eq!(dive_steps[0].depth, actual_dive_steps[0].depth);
        assert_eq!(dive_steps[0].time, actual_dive_steps[0].time);
        assert_eq!(dive_steps[1].depth, actual_dive_steps[1].depth);
        assert_eq!(dive_steps[1].time, actual_dive_steps[1].time);
    }
}