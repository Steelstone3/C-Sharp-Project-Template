use super::dive_step::DiveStep;

#[derive(Copy, Clone)]
pub struct GasManagement {
    pub initial_pressurised_cylinder_volume: i32,
    pub gas_used: i32,
    pub gas_remaining: i32,
    pub surface_air_consumption_rate: i32,
}

impl GasManagement{
    pub fn calculate_initial_pressurised_cylinder_volume(cylinder_volume: i32, cylinder_pressure: i32) -> i32 {
        cylinder_volume * cylinder_pressure
    }

    pub fn update_gas_management(self, dive_step: DiveStep) -> Self {
        let gas_used = GasManagement::calculate_gas_used(dive_step, self.surface_air_consumption_rate);
        let gas_remaining = GasManagement::calculate_remaining_pressurised_cylinder_volume(self.gas_remaining, gas_used);
        
        GasManagement {
            gas_used,
            gas_remaining,
            initial_pressurised_cylinder_volume: self.initial_pressurised_cylinder_volume,
            surface_air_consumption_rate: self.surface_air_consumption_rate,
        }
    }
    
    fn calculate_remaining_pressurised_cylinder_volume(remaining_gas: i32, gas_used: i32) -> i32 {
        remaining_gas - gas_used
    }
    
    fn calculate_gas_used(dive_step: DiveStep, surface_air_consumption_rate: i32) -> i32 {
        ((dive_step.depth / 10) + 1) * dive_step.time * surface_air_consumption_rate
    }
}
    
    #[cfg(test)]
mod gas_management_should {
    use crate::models::gas_management::GasManagement;
    use crate::tests::test_fixtures_dive_plan::test_fixture_dive_step_default;

    #[test]
    fn calculate_initial_pressurised_cylinder_volume() {
        //Arrange
        let cylinder_volume = 12;
        let cylinder_pressure = 200;
        let expected_pressurised_cylinder_volume = 2400;

        //Act
        let result = GasManagement::calculate_initial_pressurised_cylinder_volume(cylinder_volume, cylinder_pressure);

        //Assert
        assert_eq!(expected_pressurised_cylinder_volume, result);
    }

    #[test]
    fn update_gas_management() {
        //Arrange
        let dive_step = test_fixture_dive_step_default();
        let gas_management = GasManagement {
            surface_air_consumption_rate: 12,
            gas_used: 0,
            gas_remaining: 2400,
            initial_pressurised_cylinder_volume: 2400,
        };

        //Act
        let result = GasManagement::update_gas_management(gas_management, dive_step);

        //Assert
        assert_eq!(720, result.gas_used);
        assert_eq!(1680, result.gas_remaining);
    }
}