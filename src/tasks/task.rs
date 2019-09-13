use enum_dispatch::enum_dispatch;

use serde::{Deserialize, Serialize};

use crate::tasks::Harvest;

#[enum_dispatch]
pub trait TaskTrait {
    fn name(&self) -> &str;
}

#[enum_dispatch(TaskTrait)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Task {
    Harvest,
}
