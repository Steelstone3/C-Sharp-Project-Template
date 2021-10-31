pub mod dive_step {
    use std::fs::File;
    use std::io::Write;
    use crate::models::dive_step::dive_step::DiveStep;

    pub fn create_dive_step_file(dive_step: &DiveStep) -> std::io::Result<()> {
        let mut json_dive_step_file = File::create("dive_step.json")?;
        let json_dive_step = serde_json::ser::to_string_pretty(&dive_step)?;
        write!(json_dive_step_file, "{}", json_dive_step)?;
        Ok(())
    }
}