pub mod dive_model {
    use crate::factories::zhl16_dive_model::zhl16_dive_model::create_zhl16_dive_model;
    use crate::models::dive_model::dive_model::DiveModel;

    pub fn select_dive_model() -> DiveModel {
        println!("ZHL16 Bulhmann model selected");
        return create_zhl16_dive_model();
    }
}