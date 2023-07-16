use serde::{Deserialize, Serialize};
 #[derive(Deserialize, Serialize, Debug)]
    pub struct Validator {
    pub epoch: i32,
    pub network_participation: i32,
    pub validator_participation: i32,
}

impl Validator {
    pub fn new() -> Self {
        Self {
            epoch: 0,
            network_participation: 0,
            validator_participation: 0,
        }
    }
}
