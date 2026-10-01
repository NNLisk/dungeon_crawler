mod game;

use std::{io};

use game::{player::Player, game_instance::Game};




fn main() {

    let player = Player::new(String::from("samuel"), 100);
    let mut game = Game::new(player);

    start_menu(&mut game);


}


fn start_menu(p: &mut Game) {
    
    println!("Welcome to dungeon crawler!");

    let mut name = String::new();
    

    io::stdin()
        .read_line(&mut name)
        .expect("InputError: player name");

    p.get_player_ref().set_name(name);


}