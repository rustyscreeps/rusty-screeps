use core::borrow::Borrow;
use screeps::RoomName;

pub struct Colony {
    _source: screeps::Room,
    spawns: Vec<String>,
    name: RoomName,
    stage: ColonyStage,
}

impl Colony {
    pub fn from(room: screeps::Room) -> Colony {
        let _source = room.borrow();
        let name: RoomName = _source.name();
        let memory = room.memory().dict_or_create("colony").unwrap();
        if !memory.bool("stage") {
            memory.set("stage", ColonyStage::Bootstrap.as_str())
        }
        Colony {
            _source: room,
            spawns: vec![],
            name,
            stage: ColonyStage::from_string(memory.string("stage").unwrap().unwrap()),
        }
    }

    pub fn register_spawn(&mut self, spawn_id: String) {
        self.spawns.push(spawn_id);
    }

    pub fn name(&self) -> &RoomName {
        self.name.borrow()
    }

    pub fn stage(&self) -> &ColonyStage {
        self.stage.borrow()
    }

    pub fn room(&self) -> &screeps::Room {
        self._source.borrow()
    }

    pub fn spawns(&self) -> &Vec<String> {
        self.spawns.borrow()
    }
}

pub enum ColonyStage {
    Bootstrap,
    Running,
}

impl ColonyStage {
    pub fn from_string(stage: String) -> ColonyStage {
        match stage.as_str() {
            "bootstrap" => ColonyStage::Bootstrap,
            "running" => ColonyStage::Running,
            _ => ColonyStage::Bootstrap,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ColonyStage::Bootstrap => "bootstrap",
            ColonyStage::Running => "running",
        }
    }
}
