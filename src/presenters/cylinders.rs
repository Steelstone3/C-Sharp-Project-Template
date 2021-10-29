pub mod cylinder {
    use crate::models::cylinder::cylinder::Cylinder;
    use crate::presenters::cylinder::cylinder::create_cylinder;
    use crate::presenters::presenter::presenters::read_numeric_i32;

    pub fn create_cylinders() -> Vec<Cylinder> {
        let mut cylinders: Vec<Cylinder> = Vec::new();
        let cylinder = create_cylinder();
        cylinders.push(cylinder);

        //TODO while user wants to create more cylinders let them loop REQUIRES: new read_boolean presenter logic

        return cylinders;
    }

    pub fn select_cylinder(cylinders: &mut Vec<Cylinder>) -> usize {
        for index in 0..cylinders.len() {
            println!("\nMix {}: Oxygen: {}%, Nitrogen: {}%, Helium: {}%", index, cylinders[index].gas_mixture.oxygen,cylinders[index].gas_mixture.nitrogen, cylinders[index].gas_mixture.helium);
        }

        return read_numeric_i32("Select cylinder:", 0, (cylinders.len() - 1) as i32) as usize;
    }
}