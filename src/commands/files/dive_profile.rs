pub mod dive_profile {
    use std::fs::File;
    use std::io::{Read, Write};
    use crate::factories::file_factory::file_factory::create_dive_profile_file;
    use crate::models::dive_profile::dive_profile_model::DiveProfile;

    pub fn upsert_dive_profile_file(dive_profiles: &Vec<DiveProfile>) -> std::io::Result<()> {
        let mut json_dive_profile_file = create_dive_profile_file();
        let json_dive_profile = serde_json::ser::to_string_pretty(&dive_profiles).expect("Can't convert dive_profiles to string");
        write!(json_dive_profile_file, "{}", json_dive_profile).expect("Can't update dive_profile.json file");

        Ok(())
    }

    pub fn read_dive_profile_file() -> Vec<DiveProfile> {
        let mut file = open_the_file();
        let contents = get_file_contents(&mut file);
        return parse_to_application_data(&contents);
    }

    fn open_the_file() -> File {
        let f = File::open("dive_profile.json");

        let file = match f {
            Ok(f) => f,
            Err(_) => create_dive_profile_file()
        };
        file
    }

    fn get_file_contents(file: &mut File) -> String {
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Can't read dive_profile.json file.");
        contents
    }

    fn parse_to_application_data(contents: &String) -> Vec<DiveProfile> {
        if contents.is_empty()
        {
            let empty_dive_profiles: Vec<DiveProfile> = Vec::new();
            return empty_dive_profiles;
        }

        return serde_json::from_str(&contents).expect("Can't parse dive_profile.json contents to application data");
    }
}