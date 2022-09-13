use super::presenter::{confirmation, parse_numeric_value, text_prompt};
use crate::models::{cylinder::Cylinder, gas_management::GasManagement, gas_mixture::GasMixture};

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
    let cylinder_volume = parse_numeric_value(text_prompt(
        "Enter cylinder volume (L):",
        "Enter a value 3 - 30",
        "12",
    ));
    let cylinder_pressure = parse_numeric_value(text_prompt(
        "Enter cylinder pressure (BAR):",
        "Enter a value 50 - 300",
        "200",
    ));

    let gas_mixture = GasMixture::new(
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
    );

    let gas_management = GasManagement::new(
        Cylinder::calculate_initial_pressurised_cylinder_volume(cylinder_volume, cylinder_pressure),
        parse_numeric_value(text_prompt(
            "Enter surface air consumption rate (L/min):",
            "Enter a value 3 - 30",
            "12",
        )),
    );

    Cylinder::new(
        cylinder_volume,
        cylinder_pressure,
        gas_mixture,
        gas_management,
    )
}