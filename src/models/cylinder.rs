use std::fmt::Display;

use crate::models::gas_management::GasManagement;
use crate::models::gas_mixture::GasMixture;

pub struct Cylinder {
    pub cylinder_volume: u32,
    pub cylinder_pressure: u32,
    pub gas_mixture: GasMixture,
    pub gas_management: GasManagement,
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