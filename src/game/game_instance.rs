use rand::seq::{IndexedRandom};
use std::time::Duration;
use std::{io, thread, unreachable};
use std::sync::{mpsc};

use crate::game::item::{Potion, initiate_potion_options};
use crate::game::{player::Player};
use crate::game::enemy::{Enemy, initiate_enemy_types};
use crate::logger::logger::{Logger, LogLevel};

enum Events {
    EnemyHit,
    PlayerHit,
    PlayerHeal,
    PlayerShield,
}

pub enum PlayerError {
    Dead,
}
pub struct Game {
    player: Player,
    floor: i8,
    room: i8,
    enemies: Vec<Enemy>,
    shop_items: Vec<Potion>,
    log: Logger,
}

impl Game {

    pub fn new(p: Player) -> Self {
        Game {
            player: p,
            floor: 1,
            room: 1,
            enemies: initiate_enemy_types(),
            shop_items: initiate_potion_options(),
            log: Logger::new(LogLevel::Debug, "Game"),
        }
    }

    pub fn get_player_ref(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn get_enemy(&self) -> Enemy {
        let mut rng = rand::rng();
        let mut enemy = self.enemies.choose(&mut rng).expect("ChooseError").clone();


        enemy.adjust_to_player_level(self.floor, self.room);

        enemy
    }

    pub fn get_starting_potions(&mut self) {
        let mut rng = rand::rng();
        
        for _ in 0..self.player.get_item_capacity() - 2 {
            let potion = self.shop_items.choose(&mut rng).expect("ChooseError").clone();
            let _ = &mut self.player.add_potion(potion);
        };
    }

    pub fn fight(&mut self) -> Result<(), PlayerError> {

        let mut enemy = self.get_enemy();
        let mut input = String::new();

        let (tx, rx) = mpsc::channel();
        let tx_enemy = tx.clone();
        let tx_player = tx.clone();

        let mut shield = false;
        let mut player_potions = 3;

        
        

        println!("======== Your turn ========");
        println!("(hit) Attack | (heal) Heal | (shield) shield ");

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(3));
            if tx_enemy.send(Events::EnemyHit).is_err() {
                break;
            }
        });

        thread::spawn(move || loop {
            
            input.clear();

            io::stdin()
                .read_line(&mut input)
                .expect("InputError");

            match input.as_str().trim() {
                "slam" => {
                    if tx_player.send(Events::PlayerHit).is_err() {
                        break;
                    }
                }, 
                "heal" => {
                    if tx_player.send(Events::PlayerHeal).is_err() {
                        break;
                    }
                },
                "shield" => {
                    if tx_player.send(Events::PlayerShield).is_err() {
                        break;
                    }
                }
                _ => continue,
                
            }
        });

        loop {
            if let Ok(msg) = rx.recv() {
                match msg {
                    Events::EnemyHit => {
                        if enemy.attack(&mut self.player, shield).is_err() {
                            self.log.info("Player died");
                            return Err(PlayerError::Dead);
                        }
                    },
                    Events::PlayerHit => {
                        if self.player.attack(&mut enemy).is_err() {
                            self.log.info("Enemy has died");
                            return Ok(());
                        }
                    },
                    Events::PlayerHeal => {
                        self.player.heal(player_potions);
                        player_potions -= 1;
                    },
                    Events::PlayerShield => shield = true,
                };
            }
        }
        
    }


    pub fn shop(&mut self) {

        let mut input = String::new();

        println!("Hello there! Remember me? Arnham, the crawler! I have some items, I can give you one if you'd like?");

        for item in &self.shop_items {
            println!("{} \n || {}", item.get_name(), item.get_desc());
        }

        println!("Which one would you like?");

        io::stdin()
            .read_line(&mut input)
            .expect("InputError");

        let choice = input.trim().parse().expect("ParsingError");

        match choice {
            1..4 => {
                self.player.add_potion(self.shop_items[choice-1].clone());
                return;
            },
            _ => println!("Bad choice"),
        }

    }


    // 3 floors of 4 rooms!
    pub fn set_floor(&mut self, f: i8) {

        match f {
            0..=3 => {
                self.floor = f;
                self.log.info(format!("Floor set to {f}"));
            },
            4.. => self.floor = 0,
            _ => unreachable!()
        }


    }

    pub fn get_floor(&self) -> i8 {
        self.floor
    }

    pub fn set_room(&mut self, r: i8) {
        
        match r {
            0..=4 => {
                self.room = r;
                self.log.info(format!("Room set to {r}"));
            },
            5.. => self.room = 0,
            _ => unreachable!()
        }
    }

    pub fn get_room(&self) -> i8 {
        self.room
    }


}


pub fn potion_fight_menu(g: &mut Game) {

    let mut input = String::new();
    let player_item_count = g.get_player_ref().get_item_count();


    println!("======== Potions ========");
    g.player.print_items();

    println!("You may use one potion before the fight, which one?");

    io::stdin()
        .read_line(&mut input)
        .expect("InputError");

    let choice: usize = input.trim().parse().expect("ParsingError");

    if choice > 0 && choice < player_item_count {
        g.player.use_potion(choice)
    }
}
