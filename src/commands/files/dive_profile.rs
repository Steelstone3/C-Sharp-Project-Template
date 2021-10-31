pub mod dive_profile{
    use std::fs::File;
    use std::io::Write;
    use crate::models::dive_profile::dive_profile_model::DiveProfileModel;

    pub fn create_dive_profile_file(dive_profiles: &mut Vec<DiveProfileModel>) -> std::io::Result<()> {
        let mut json_dive_profile_file = File::create("dive_profile.json")?;
        let json_dive_profile = serde_json::ser::to_string_pretty(&dive_profiles)?;
        write!(json_dive_profile_file, "{}", json_dive_profile)?;
        Ok(())
    }
}