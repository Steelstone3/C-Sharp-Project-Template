pub mod cylinder {
    use crate::controllers::gas_management::gas_management::calculate_initial_pressurised_cylinder_volume;
    use crate::controllers::gas_mixture::gas_mixture_controller;
    use crate::models::cylinder::cylinder::Cylinder;
    use crate::models::gas_management::gas_management::GasManagement;
    use crate::models::gas_mixture::gas_mixture::GasMixture;
    use crate::presenters::presenter::presenters::{read_numeric_i32, read_string};

    pub fn create_cylinder() -> Cylinder {
        let name = read_string("Enter cylinder name:");
        let cylinder_volume = read_numeric_i32("Enter cylinder volume (L):", 3, 30);
        let cylinder_pressure = read_numeric_i32("Enter cylinder starting pressure (Bar):", 50, 300);
        let surface_air_consumption_rate = read_numeric_i32("Enter surface air consumption rate (L/min):", 5, 30);

        let initial_pressurised_cylinder_volume = calculate_initial_pressurised_cylinder_volume(cylinder_volume, cylinder_pressure);
        let gas_mixture = enter_gas_mixture();
        let gas_management = initialise_gas_management(initial_pressurised_cylinder_volume, surface_air_consumption_rate);

        return Cylinder { name, cylinder_volume, cylinder_pressure, gas_mixture, gas_management };
    }

    pub fn display_cylinder(gas_management: GasManagement) {
        println!("\nInitial pressurised cylinder volume: {} (L)\nGas remaining: {} (L)\nGas used for current dive step: {} (L)\n", gas_management.initial_pressurised_cylinder_volume, gas_management.gas_remaining, gas_management.gas_used);
    }

    fn enter_gas_mixture() -> GasMixture {
        let oxygen = read_numeric_i32("Enter oxygen (%):", 5, 100);
        let helium = read_numeric_i32("Enter helium (%):", 0, gas_mixture_controller::calculate_helium_percentage_maximum_limit(oxygen));
        let nitrogen = gas_mixture_controller::calculate_nitrogen_percentage(oxygen, helium);

        return GasMixture { oxygen, helium, nitrogen };
    }

    fn initialise_gas_management(initial_pressurised_cylinder_volume: i32, surface_air_consumption_rate: i32) -> GasManagement {
        return GasManagement {
            initial_pressurised_cylinder_volume,
            gas_used: 0,
            gas_remaining: initial_pressurised_cylinder_volume,
            surface_air_consumption_rate,
        };
    }
}
