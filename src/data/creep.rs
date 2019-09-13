use crate::tasks::Task;
use core::borrow::Borrow;
use screeps::memory::MemoryReference;
use screeps::HasPosition;
use screeps::ReturnCode;
use screeps::Room;
use screeps::RoomObjectProperties;
use screeps::RoomPosition;
use screeps::Source;
use screeps::StructureController;
use screeps::{ConstructionSite, ResourceType, Transferable};
use std::fmt;
use stdweb::js;

pub struct Creep {
    _source: screeps::Creep,
    name: String,
    spawning: bool,
    carry_total: u32,
    carry_capacity: u32,
    ticks_to_live: u32,
    pos: RoomPosition,
    room: Room,
    job: Job,
    tasks: Vec<Task>,
}

impl Creep {
    pub fn from(creep: screeps::Creep) -> Creep {
        let _source = creep.borrow();
        let name: String = _source.name();
        let job_name: String = name.split(':').next().expect("invalid name!").to_string();
        let spawning = _source.spawning();
        let carry_total = _source.carry_total();
        let carry_capacity = _source.carry_capacity();
        let ticks_to_live: u32 = if !spawning {
            _source.ticks_to_live()
        } else {
            1500
        };
        let pos = _source.pos();
        let room = _source.room();
        Creep {
            name,
            spawning,
            carry_total,
            carry_capacity,
            ticks_to_live,
            pos,
            room,
            job: Job::from_string(job_name),
            tasks: vec![],
            _source: creep,
        }
    }

    fn unpack_tasks(&self, creep: &screeps::Creep) -> Option<Vec<Task>> {
        let _packed_tasks = creep.memory().string("tasks");
        if _packed_tasks.is_err() {
            error!("could not unpack!");
        }
        let packed_tasks = _packed_tasks.unwrap();
        if packed_tasks.is_none() {
            return None;
        }
        let tasks = packed_tasks.unwrap();
        let unpacked: Result<Vec<Task>, serde_json::error::Error> = serde_json::from_str(&tasks);
        if unpacked.is_err() {
            error!(
                "could not deserialize: '{}' with {:?}",
                tasks,
                unpacked.unwrap_err()
            );
            return None;
        }
        return Some(unpacked.unwrap());
    }

    pub fn refresh(&mut self, creep: screeps::Creep) {
        let _source = creep.borrow();
        self.spawning = _source.spawning();
        self.carry_total = _source.carry_total();
        self.carry_capacity = _source.carry_capacity();
        if !self.spawning {
            // this will panic otherwise
            self.ticks_to_live = _source.ticks_to_live();
        }
        self.pos = _source.pos();
        self.room = _source.room();
        if let Some(tasks) = self.unpack_tasks(_source) {
            self.tasks = tasks;
        }
        self._source = creep;
        self.show_creep_circle();
    }

    fn show_creep_circle(&self) {
        let room = self.room.name();
        let x = self.pos.x();
        let y = self.pos.y();
        js! {
            console.addVisual(@{room}, {t: 'c', x: @{x}, y: @{y},
            s: {radius: 0.55, fill: "transparent", stroke: "red"}});
        }
    }

    pub fn name(&self) -> &str {
        self.name.borrow()
    }

    pub fn spawning(&self) -> bool {
        self.spawning
    }

    pub fn memory(&self) -> MemoryReference {
        self._source.memory()
    }

    pub fn carry_total(&self) -> u32 {
        self.carry_total
    }

    pub fn carry_capacity(&self) -> u32 {
        self.carry_capacity
    }

    pub fn ticks_to_live(&self) -> u32 {
        self.ticks_to_live
    }

    pub fn pos(&self) -> &RoomPosition {
        self.pos.borrow()
    }

    pub fn room(&self) -> &Room {
        self.room.borrow()
    }

    pub fn move_to<T: ?Sized + HasPosition>(&self, target: &T) -> ReturnCode {
        self._source.move_to(target)
    }

    pub fn harvest(&self, source: &Source) -> ReturnCode {
        self._source.harvest(source)
    }

    pub fn build(&self, construction_site: &ConstructionSite) -> ReturnCode {
        self._source.build(construction_site)
    }

    pub fn upgrade_controller(&self, controller: &StructureController) -> ReturnCode {
        self._source.upgrade_controller(controller)
    }

    pub fn transfer_all_energy<T: ?Sized + Transferable>(&self, target: &T) -> ReturnCode {
        self._source.transfer_all(target, ResourceType::Energy)
    }

    pub fn job(&self) -> &Job {
        self.job.borrow()
    }
}

#[derive(Debug, PartialEq)]
pub enum Job {
    Upgrader,
    Starter,
    Builder,
    Unassigned,
}

impl Job {
    fn from_string(job: String) -> Job {
        match job.as_str() {
            "Upgrader" => Job::Upgrader,
            "Starter" => Job::Starter,
            "Builder" => Job::Builder,
            _ => Job::Unassigned,
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            Job::Upgrader => "Upgrader",
            Job::Starter => "Starter",
            Job::Builder => "Builder",
            Job::Unassigned => "Unassigned",
        }
    }
}

impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
