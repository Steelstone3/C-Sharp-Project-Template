pub mod cylinder {
    use crate::controllers::gas_mixture::gas_mixture_controller;
    use crate::models::gas_mixture::gas_mixture::GasMixture;
    use crate::presenters::presenter;

    pub fn enter_gas_mixture() -> GasMixture {
        let oxygen = presenter::read_numeric_i32("Enter oxygen (%):", 5, 100);
        let helium = presenter::read_numeric_i32("Enter helium (%):", 0, gas_mixture_controller::calculate_helium_percentage(oxygen));
        let nitrogen = gas_mixture_controller::calculate_nitrogen_percentage(oxygen, helium);

        return GasMixture { oxygen, helium, nitrogen };
    }
}
