use crate::data::Creep;
use crate::tasks::task::TaskError::Invalid;
use crate::tasks::task::{TaskOption, TaskResult};
use crate::tasks::TaskTrait;
use screeps::{find, HasId, ReturnCode, Source};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Harvest {
    #[serde(skip_serializing_if = "Option::is_none")]
    _source_id: Option<String>,
}

impl Harvest {
    fn is_valid(&self, creep: &Creep, source: &Source) -> TaskResult {
        if source.energy() == 0 && source.ticks_to_regeneration() > 10
            || creep.carry_total() == creep.carry_capacity()
        {
            return Err(Invalid);
        }
        Ok(())
    }
}

impl TaskTrait for Harvest {
    fn name(&self) -> &str {
        "Harvest"
    }

    fn start(&mut self, creep: &Creep) {
        if self._source_id.is_none() {
            let _source = creep
                .room()
                .find(find::SOURCES)
                .iter()
                .find(|source| source.energy() > 0)
                .cloned();
            if let Some(source) = _source {
                self._source_id = Some(source.id());
            }
        }
    }

    fn execute(&self, creep: &Creep) -> TaskResult {
        let source_id = &self._source_id.as_ref().ok_or_invalid()?;
        let source = &screeps::game::get_object_typed(source_id)?.ok_or_invalid()?;
        self.is_valid(creep, source)?;
        if creep.pos().is_near_to(source) {
            let r = creep.harvest(source);
            if r != ReturnCode::Ok {
                // this will invalidate the task next tick
                if r != ReturnCode::NotEnough {
                    warn!("couldn't harvest upgrader: {:?}", r);
                }
            }
        } else {
            creep.move_to(source);
        }
        Ok(())
    }
}
