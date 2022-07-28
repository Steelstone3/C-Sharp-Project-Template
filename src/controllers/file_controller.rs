use std::fs::File;
use std::io::Read;
use crate::commands::files::error_messages::{CAN_NOT_CREATE_FILE_ERROR, CAN_NOT_READ_FILE_ERROR};
use crate::commands::files::file_names::{DIVE_PROFILE_FILE_NAME, DIVE_STEP_FILE_NAME};

pub fn create_dive_step_file() -> File {
    File::create(DIVE_STEP_FILE_NAME).expect(CAN_NOT_CREATE_FILE_ERROR)
}

pub fn create_dive_profile_file() -> File {
    File::create(DIVE_PROFILE_FILE_NAME).expect(CAN_NOT_CREATE_FILE_ERROR)
}

pub fn open_the_file(path: &str) -> File {
    let f = File::open(path);

    let file = match f {
        Ok(f) => f,
        Err(_) => {
            create_dive_profile_file();
            create_dive_step_file()
        }
    };
    file
}

pub fn get_file_contents(file: &mut File) -> String {
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(CAN_NOT_READ_FILE_ERROR);
    contents
}