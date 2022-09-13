use inquire::{Text, Confirm, Select};

use crate::models::{dive_model::DiveModel, cylinder::Cylinder};

pub fn select_dive_model() -> DiveModel {
    Select::new(
        "Select dive model:",
        vec![DiveModel::create_zhl16_dive_model(), DiveModel::create_zhl16_dive_model()],
    )
    .prompt()
    .unwrap()
}

pub fn select_cylinder(cylinders: Vec<Cylinder>) -> Cylinder {
    Select::new(
        "Select cylinder:",
        cylinders,
    )
    .prompt()
    .unwrap()
}

pub fn text_prompt(message:&str, help_prompt:&str, default_value:&str) -> String  {
    Text::new(message)
            .with_help_message(help_prompt)
            .with_default(default_value)
            .prompt()
            .unwrap()
}

pub fn confirmation(message:&str) -> bool{
    Confirm::new(message).with_default(false).prompt().unwrap_or_default()
}

pub fn parse_numeric_value(input: String) -> u32 {
    match input.chars().find(|character| character.is_numeric()) {
        Some(_) => input.as_str().trim().parse::<u32>().unwrap(),
        None => panic!("Not a numeric value"),
    }
}
