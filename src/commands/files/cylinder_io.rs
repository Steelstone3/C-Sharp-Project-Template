use super::file_names::CYLINDER_FILE_NAME;
use crate::commands::files::error_messages::{
    CAN_NOT_PARSE_FILE_TO_APP_DATA_ERROR, CAN_NOT_UPDATE_FILE_ERROR, PARSE_APP_DATA_ERROR,
};
use crate::controllers::file_controller::{create_cylinder_file, get_file_contents, open_the_file};
use crate::models::cylinder::Cylinder;
use std::io::Write;

#[allow(dead_code)]
pub fn upsert_cylinder_file(dive_profiles: &Vec<Vec<Cylinder>>) -> std::io::Result<()> {
    let mut json_dive_profile_file = create_cylinder_file();
    let json_dive_profile =
        serde_json::ser::to_string_pretty(&dive_profiles).expect(PARSE_APP_DATA_ERROR);
    write!(json_dive_profile_file, "{}", json_dive_profile).expect(CAN_NOT_UPDATE_FILE_ERROR);

    Ok(())
}

#[allow(dead_code)]
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
    // #[ignore = "Writing test"]
    fn read_from_the_file_and_parse_to_the_cylinder_model() {
        let cylinders = vec![
            vec![cylinder_test_fixture(), cylinder_test_fixture()],
            vec![cylinder_test_fixture(), cylinder_test_fixture()],
        ];

        upsert_cylinder_file(&cylinders)
            .expect("integration test dive_profile.json file didn't upsert");
        let cylinders = read_cylinder_file();

        assert_eq!(12, cylinders[0][0].cylinder_volume);
        assert_eq!(200, cylinders[0][0].cylinder_pressure);
        assert_eq!(2400, cylinders[0][0].initial_pressurised_cylinder_volume);
        assert_eq!(21, cylinders[0][0].gas_mixture.oxygen);
        assert_eq!(10, cylinders[0][0].gas_mixture.helium);
        assert_eq!(69, cylinders[0][0].gas_mixture.nitrogen);
        assert_eq!(2400, cylinders[0][0].gas_management.gas_remaining);
        assert_eq!(0, cylinders[0][0].gas_management.gas_used);
        assert_eq!(
            12,
            cylinders[0][0].gas_management.surface_air_consumption_rate
        );

        assert_eq!(12, cylinders[0][1].cylinder_volume);
        assert_eq!(200, cylinders[0][1].cylinder_pressure);
        assert_eq!(2400, cylinders[0][1].initial_pressurised_cylinder_volume);
        assert_eq!(21, cylinders[0][1].gas_mixture.oxygen);
        assert_eq!(10, cylinders[0][1].gas_mixture.helium);
        assert_eq!(69, cylinders[0][1].gas_mixture.nitrogen);
        assert_eq!(2400, cylinders[0][1].gas_management.gas_remaining);
        assert_eq!(0, cylinders[0][1].gas_management.gas_used);
        assert_eq!(
            12,
            cylinders[0][1].gas_management.surface_air_consumption_rate
        );

        assert_eq!(12, cylinders[1][0].cylinder_volume);
        assert_eq!(200, cylinders[1][0].cylinder_pressure);
        assert_eq!(2400, cylinders[1][0].initial_pressurised_cylinder_volume);
        assert_eq!(21, cylinders[1][0].gas_mixture.oxygen);
        assert_eq!(10, cylinders[1][0].gas_mixture.helium);
        assert_eq!(69, cylinders[1][0].gas_mixture.nitrogen);
        assert_eq!(2400, cylinders[1][0].gas_management.gas_remaining);
        assert_eq!(0, cylinders[1][0].gas_management.gas_used);
        assert_eq!(
            12,
            cylinders[1][0].gas_management.surface_air_consumption_rate
        );

        assert_eq!(12, cylinders[1][1].cylinder_volume);
        assert_eq!(200, cylinders[1][1].cylinder_pressure);
        assert_eq!(2400, cylinders[1][1].initial_pressurised_cylinder_volume);
        assert_eq!(21, cylinders[1][1].gas_mixture.oxygen);
        assert_eq!(10, cylinders[1][1].gas_mixture.helium);
        assert_eq!(69, cylinders[1][1].gas_mixture.nitrogen);
        assert_eq!(2400, cylinders[1][1].gas_management.gas_remaining);
        assert_eq!(0, cylinders[1][1].gas_management.gas_used);
        assert_eq!(
            12,
            cylinders[1][1].gas_management.surface_air_consumption_rate
        );
    }

    fn cylinder_test_fixture() -> Cylinder {
        Cylinder::new(12, 200, GasMixture::new(21, 10), 12)
    }
}
