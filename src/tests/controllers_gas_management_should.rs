#[cfg(test)]
mod controllers_gas_management_should {
    use crate::controllers::gas_management::gas_management;
    use crate::models::gas_management::gas_management::GasManagement;
    use crate::tests::test_fixtures_dive_stage::test_fixtures_dive_stage::test_fixture_dive_step;

    #[test]
    fn calculate_initial_pressurised_cylinder_volume() {
        //Arrange
        let cylinder_volume = 12;
        let cylinder_pressure = 200;
        let expected_pressurised_cylinder_volume = 2400;

        //Act
        let result = gas_management::calculate_initial_pressurised_cylinder_volume(cylinder_volume, cylinder_pressure);

        //Assert
        assert_eq!(expected_pressurised_cylinder_volume, result);
    }

    #[test]
    fn update_gas_management() {
        //Arrange
        let dive_step = test_fixture_dive_step();
        let gas_management = GasManagement {
            surface_air_consumption_rate: 12,
            gas_used: 0,
            gas_remaining: 2400,
            initial_pressurised_cylinder_volume: 2400,
        };

        //Act
        let result = gas_management::update_gas_management(gas_management, dive_step);

        //Assert
        assert_eq!(720, result.gas_used);
        assert_eq!(1680, result.gas_remaining);
    }
}