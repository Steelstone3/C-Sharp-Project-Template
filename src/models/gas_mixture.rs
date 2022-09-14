use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct GasMixture {
    pub oxygen: u32,
    pub helium: u32,
    pub nitrogen: u32,
}

impl GasMixture {
    pub fn new(oxygen: u32, helium: u32) -> Self {
        GasMixture::assign_gas_mixture(oxygen, helium)
    }

    fn assign_gas_mixture(mut oxygen: u32, mut helium: u32) -> GasMixture {
        if oxygen > 100 {
            oxygen = 100 - helium;
        }

        if helium > 100 {
            helium = 100 - oxygen;
        }

        GasMixture {
            oxygen,
            helium,
            nitrogen: GasMixture::calculate_nitrogen_percentage(oxygen, helium),
        }
    }

    fn calculate_nitrogen_percentage(oxygen: u32, helium: u32) -> u32 {
        100 - oxygen - helium
    }
}

impl Default for GasMixture {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

#[cfg(test)]
mod gas_mixture_should {
    use super::*;

    #[test]
    fn allow_assignment_of_oxygen() {
        let gas_mixture = GasMixture::new(70,0);

        assert_eq!(70, gas_mixture.oxygen);
        assert_eq!(0, gas_mixture.helium);
        assert_eq!(30, gas_mixture.nitrogen);
    }

    #[test]
    fn allow_overflow_assignment_of_oxygen() {
        let gas_mixture = GasMixture::new(120, 5);

        assert_eq!(95, gas_mixture.oxygen);
        assert_eq!(5, gas_mixture.helium);
        assert_eq!(0, gas_mixture.nitrogen);
    }

    #[test]
    fn allow_assignment_of_helium() {
        let gas_mixture = GasMixture::new(21, 70);

        assert_eq!(70, gas_mixture.helium);
        assert_eq!(21, gas_mixture.oxygen);
        assert_eq!(9, gas_mixture.nitrogen);
    }

    #[test]
    fn allow_overflow_assignment_of_helium() {
        let gas_mixture = GasMixture::new(5,120);

        assert_eq!(95, gas_mixture.helium);
        assert_eq!(5, gas_mixture.oxygen);
        assert_eq!(0, gas_mixture.nitrogen);
    }

    #[test]
    fn calculate_nitrogen_percentage() {
        let gas_mixture = GasMixture::new(40,40);

        assert_eq!(40, gas_mixture.oxygen);
        assert_eq!(40, gas_mixture.helium);
        assert_eq!(20, gas_mixture.nitrogen);
    }
}
