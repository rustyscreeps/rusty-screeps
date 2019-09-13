use crate::tasks::TaskTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Harvest {}

impl TaskTrait for Harvest {
    fn name(&self) -> &str {
        "Harvest"
    }
}
