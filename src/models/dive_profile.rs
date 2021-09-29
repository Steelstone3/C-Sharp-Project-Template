pub mod dive_profile_model {
    #[derive(Copy, Clone)]
    pub struct DiveProfileModel {
        pub max_surface_pressures: [f32; 16],
        pub compartment_load: [f32; 16],
        pub tissue_pressures_nitrogen: [f32; 16],
        pub tissue_pressures_helium: [f32; 16],
        pub tissue_pressures_total: [f32; 16],
        pub tolerated_ambient_pressures: [f32; 16],
        pub a_values: [f32; 16],
        pub b_values: [f32; 16],
        pub pressure_oxygen: f32,
        pub helium_at_pressure: f32,
        pub nitrogen_at_pressure: f32,
    }
}

