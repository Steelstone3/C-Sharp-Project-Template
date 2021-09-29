pub mod dive_step {
    use crate::models::dive_step::dive_step::DiveStep;
    use crate::presenters::presenter;

    pub fn enter_dive_step() -> DiveStep {
        let depth = presenter::read_numeric_i32("Enter depth (m):", 1, 100);
        let time = presenter::read_numeric_i32("Enter time (min):", 1, 60);

        return DiveStep { depth, time };
    }
}
