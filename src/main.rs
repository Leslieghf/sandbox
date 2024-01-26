pub mod game;

use game::*;

use log::*;

fn main() {
    let game_manager = GAME_MANAGER.clone();
    let mut game_manager = match game_manager.lock() {
        Ok(game_manager) => game_manager,
        Err(_) => panic!("Failed to lock game manager!"),
    };

    let mut game_handle = game_manager.create_game("your_mom".to_string());
    let game = match game_handle.access(true) {
        Ok(game) => game,
        Err(_) => panic!("Failed to lock game!"),
    };

    // continue implementing example here
    // do extensive testing
    // insert lot's of error/warn/info/debug/trace messages
    // add checks to prevent a module being edited while it is currently being used




    // hmmmmmm let's turn on our brains for a second

    //          WARNING: When continuing this work, review the code a bit to re-familiarize yourself with it.


    // TODO:    Rethink the decision to make one module consist of one piece of config and one piece of state. 
    //          Modules define one TYPE of config, and one TYPE of state, but there can be multiple instance pairs of config/state
    //          So, a module is a TYPE of config/state, and a module instance is a specific instance of that TYPE of config/state
    //          So, a module is only a lightweight identifier, and not a complex structure. The complexity is hidden inside config/state and the source code inside the module.
    //          I propose we reduce the multi-structured concept of a module into a single simple struct. This struct contains the module id, name, and the module type.

    //          Maybe we split the entire concept of a module into two structs: ModuleType and ModuleInstance
}