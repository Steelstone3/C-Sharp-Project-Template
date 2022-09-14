use super::presenter::{confirmation, parse_numeric_value, text_prompt};
use crate::models::{
    cylinder::Cylinder, dive_step::DiveStep, gas_mixture::GasMixture,
};

pub fn welcome_message() {
    println!("Welcome to Bubbles Dive Planner Console Rust");
}

pub fn create_cylinders() -> Vec<Cylinder> {
    let mut cylinders: Vec<Cylinder> = vec![create_cylinder()];

    while confirmation("Create cylinder:") {
        cylinders.push(create_cylinder());
    }

    cylinders
}

fn create_cylinder() -> Cylinder {
    Cylinder::new(
        parse_numeric_value(text_prompt(
            "Enter cylinder volume (L):",
            "Enter a value 3 - 30",
            "12",
        )),
        parse_numeric_value(text_prompt(
            "Enter cylinder pressure (BAR):",
            "Enter a value 50 - 300",
            "200",
        )),
        GasMixture::new(
            parse_numeric_value(text_prompt(
                "Enter oxygen (%):",
                "Enter a value 5 - 100",
                "21",
            )),
            parse_numeric_value(text_prompt(
                "Enter helium (%):",
                "Enter a value 0 - 100",
                "0",
            )),
        ),
        parse_numeric_value(text_prompt(
            "Enter surface air consumption rate (L/min):",
            "Enter a value 3 - 30",
            "12",
        )),
    )
}

pub fn create_dive_step() -> DiveStep {
    DiveStep {
        depth: parse_numeric_value(text_prompt(
            "Enter depth (M):",
            "Enter a value 1 - 100",
            "1",
        )),
        time: parse_numeric_value(text_prompt(
            "Enter time (min):",
            "Enter a value 1 - 60",
            "1",
        )),
    }
}
