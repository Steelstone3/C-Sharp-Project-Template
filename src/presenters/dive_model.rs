pub mod dive_model {
    use crate::models::dive_model::dive_model::{create_zhl16_dive_model, DiveModel};

    pub fn select_dive_model() -> DiveModel {
        return create_zhl16_dive_model();
    }
}