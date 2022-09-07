use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct DiveStep {
    pub depth: u32,
    pub time: u32,
}
