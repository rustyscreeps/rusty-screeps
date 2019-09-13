use enum_dispatch::enum_dispatch;

use serde::{Deserialize, Serialize};

use crate::data::Creep;
use crate::tasks::Harvest;
use screeps::ConversionError;
use std::error::Error;

pub enum TaskError {
    Invalid,
    OptionFailed,
    ConversionError(ConversionError),
    Error(Box<dyn Error>),
}

pub trait TaskOption<T> {
    fn ok_or_invalid(self) -> Result<T, TaskError>;
}

impl<T> TaskOption<T> for Option<T> {
    fn ok_or_invalid(self) -> Result<T, TaskError> {
        if self.is_none() {
            return Err(TaskError::OptionFailed);
        }
        Ok(self.unwrap())
    }
}

impl From<ConversionError> for TaskError {
    fn from(error: ConversionError) -> Self {
        TaskError::ConversionError(error)
    }
}

pub type TaskResult = Result<(), TaskError>;

#[enum_dispatch]
pub trait TaskTrait {
    fn name(&self) -> &str;
    fn start(&mut self, creep: &Creep);
    fn execute(&self, creep: &Creep) -> TaskResult;
}

#[enum_dispatch(TaskTrait)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Task {
    Harvest,
}
