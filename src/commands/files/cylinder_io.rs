use super::file_names::CYLINDER_FILE_NAME;
use crate::commands::files::error_messages::{
    CAN_NOT_PARSE_FILE_TO_APP_DATA_ERROR, CAN_NOT_UPDATE_FILE_ERROR, PARSE_APP_DATA_ERROR,
};
use crate::controllers::file_controller::{create_cylinder_file, get_file_contents, open_the_file};
use crate::models::cylinder::Cylinder;
use std::io::Write;

pub fn upsert_cylinder_file(dive_profiles: &Vec<Vec<Cylinder>>) -> std::io::Result<()> {
    let mut json_dive_profile_file = create_cylinder_file();
    let json_dive_profile =
        serde_json::ser::to_string_pretty(&dive_profiles).expect(PARSE_APP_DATA_ERROR);
    write!(json_dive_profile_file, "{}", json_dive_profile).expect(CAN_NOT_UPDATE_FILE_ERROR);

    Ok(())
}

pub fn read_cylinder_file() -> Vec<Vec<Cylinder>> {
    let mut file = open_the_file(CYLINDER_FILE_NAME);
    let contents = get_file_contents(&mut file);
    parse_to_application_data(&contents)
}

fn parse_to_application_data(contents: &String) -> Vec<Vec<Cylinder>> {
    if contents.is_empty() {
        let empty_dive_profiles: Vec<Vec<Cylinder>> = Vec::new();
        return empty_dive_profiles;
    }

    serde_json::from_str(contents).expect(CAN_NOT_PARSE_FILE_TO_APP_DATA_ERROR)
}

#[cfg(test)]
mod dive_profile_should {
    use crate::models::gas_mixture::GasMixture;

    use super::*;

    #[test]
    fn read_from_the_file_and_parse_to_the_cylinder_model() {
        let expected_cylinders = vec![
            vec![cylinder_test_fixture(), cylinder_test_fixture()],
            vec![cylinder_test_fixture(), cylinder_test_fixture()],
        ];

        upsert_cylinder_file(&expected_cylinders)
            .expect("integration test dive_profile.json file didn't upsert");
        let cylinders = read_cylinder_file();

        assert_eq!(expected_cylinders, cylinders);
    }

    fn cylinder_test_fixture() -> Cylinder {
        Cylinder::new(12, 200, GasMixture::new(21, 10), 12)
    }
}
