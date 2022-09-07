use inquire::Select;
use inquire::Text;

use crate::models::cylinder::Cylinder;
use crate::models::gas_management::GasManagement;
use crate::models::gas_mixture::GasMixture;

pub fn welcome_message() {
    println!("Welcome to Bubbles Dive Planner Console Rust");
}

pub fn select_dive_model() -> String {
    Select::new(
        "Select Dive Model",
        vec!["Zhl16".to_string(), "USN-V6".to_string()],
    )
    .prompt()
    .unwrap()
}

pub fn create_cylinders() {
    create_cylinder();
}

fn create_cylinder() -> Cylinder {
    let cylinder_volume = parse_numeric_value(
        Text::new("Enter cylinder volume (L):")
            .with_help_message("Enter a value 3 - 30")
            .with_default("12")
            .prompt()
            .unwrap(),
    );
    let cylinder_pressure = parse_numeric_value(
        Text::new("Enter cylinder volume (L):")
            .with_help_message("Enter a value 3 - 30")
            .with_default("12")
            .prompt()
            .unwrap(),
    );

    Cylinder {
        cylinder_volume: 20,
        cylinder_pressure: 20,
        gas_mixture: GasMixture {
            oxygen: 21,
            helium: 0,
            nitrogen: 79,
        },
        gas_management: GasManagement {
            initial_pressurised_cylinder_volume: 0,
            gas_used: 0,
            gas_remaining: 0,
            surface_air_consumption_rate: 0,
        },
    }
    // let cylinder_volume = read_numeric_i32("\nEnter cylinder volume (L):", 3, 30);
    // let cylinder_pressure = read_numeric_i32("Enter cylinder starting pressure (Bar):", 50, 300);
    // let surface_air_consumption_rate = read_numeric_i32("Enter surface air consumption rate (L/min):", 5, 30);
}

fn parse_numeric_value(input: String) -> u32 {
    match input.chars().find(|character| character.is_numeric()) {
        Some(_) => input.as_str().trim().parse::<u32>().unwrap(),
        None => panic!("Not a numeric value"),
    }
}
