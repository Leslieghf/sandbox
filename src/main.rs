pub mod game;

use game::*;

use log::*;

fn main() {
    let game_manager = GAME_MANAGER.clone();
    let mut game_manager = match game_manager.lock() {
        Ok(game_manager) => game_manager,
        Err(_) => panic!("Failed to lock game manager!"),
    };

    let mut game = game_manager.create_game("your_mom".to_string());
    let game = match game.access(true) {
        Ok(game) => game,
        Err(_) => panic!("Failed to lock game!"),
    };

    // continue implementing example here
    // do extensive testing
    // insert lot's of error/warn/info/debug/trace messages
    // add checks to prevent a module being edited while it is currently being used




    // change gameconfig and gamestate to use the new module system
}