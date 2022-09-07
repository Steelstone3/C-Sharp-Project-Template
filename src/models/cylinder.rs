use std::fmt::Display;

use crate::models::gas_management::GasManagement;
use crate::models::gas_mixture::GasMixture;

use super::gas_management;

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
        gas_management: GasManagement,
    ) -> Self {
        let initial_pressurised_cylinder_volume =
            Self::calculate_initial_pressurised_cylinder_volume(cylinder_volume, cylinder_pressure);

        Cylinder {
            cylinder_volume,
            cylinder_pressure,
            initial_pressurised_cylinder_volume,
            gas_mixture,
            gas_management,
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
