use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct DiveStep {
    pub depth: i32,
    pub time: i32,
}
