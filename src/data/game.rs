use crate::data::{Colony, Creep, Job, Spawn};
use std::collections::HashMap;

const CONSIDER_CREEP_EXPIRED_AT: u32 = 150;

#[derive(Default)]
pub struct Game {
    pub counter: u32,
    pub creeps: HashMap<String, Creep>,
    pub spawns: HashMap<String, Spawn>,
    pub colonies: HashMap<String, Colony>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            counter: 0,
            creeps: HashMap::new(),
            spawns: HashMap::new(),
            colonies: HashMap::new(),
        }
    }

    pub fn refresh_state(&mut self) {
        let start_time = screeps::game::cpu::get_used();
        debug!(
            "game refresh starting! CPU: {}",
            screeps::game::cpu::get_used()
        );
        debug!("counter: {}", self.counter);
        self.counter += 1;

        self.refresh_spawns();
        self.refresh_creeps();

        debug!("We have {} creeps", self.creeps.len());

        let duration = screeps::game::cpu::get_used() - start_time;
        debug!(
            "game refresh ended! duration: {} initial CPU: {}",
            duration, start_time
        );
    }

    fn refresh_creeps(&mut self) {
        for creep in screeps::game::creeps::values() {
            // hack to get typing to work
            let _creep: screeps::Creep = creep;
            let _name: String = _creep.name();
            match self.creeps.get_mut(_creep.name().as_str()) {
                Some(job_creep) => {
                    job_creep.refresh(_creep);
                }
                None => {
                    self.creeps.insert(_name, Creep::from(_creep));
                }
            }
        }

        let creep_names: Vec<String> = screeps::game::creeps::keys();
        let mut expired_creeps: Vec<String> = vec![];
        for creep in self.creeps.values() {
            let _name: &String = &creep.name().to_string();
            if !creep_names.contains(_name) {
                // we have to destroy this creep
                info!(
                    "marking creep as expired: {} of {}",
                    _name,
                    creep.job().as_str()
                );
                expired_creeps.push(_name.clone());
            }
        }

        for expired_creep in expired_creeps {
            info!("cleaning out creep {}", expired_creep);
            // we might want to run destructors here...
            self.creeps.remove(expired_creep.as_str());
        }
    }

    fn insert_spawn_into_colony(&mut self, spawn_id: &str) {
        let spawn: &Spawn = self.spawns.get(spawn_id).unwrap();
        let room_id = spawn.room_id();
        match self.colonies.get_mut(room_id) {
            Some(colony) => {
                colony.register_spawn(spawn.name().to_string());
            }
            None => {
                self.colonies
                    .insert(room_id.to_string(), Colony::from(spawn.room().to_owned()));
                let colony = self
                    .colonies
                    .get_mut(room_id)
                    .expect("colony insert failed");
                colony.register_spawn(spawn.name().to_string());
            }
        }
    }

    fn refresh_spawns(&mut self) {
        for spawn in screeps::game::spawns::values() {
            // hack to get typing to work
            let _spawn: screeps::StructureSpawn = spawn;
            let _name_str = _spawn.name();
            match self.spawns.get_mut(&_name_str) {
                Some(own_spawn) => {
                    own_spawn.refresh(_spawn);
                }
                None => {
                    self.spawns.insert(_spawn.name(), Spawn::from(_spawn));
                    self.insert_spawn_into_colony(&_name_str);
                }
            }
        }

        let spawn_names: Vec<String> = screeps::game::spawns::keys();
        let mut destroyed_spawns: Vec<String> = vec![];
        for spawn in self.spawns.values() {
            let _name: &String = &spawn.name().to_string();
            if !spawn_names.contains(_name) {
                // we have to destroy this spawn
                info!("marking spawn as destroyed: {}", _name);
                destroyed_spawns.push(_name.clone());
            }
        }

        for destroyed_spawn in destroyed_spawns {
            info!("cleaning out spawn {}", destroyed_spawn);
            // we might want to run destructors here...
            self.spawns.remove(destroyed_spawn.as_str());
        }
    }

    pub fn get_active_creep_by_job(&self, job: &Job) -> Vec<&Creep> {
        self.get_creep_by_job(job, true)
    }

    pub fn get_creep_by_job(&self, job: &Job, ignore_almost_expired: bool) -> Vec<&Creep> {
        self.creeps
            .iter()
            .filter(|(_, creep)| {
                if ignore_almost_expired && creep.ticks_to_live() <= CONSIDER_CREEP_EXPIRED_AT {
                    return false;
                }
                creep.job() == job
            })
            .map(|(_, c)| c)
            .collect()
    }
}
