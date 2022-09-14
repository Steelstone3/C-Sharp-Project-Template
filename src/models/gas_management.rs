use super::dive_step::DiveStep;

#[derive(Copy, Clone)]
pub struct GasManagement {
    pub gas_used: u32,
    pub gas_remaining: u32,
    pub surface_air_consumption_rate: u32,
}

impl GasManagement {
    pub fn new(gas_remaining:u32, surface_air_consumption_rate: u32) -> Self {
        GasManagement {
            gas_used: 0,
            gas_remaining,
            surface_air_consumption_rate,
        }
    }

    pub fn update_gas_management(self, dive_step: DiveStep) -> Self {
        let gas_used =
            GasManagement::calculate_gas_used(dive_step, self.surface_air_consumption_rate);
        let gas_remaining =
            GasManagement::calculate_remaining_pressurised_cylinder_volume(self, gas_used);

        GasManagement {
            gas_used,
            gas_remaining,
            surface_air_consumption_rate: self.surface_air_consumption_rate,
        }
    }

    fn calculate_remaining_pressurised_cylinder_volume(self, gas_used: u32) -> u32 {
        self.gas_remaining - gas_used
    }

    fn calculate_gas_used(dive_step: DiveStep, surface_air_consumption_rate: u32) -> u32 {
        ((dive_step.depth / 10) + 1) * dive_step.time * surface_air_consumption_rate
    }
}

impl Default for GasManagement {
    fn default() -> Self {
        Self {
            gas_used: Default::default(),
            gas_remaining: Default::default(),
            surface_air_consumption_rate: 12,
        }
    }
}

#[cfg(test)]
mod gas_management_should {
    use super::*;

    #[test]
    fn update_gas_management() {
        //Arrange
        let dive_step = dive_step_test_fixture();
        let gas_management = gas_management_test_fixture();

        //Act
        let result = GasManagement::update_gas_management(gas_management, dive_step);

        //Assert
        assert_eq!(720, result.gas_used);
        assert_eq!(1680, result.gas_remaining);
    }

    fn dive_step_test_fixture() -> DiveStep {
        DiveStep {
            depth: 50,
            time: 10,
        }
    }

    fn gas_management_test_fixture() -> GasManagement {
        GasManagement {
            surface_air_consumption_rate: 12,
            gas_used: 0,
            gas_remaining: 2400,
        }
    }
}
