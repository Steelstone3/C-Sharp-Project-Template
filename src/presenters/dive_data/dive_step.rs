use crate::DiveStep;
use crate::presenters::presenter::read_numeric_i32;

pub fn enter_dive_step() -> DiveStep {
    let depth = read_numeric_i32("\nEnter depth (m):", 1, 100);
    let time = read_numeric_i32("Enter time (min):", 1, 60);

    return DiveStep { depth, time };
}