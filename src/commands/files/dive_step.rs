pub mod dive_step {
    use std::fs::File;
    use std::io::{Read, Write};

    use crate::factories::file_factory::file_factory::create_dive_step_file;
    use crate::models::dive_step::dive_step::DiveStep;

    pub fn upsert_dive_step_file(dive_steps: &Vec<DiveStep>) -> std::io::Result<()> {
        let mut json_dive_step_file = create_dive_step_file();
        let json_dive_step = serde_json::ser::to_string_pretty(&dive_steps).expect("Can't convert dive_steps to string");
        write!(json_dive_step_file, "{}", json_dive_step).expect("Can't update dive_step.json file");
        Ok(())
    }

    pub fn read_dive_step_file() -> Vec<DiveStep> {
        let mut file = open_the_file();
        let contents = get_file_contents(&mut file);
        return parse_to_application_data(&contents);
    }

    fn open_the_file() -> File {
        let f = File::open("dive_step.json");

        let file = match f {
            Ok(f) => f,
            Err(_) => create_dive_step_file()
        };
        file
    }

    fn get_file_contents(file: &mut File) -> String {
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Can't read dive_step.json file.");
        contents
    }

    fn parse_to_application_data(contents: &String) -> Vec<DiveStep> {
        if contents.is_empty()
        {
            let empty_dive_steps: Vec<DiveStep> = Vec::new();
            return empty_dive_steps;
        }

        return serde_json::from_str(&contents).expect("Can't parse dive_step.json contents to application data");
    }
}