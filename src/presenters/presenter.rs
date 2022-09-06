use inquire::Select;
use inquire::Text;

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

pub fn create_cylinder() {
    let cylinder_volume = parse_numeric_value(Text::new("Enter cylinder volume (L):").with_help_message("Enter a value 3 - 30").with_default("12").prompt().unwrap());
    

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