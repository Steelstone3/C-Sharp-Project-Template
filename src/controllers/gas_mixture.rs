pub fn calculate_helium_percentage_maximum_limit(oxygen: i32) -> i32 {
    return 100 - oxygen;
}

pub fn calculate_nitrogen_percentage(oxygen: i32, helium: i32) -> i32 {
    return 100 - oxygen - helium;
}

#[cfg(test)]
mod controllers_gas_mixture_should {
    #[test]
    fn calculate_helium_percentage() {
        assert_eq!(30, super::calculate_helium_percentage_maximum_limit(70))
    }

    #[test]
    fn calculate_nitrogen_percentage() {
        assert_eq!(20, super::calculate_nitrogen_percentage(40, 40))
    }
}