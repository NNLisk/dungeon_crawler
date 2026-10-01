mod game;

use std::{io};

use game::{player::Player, game_instance::Game};




fn main() {

    let mut player = start_menu();
    
    let mut game = Game::new(player);

}


fn start_menu() -> Player {
    
    let mut player = Player::new(String::from("samuel"), 100);

    
    println!("Welcome to dungeon crawler!\nPick your starting weapon!\n (R)ange, (S)peed, (D)amage");
    println!("|    Bow & Quiver (1)    |    Sword (2)    |    Great Hammer (3)    |");
    println!("|    R1000 S500 D250     | R150 S750 D450  |    R200 S100 D1000     |");

    let mut weapon = String::new();

    loop {

        io::stdin()
        .read_line(&mut weapon)
        .expect("InputError: player name");

        let choice: i32 = weapon.trim().parse().expect("ParsingError");
        
        match choice {
            1 => {
                player.change_range(100);
                player.change_speed(50);
                player.change_dmg(25);
            },
            2 => {
                player.change_range(15);
                player.change_speed(75);
                player.change_dmg(45);
            },
            3 => {
                player.change_range(20);
                player.change_speed(10);
                player.change_dmg(100);
            },
            _ => continue
        };
    }


    
}