pub mod dive_stage {
    use crate::models::dive_model::dive_model::DiveModel;
    use crate::models::dive_profile::dive_profile_model::DiveProfileModel;
    use crate::models::dive_step::dive_step::DiveStep;
    use crate::models::gas_mixture::gas_mixture::GasMixture;

    pub fn run_dive_profile(dive_model: DiveModel, dive_step: DiveStep, gas_mixture: GasMixture) -> DiveProfileModel {
        todo!();
    }
}