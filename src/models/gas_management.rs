#[derive(Copy, Clone)]
pub struct GasManagement {
    pub initial_pressurised_cylinder_volume: i32,
    pub gas_used: i32,
    pub gas_remaining: i32,
    pub surface_air_consumption_rate: i32,
}