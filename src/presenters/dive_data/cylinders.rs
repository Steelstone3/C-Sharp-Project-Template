use crate::models::cylinder::Cylinder;
use crate::presenters::dive_data::cylinder::create_cylinder;
use crate::presenters::presenter::{read_boolean, read_numeric_i32};

pub fn create_cylinders() -> Vec<Cylinder> {
    let mut cylinders = vec!(create_cylinder());

    while read_boolean("\nCreate another cylinder [y/N]:") {
        cylinders.push(create_cylinder());
    }

    cylinders
}

pub fn select_cylinder(cylinders: &mut Vec<Cylinder>) -> usize {
    for (index, _) in cylinders.iter().enumerate() {
        println!("\nMix {}: Oxygen: {}%, Nitrogen: {}%, Helium: {}%", index, cylinders[index].gas_mixture.oxygen, cylinders[index].gas_mixture.nitrogen, cylinders[index].gas_mixture.helium);
    }

    read_numeric_i32("Select cylinder:", 0, (cylinders.len() - 1) as i32) as usize
}
