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
}