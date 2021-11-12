pub mod cylinder {
    use crate::models::cylinder::cylinder::Cylinder;
    use crate::presenters::dive_data::cylinder::cylinder::create_cylinder;
    use crate::presenters::presenter::presenters::{read_boolean, read_numeric_i32};

    pub fn create_cylinders() -> Vec<Cylinder> {
        let mut cylinders: Vec<Cylinder> = Vec::new();
        cylinders.push(create_cylinder());

        //TODO This is skipping to false when "y" "yes" (etc) is entered by the user
        while read_boolean("\nCreate another cylinder [y/N]:") {
            cylinders.push(create_cylinder());
        }
        
        return cylinders;
    }

    pub fn select_cylinder(cylinders: &mut Vec<Cylinder>) -> usize {
        for index in 0..cylinders.len() {
            println!("\nMix {}: Oxygen: {}%, Nitrogen: {}%, Helium: {}%", index, cylinders[index].gas_mixture.oxygen,cylinders[index].gas_mixture.nitrogen, cylinders[index].gas_mixture.helium);
        }

        return read_numeric_i32("Select cylinder:", 0, (cylinders.len() - 1) as i32) as usize;
    }
}