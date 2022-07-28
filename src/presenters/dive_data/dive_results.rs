use crate::DiveProfile;

pub fn display_results(results: DiveProfile) {
    println!();
    let mut _compartments = 0;

    for (_compartments, compartment) in (0..results.compartment_load.len()).enumerate() {
        println!("C: {} | TPt: {} | TAP: {} | MSP: {} | CLp: {}", compartment + 1, results.tissue_pressures_total[compartment], results.tolerated_ambient_pressures[compartment], results.maximum_surface_pressures[compartment], results.compartment_load[compartment]);
    }
}
