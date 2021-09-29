pub mod dive_results {
    use crate::models::dive_profile::dive_profile_model::DiveProfileModel;

    pub fn display_results(results: DiveProfileModel) {
            let mut _compartments = 0;

            for compartment in 0..16 {
                println!("C: {} | TPt: {} | TAP: {} | MSP: {} | CLp: {}", compartment + 1, results.tissue_pressures_total[compartment], results.tolerated_ambient_pressures[compartment], results.max_surface_pressures[compartment], results.compartment_load[compartment]);

                _compartments += 1;
            }
    }
}