pub mod dive_step {
    use serde::{Serialize, Deserialize};

    #[derive(Copy, Clone, Serialize, Deserialize)]
    pub struct DiveStep {
        pub depth: i32,
        pub time: i32,
    }
}