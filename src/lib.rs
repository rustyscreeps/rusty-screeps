extern crate core;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use crate::data::Game;
use crate::lazy_static::__Deref;
use crate::tasks::{Harvest, Task, TaskTrait};
use std::collections::HashSet;
use std::sync::{Mutex, MutexGuard};
use stdweb::js;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod data;
pub mod logging;
pub mod tasks;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

pub fn game<'a>() -> MutexGuard<'a, Game> {
    GAME.lock().unwrap()
}

fn test_serde() -> Result<String, serde_json::Error> {
    let begin = screeps::game::cpu::get_used();
    let task: Task = Harvest {}.into();
    let packed = serde_json::to_string(&task)?;
    let unpacked: Task = serde_json::from_str(&packed)?;
    let end = screeps::game::cpu::get_used() - begin;
    info!("we took {}", end);
    Ok(unpacked.name().to_string())
}

pub fn init_screeps_connection(game_loop: &'static dyn Fn(&Game)) {
    std::panic::set_hook(Box::new(|info| {
        let msg = &info.to_string();
        let panic_message = msg.to_owned();
        /*
        In case this might be useful
        // Add the error stack to our message.
        //
        // This ensures that even if the `console` implementation doesn't
        // include stacks for `console.error`, the stack is still available
        // for the user. Additionally, Firefox's console tries to clean up
        // stack traces, and ruins Rust symbols in the process
        // (https://bugzilla.mozilla.org/show_bug.cgi?id=1519569) but since
        // it only touches the logged message's associated stack, and not
        // the message's contents, by including the stack in the message
        // contents we make sure it is available to the user.
        msg.push_str("\n\nStack:\n\n");
        let e = Error::new();
        let stack = e.stack();
        msg.push_str(&stack);

        // Safari's devtools, on the other hand, _do_ mess with logged
        // messages' contents, so we attempt to break their heuristics for
        // doing that by appending some whitespace.
        // https://github.com/rustwasm/console_error_panic_hook/issues/7
        msg.push_str("\n\n");
        */
        js! { @(no_return)
            console.error( @{msg} );
        }
        panic!(panic_message);
    }));
    logging::setup_logging(logging::Info);
    info!(
        "Global Reset! Compile took: {}",
        screeps::game::cpu::get_used()
    );

    match test_serde() {
        Ok(thetask) => info!("{}", thetask),
        Err(theerror) => info!("{:?}", theerror),
    }

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
        // show more error frames in js (default: 10)
        Error.stackTraceLimit = 25;

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
