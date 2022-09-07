#[derive(Copy, Clone)]
pub struct GasMixture {
    pub oxygen: i32,
    pub helium: i32,
    pub nitrogen: i32,
}

impl GasMixture {
    pub fn assign_oxygen(&mut self, oxygen: i32) {
        if oxygen > 100 {
            self.oxygen = 100 - self.helium;
        } else {
            self.oxygen = oxygen;
        }

        self.calculate_nitrogen_percentage();
    }

    pub fn assign_helium(&mut self, helium:i32) {
        if helium > 100 {
            self.helium = 100 - self.oxygen;
        } else {
            self.helium = helium;
        }

        self.calculate_nitrogen_percentage();
    }

    fn calculate_nitrogen_percentage(&mut self) -> i32 {
        self.nitrogen = 100 - self.oxygen - self.helium;
        self.nitrogen
    }
}

impl Default for GasMixture {
    fn default() -> Self {
        Self {
            oxygen: 21,
            helium: 0,
            nitrogen: 79,
        }
    }
}

#[cfg(test)]
mod gas_mixture_should {
    use super::*;

    #[test]
    fn allow_assignment_of_oxygen() {
        let mut gas_mixture = GasMixture::default();

        gas_mixture.assign_oxygen(70);

        assert_eq!(70, gas_mixture.oxygen);
        assert_eq!(0, gas_mixture.helium);
        assert_eq!(30, gas_mixture.nitrogen);
    }

    #[test]
    fn allow_overflow_assignment_of_oxygen() {
        let mut gas_mixture = GasMixture::default();
        gas_mixture.nitrogen = 30;
        gas_mixture.helium = 5;

        gas_mixture.assign_oxygen(120);

        assert_eq!(95, gas_mixture.oxygen);
        assert_eq!(5, gas_mixture.helium);
        assert_eq!(0, gas_mixture.nitrogen);
    }

    #[test]
    fn allow_assignment_of_helium() {
        let mut gas_mixture = GasMixture::default();

        gas_mixture.assign_helium(70);

        assert_eq!(70, gas_mixture.helium);
        assert_eq!(21, gas_mixture.oxygen);
        assert_eq!(9, gas_mixture.nitrogen);
    }

    #[test]
    fn allow_overflow_assignment_of_helium() {
        let mut gas_mixture = GasMixture::default();
        gas_mixture.nitrogen = 30;
        gas_mixture.oxygen = 5;

        gas_mixture.assign_helium(120);

        assert_eq!(95, gas_mixture.helium);
        assert_eq!(5, gas_mixture.oxygen);
        assert_eq!(0, gas_mixture.nitrogen);
    }

    #[test]
    fn calculate_nitrogen_percentage() {
        let mut gas_mixture = GasMixture::default();
        gas_mixture.oxygen = 40;
        gas_mixture.helium = 40;

        assert_eq!(20, gas_mixture.calculate_nitrogen_percentage());
        assert_eq!(40, gas_mixture.oxygen);
        assert_eq!(40, gas_mixture.helium);
        assert_eq!(20, gas_mixture.nitrogen);
    }
}
