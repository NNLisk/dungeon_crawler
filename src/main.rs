mod game;
mod logger;

use std::{io, process::exit};

use game::{
    player::Player, 
    game_instance::Game,
    game_instance::PlayerError,
};
use logger::logger::{Logger, LogLevel};


fn main() {

    let log = Logger::new(LogLevel::Debug, "MAIN");
    let mut boss = false;

    log.debug("Program start");

    let player = start_menu();
    log.debug("player created");

    let mut game = Game::new(player);

    log.debug("game created");

    prepare_player(&mut game);

    // FULL LOOP
    loop {

        loop {

            if game.get_room() == 4 {
                boss = true;
            }

            if game.get_room() % 2 == 0 {
                game.shop();
                log.debug("Shop complete");
            }

            match game.fight(boss) {
                Ok(()) => {
                    log.info("Player Won!");
                    game.set_room(game.get_room() + 1);
                },
                Err(PlayerError::Dead) => {
                    log.info("Player has died");
                    game.get_player_ref().set_alive(false);
                    break;
                }
            }

            boss = false;

            // FLOOR ENDING
            if game.get_room() == 0 {
                break;
            }
        }

        if !game.get_player_ref().get_alive() {
            player_lost_menu(&mut game);
            break;
        }
        
        game.set_floor(game.get_floor() + 1);
        game.set_room(1);

        if game.get_floor() == 0 {
            player_won_menu(&mut game);
            break;
        }
    }


}

fn prepare_player(g: &mut Game) {
    g.get_starting_potions();
    
    println!("=====================\nGame is dungeon crawler based on real time fighting, during the fight, you have to write \"hit\", to attack, \"heal\" to heal and \"shield\" to shield up\n the enemy will attack periodically if you're lucky they might miss\n=====================");
    
    loop {

        println!("Start game? (y/n)");
        let input = g.read_input();

        match input.as_str().trim() {
            "y" => return,
            "n" => exit(0),
            _ => continue,
        }
    }
}

fn player_won_menu(_g: &mut Game) {
    println!("Oh wow, you have won, congrats, game is over");
}

fn player_lost_menu(_g: &mut Game) {
    println!("HAH, You have lost and the game is over now");

}

fn start_menu() -> Player {

    let range: i64;
    let speed: i64;
    let damage: i64;
    let hp = 1000;
    let luck = 1.0;
    let item_cap = 5;
    print!("=====================\n");

    println!("|| Welcome to dungeon crawler!\n|| Pick your starting weapon!\n|| (R)ange, (S)peed, (D)amage\n||");
    println!("|    Bow & Quiver (1)    |    Sword (2)    |    Great Hammer (3)    |");
    println!("|    R1000 S500 D250     | R150 S750 D450  |    R200 S100 D1000     |");

    let mut weapon = String::new();
    let mut name = String::new();

    loop {
        weapon.clear();

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
    print!("Pleasure to meet you {}!\nI am Ahrnam, crawler and a fellow crawler, like you\nYes, I see a hunter's glint in your eyes. It's no easy thing finding one's way in the dark.\nI'm rambling... If we meet down there, I might have some things to trade you. Toodle-oo!\n\n", player.get_name());

    print!("=====================\n{}\n=====================\n", player);
    player
}
