mod game;
mod logger;

use std::{io};

use game::{
    player::Player, 
    game_instance::Game,
    enemy::{Enemy, initiate_enemy_types}
};
use logger::logger::{Logger, LogLevel};

fn main() {

    let log = Logger::new(LogLevel::Debug, "MAIN");

    log.debug("Game initiated");

    let player = start_menu();
    let enemies = initiate_enemy_types();
    
    let mut game = Game::new(player);

}


fn start_menu() -> Player {

    let range: i64;
    let speed: i64;
    let damage: i64;
    let hp = 1000;
    let luck = 1;
    let item_cap = 3;
    print!("=====================\n");

    println!("|| Welcome to dungeon crawler!\n|| Pick your starting weapon!\n|| (R)ange, (S)peed, (D)amage\n||");
    println!("|    Bow & Quiver (1)    |    Sword (2)    |    Great Hammer (3)    |");
    println!("|    R1000 S500 D250     | R150 S750 D450  |    R200 S100 D1000     |");

    let mut weapon = String::new();
    let mut name = String::new();

    loop {

        io::stdin()
        .read_line(&mut weapon)
        .expect("InputError: player name");

        let choice: i32 = weapon.trim().parse().expect("ParsingError");
        
        match choice {
            1 => {
                range = 1000;
                speed = 500;
                damage = 250;
                break;
            },
            2 => {
                range = 150;
                speed = 750;
                damage = 450;
                break;
            },
            3 => {
                range = 200;
                speed = 100;
                damage = 1000;
                break;
            },
            _ => continue
        };
    }

    println!("Great choice of weapon! What is your name warrior?");
    io::stdin()
        .read_line(&mut name)
        .expect("InputError");
    print!("=====================\n");

    let player = Player::new(name, hp, range, speed, damage, luck, item_cap);
    print!("=====================\n");
    print!("Pleasure to meet you {}!\nI am Ahrnam, the shopkeeper and a fellow crawler, like you\nYes, I see a hunter's glint in your eyes. It's no easy thing finding one's way in the dark.\nI'm rambling... If we meet down there, I might have some things to trade you. Toodle-oo!\n\n", player.get_name());

    print!("=====================\n{}\n=====================\n", player);
    player
}


