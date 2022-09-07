use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct DiveProfile {
    pub maximum_surface_pressures: [f32; 16],
    pub compartment_load: [f32; 16],
    pub tissue_pressures_nitrogen: [f32; 16],
    pub tissue_pressures_helium: [f32; 16],
    pub tissue_pressures_total: [f32; 16],
    pub tolerated_ambient_pressures: [f32; 16],
    pub a_values: [f32; 16],
    pub b_values: [f32; 16],
    pub oxygen_at_pressure: f32,
    pub helium_at_pressure: f32,
    pub nitrogen_at_pressure: f32,
}

impl Default for DiveProfile {
    fn default() -> Self {
        Self {
            maximum_surface_pressures: Default::default(),
            compartment_load: Default::default(),
            tissue_pressures_nitrogen: Default::default(),
            tissue_pressures_helium: Default::default(),
            tissue_pressures_total: Default::default(),
            tolerated_ambient_pressures: Default::default(),
            a_values: Default::default(),
            b_values: Default::default(),
            oxygen_at_pressure: Default::default(),
            helium_at_pressure: Default::default(),
            nitrogen_at_pressure: Default::default(),
        }
    }
}
