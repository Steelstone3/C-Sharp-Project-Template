pub mod gas_mixture_controller {
    pub fn calculate_helium_percentage_maximum_limit(oxygen:i32) -> i32{
        return 100 - oxygen;
    }

    pub fn calculate_nitrogen_percentage(oxygen:i32, helium:i32) ->i32 {
        return 100 - oxygen - helium;
    }
}

#[cfg(test)]
mod gas_mixture_controller_should {
    use super::*;

    #[test]
    #[ignore]
    fn calculate_helium_percentage() {
        assert_eq!(30, gas_mixture_controller::calculate_helium_percentage_maximum_limit(70))
    }

    #[test]
    #[ignore]
    fn calculate_nitrogen_percentage() {
        assert_eq!(20, gas_mixture_controller::calculate_nitrogen_percentage(40,40))
    }
}