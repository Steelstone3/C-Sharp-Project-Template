pub mod dive_model
{
    #[derive(Copy, Clone)]
    pub struct DiveModel {
        pub compartment_count: usize,
        pub nitrogen_half_time: [f32; 16],
        pub helium_half_time: [f32; 16],
        pub a_values_nitrogen: [f32; 16],
        pub b_values_nitrogen: [f32; 16],
        pub a_values_helium: [f32; 16],
        pub b_values_helium: [f32; 16],
    }
}