use std::fs::File;
use std::io::Read;

use crate::commands::files::file_names::CYLINDER_FILE_NAME;

const DIVE_STEP_FILE_NAME: &str = "dive_step.json";
const DIVE_PROFILE_FILE_NAME: &str = "dive_profile.json";
const CAN_NOT_CREATE_FILE_ERROR: &str = "Can't create file";
const CAN_NOT_READ_FILE_ERROR: &str = "Can't read file";

//TODO create_file(path: &str)
#[allow(dead_code)]
pub fn create_dive_step_file() -> File {
    File::create(DIVE_STEP_FILE_NAME).expect(CAN_NOT_CREATE_FILE_ERROR)
}

#[allow(dead_code)]
pub fn create_dive_profile_file() -> File {
    File::create(DIVE_PROFILE_FILE_NAME).expect(CAN_NOT_CREATE_FILE_ERROR)
}

#[allow(dead_code)]
pub fn create_cylinder_file() -> File {
    File::create(CYLINDER_FILE_NAME).expect(CAN_NOT_CREATE_FILE_ERROR)
}

#[allow(dead_code)]
pub fn open_the_file(path: &str) -> File {
    let f = File::open(path);

    match f {
        Ok(f) => f,
        Err(_) => {
            create_dive_profile_file();
            create_dive_step_file()
        }
    }
}

#[allow(dead_code)]
pub fn get_file_contents(file: &mut File) -> String {
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(CAN_NOT_READ_FILE_ERROR);
    contents
}