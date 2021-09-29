pub mod gas_mixture_controller {
    pub fn calculate_helium_percentage(oxygen:i32) -> i32{
        return 100 - oxygen;
    }

    pub fn calculate_nitrogen_percentage(oxygen:i32, helium:i32) ->i32 {
        return 100 - oxygen - helium;
    }
}