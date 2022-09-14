use crate::models::gas_management::GasManagement;
use crate::models::gas_mixture::GasMixture;
use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Cylinder {
    pub cylinder_volume: u32,
    pub cylinder_pressure: u32,
    pub initial_pressurised_cylinder_volume: u32,
    pub gas_mixture: GasMixture,
    pub gas_management: GasManagement,
}

impl Cylinder {
    pub fn new(
        cylinder_volume: u32,
        cylinder_pressure: u32,
        gas_mixture: GasMixture,
        surface_air_consumption_rate: u32,
    ) -> Self {
        let initial_pressurised_cylinder_volume =
            Self::calculate_initial_pressurised_cylinder_volume(cylinder_volume, cylinder_pressure);

        Cylinder {
            cylinder_volume,
            cylinder_pressure,
            initial_pressurised_cylinder_volume,
            gas_mixture,
            gas_management: GasManagement::new(
                initial_pressurised_cylinder_volume,
                surface_air_consumption_rate,
            ),
        }
    }

    pub fn calculate_initial_pressurised_cylinder_volume(
        cylinder_volume: u32,
        cylinder_pressure: u32,
    ) -> u32 {
        cylinder_volume * cylinder_pressure
    }
}

impl Display for Cylinder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nMix: Oxygen: {}%, Nitrogen: {}%, Helium: {}% Gas Used: {}L Gas Remaining: {}L",
            self.gas_mixture.oxygen,
            self.gas_mixture.nitrogen,
            self.gas_mixture.helium,
            self.gas_management.gas_used,
            self.gas_management.gas_remaining
        )
    }
}

#[cfg(test)]
mod cylinder_should {
    use super::*;

    #[test]
    fn create_a_new_cylinder() {
        //Arrange
        let gas_mixture = GasMixture::new(21, 10);

        //Act
        let cylinder = Cylinder::new(12, 200, gas_mixture, 12);

        //Assert
        assert_eq!(12, cylinder.cylinder_volume);
        assert_eq!(200, cylinder.cylinder_pressure);
        assert_eq!(2400, cylinder.initial_pressurised_cylinder_volume);
        assert_eq!(21, cylinder.gas_mixture.oxygen);
        assert_eq!(10, cylinder.gas_mixture.helium);
        assert_eq!(69, cylinder.gas_mixture.nitrogen);
        assert_eq!(12, cylinder.gas_management.surface_air_consumption_rate);
        assert_eq!(2400, cylinder.gas_management.gas_remaining);
        assert_eq!(0, cylinder.gas_management.gas_used);
    }

    #[test]
    fn calculate_initial_pressurised_cylinder_volume() {
        //Arrange
        let cylinder_volume = 12;
        let cylinder_pressure = 200;
        let expected_pressurised_cylinder_volume = 2400;

        //Act
        let result = Cylinder::calculate_initial_pressurised_cylinder_volume(
            cylinder_volume,
            cylinder_pressure,
        );

        //Assert
        assert_eq!(expected_pressurised_cylinder_volume, result);
    }
}
