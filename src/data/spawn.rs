use crate::data::Job;
use core::borrow::Borrow;
use screeps::{
    CanStoreEnergy, HasPosition, Part, ReturnCode, Room, RoomName, RoomObjectProperties,
    RoomPosition,
};

pub struct Spawn {
    _source: screeps::StructureSpawn,
    name: String,
    pos: RoomPosition,
    room: Room,
    room_id: RoomName,
}

impl Spawn {
    pub fn from(spawn: screeps::StructureSpawn) -> Spawn {
        let _source = spawn.borrow();
        let name: String = _source.name();
        let pos = _source.pos();
        let room = _source.room();
        let room_id = room.name();
        Spawn {
            _source: spawn,
            name,
            pos,
            room,
            room_id,
        }
    }

    pub fn refresh(&mut self, spawn: screeps::StructureSpawn) {
        let _source = spawn.borrow();
        self._source = spawn;
    }

    pub fn name(&self) -> &str {
        self.name.borrow()
    }

    pub fn pos(&self) -> &RoomPosition {
        self.pos.borrow()
    }

    pub fn room(&self) -> &Room {
        self.room.borrow()
    }

    pub fn room_id(&self) -> &RoomName {
        self.room_id.borrow()
    }

    pub fn energy(&self) -> u32 {
        self._source.energy()
    }

    pub fn energy_capacity(&self) -> u32 {
        self._source.energy_capacity()
    }

    pub fn spawn_job_creep(&self, body: &[Part], job: Job) -> ReturnCode {
        let name = screeps::game::time();
        let mut additional = 0;
        loop {
            let name = format!("{}:{}{}", job.as_str(), name, additional);
            let res = self.spawn_creep(&body, &name);

            if res == ReturnCode::NameExists {
                additional += 1;
            } else {
                break res;
            }
        }
    }

    fn spawn_creep(&self, body: &[Part], name: &str) -> ReturnCode {
        self._source.spawn_creep(body, name)
    }
}
