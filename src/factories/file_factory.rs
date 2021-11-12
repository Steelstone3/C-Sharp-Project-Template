pub mod file_factory {
    use std::fs::File;

    pub fn create_dive_step_file() -> File {
        File::create("dive_step.json").expect("Can't create dive_step.json file")
    }

    pub fn create_dive_profile_file() -> File {
        File::create("dive_profile.json").expect("Can't create dive_profile.json file")
    }
}


