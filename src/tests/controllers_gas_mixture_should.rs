#[cfg(test)]
mod controllers_gas_mixture_should {
    use crate::controllers::gas_mixture::gas_mixture_controller;

    #[test]
    fn calculate_helium_percentage() {
        assert_eq!(30, gas_mixture_controller::calculate_helium_percentage_maximum_limit(70))
    }

    #[test]
    fn calculate_nitrogen_percentage() {
        assert_eq!(20, gas_mixture_controller::calculate_nitrogen_percentage(40,40))
    }
}