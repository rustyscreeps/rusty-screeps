#![recursion_limit = "128"]
extern crate core;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use crate::data::Game;
use std::collections::HashSet;
use std::sync::{Mutex, MutexGuard};

use crate::lazy_static::__Deref;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod data;
pub mod logging;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

pub fn game<'a>() -> MutexGuard<'a, Game> {
    let game: MutexGuard<Game> = GAME.lock().unwrap();
    return game;
}

pub fn init_screeps_connection(game_loop: &'static dyn Fn(&Game)) {
    stdweb::initialize();
    std::panic::set_hook(Box::new(|info| {
        let value = &info.to_string();
        let panic_message = value.to_owned();
        js! { @(no_return)
            console.error( @{value} );
        }
        panic!(panic_message);
    }));
    logging::setup_logging(logging::Info);
    info!(
        "Global Reset! Compile took: {}",
        screeps::game::cpu::get_used()
    );

    let _game_loop = move || {
        debug!("loop starting! CPU: {}", screeps::game::cpu::get_used());

        game().refresh_state();

        game_loop(game().deref());

        let time = screeps::game::time();

        if time % 128 == 3 {
            info!("running memory cleanup");
            cleanup_memory().expect("expected Memory.creeps format to be a regular memory object");
        }

        debug!("done! cpu: {}", screeps::game::cpu::get_used())
    };

    js! {
        var game_loop = @{_game_loop};

        module.exports.loop = function() {
            // Provide actual error traces.

            if(Game.cpu.bucket < 500){
                console.log("we are running out of time!" + JSON.stringify(Game.cpu));
                return;
            }
            try {
                game_loop();
            } catch (error) {
                // console_error function provided by 'screeps-game-api'
                console_error("caught exception:", error);
                if (error.stack) {
                    console_error("stack trace:", error.stack);
                }
                console_error("resetting VM next tick.");
                // reset the VM since we don't know if everything was cleaned up and don't
                // want an inconsistent state.
                module.exports.loop = wasm_initialize;
            }
        }
    }
}

fn cleanup_memory() -> Result<(), Box<dyn (::std::error::Error)>> {
    let alive_creeps: HashSet<String> = screeps::game::creeps::keys().into_iter().collect();

    let screeps_memory = match screeps::memory::root().dict("creeps")? {
        Some(v) => v,
        None => {
            warn!("not cleaning game creep memory: no Memory.creeps dict");
            return Ok(());
        }
    };

    for mem_name in screeps_memory.keys() {
        if !alive_creeps.contains(&mem_name) {
            debug!("cleaning up creep memory of dead creep {}", mem_name);
            screeps_memory.del(&mem_name);
        }
    }

    Ok(())
}
