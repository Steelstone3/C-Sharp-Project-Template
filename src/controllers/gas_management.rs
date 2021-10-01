pub mod gas_management {
    use crate::models::dive_step::dive_step::DiveStep;
    use crate::models::gas_management::gas_management::GasManagement;

    pub fn calculate_initial_pressurised_cylinder_volume(cylinder_volume: i32, cylinder_pressure: i32) -> i32 {
        return cylinder_volume * cylinder_pressure;
    }

    pub fn update_gas_management(gas_management: GasManagement, dive_step: DiveStep) -> GasManagement {
        let gas_used = calculate_gas_used(dive_step, gas_management.surface_air_consumption_rate);
        let gas_remaining = calculate_remaining_pressurised_cylinder_volume(gas_management.gas_remaining, gas_used);

        return GasManagement {
            gas_used,
            gas_remaining,
            initial_pressurised_cylinder_volume: gas_management.initial_pressurised_cylinder_volume,
            surface_air_consumption_rate: gas_management.surface_air_consumption_rate,
        }
    }

    fn calculate_remaining_pressurised_cylinder_volume(remaining_gas: i32, gas_used: i32) -> i32 {
        return remaining_gas - gas_used;
    }

    fn calculate_gas_used(dive_step:DiveStep, surface_air_consumption_rate:i32) -> i32 {
        return ( ( dive_step.depth / 10 ) + 1 ) * dive_step.time * surface_air_consumption_rate;
    }
}
