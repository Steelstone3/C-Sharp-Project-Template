use crate::factories::zhl16_dive_model::create_zhl16_dive_model;
use crate::models::dive_model::DiveModel;

pub fn select_dive_model() -> DiveModel {
        println!("\nZHL16 Bulhmann model selected");
        return create_zhl16_dive_model();
    }
