use crate::DiveStep;
use crate::models::gas_management::GasManagement;

pub fn calculate_initial_pressurised_cylinder_volume(cylinder_volume: i32, cylinder_pressure: i32) -> i32 {
    cylinder_volume * cylinder_pressure
}

pub fn update_gas_management(gas_management: GasManagement, dive_step: DiveStep) -> GasManagement {
    let gas_used = calculate_gas_used(dive_step, gas_management.surface_air_consumption_rate);
    let gas_remaining = calculate_remaining_pressurised_cylinder_volume(gas_management.gas_remaining, gas_used);

    GasManagement {
        gas_used,
        gas_remaining,
        initial_pressurised_cylinder_volume: gas_management.initial_pressurised_cylinder_volume,
        surface_air_consumption_rate: gas_management.surface_air_consumption_rate,
    }
}

fn calculate_remaining_pressurised_cylinder_volume(remaining_gas: i32, gas_used: i32) -> i32 {
    remaining_gas - gas_used
}

fn calculate_gas_used(dive_step: DiveStep, surface_air_consumption_rate: i32) -> i32 {
    ((dive_step.depth / 10) + 1) * dive_step.time * surface_air_consumption_rate
}

#[cfg(test)]
mod controllers_gas_management_should {
    use crate::models::gas_management::GasManagement;
    use crate::tests::test_fixtures_dive_plan::test_fixture_dive_step_default;

    #[test]
    fn calculate_initial_pressurised_cylinder_volume() {
        //Arrange
        let cylinder_volume = 12;
        let cylinder_pressure = 200;
        let expected_pressurised_cylinder_volume = 2400;

        //Act
        let result = super::calculate_initial_pressurised_cylinder_volume(cylinder_volume, cylinder_pressure);

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
        let result = super::update_gas_management(gas_management, dive_step);

        //Assert
        assert_eq!(720, result.gas_used);
        assert_eq!(1680, result.gas_remaining);
    }
}