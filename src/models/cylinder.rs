pub mod cylinder {
    use crate::models::gas_management::gas_management::GasManagement;
    use crate::models::gas_mixture::gas_mixture::GasMixture;

    pub struct Cylinder{
        pub cylinder_volume: i32,
        pub cylinder_pressure: i32,
        pub gas_mixture: GasMixture,
        pub gas_management: GasManagement,
    }
}