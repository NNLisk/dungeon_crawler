use rand::seq::{IndexedRandom};
use std::time::Duration;
use std::{io, thread, unreachable};
use std::sync::{mpsc};

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
    log: Logger,
}

impl Game {

    pub fn new(p: Player) -> Self {
        Game {
            player: p,
            floor: 1,
            room: 1,
            enemies: initiate_enemy_types(),
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

    pub fn fight(&mut self) -> Result<(), PlayerError> {

        let mut enemy = self.get_enemy();
        let mut input = String::new();

        let (tx, rx) = mpsc::channel();
        let tx_enemy = tx.clone();
        let tx_player = tx.clone();

        let mut shield = false;

        println!("======== Your turn ========");
        println!("1) Attack | 2) Heal | 3) shield ");

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
                "SLAM" => {
                    if tx_player.send(Events::PlayerHit).is_err() {
                        break;
                    }
                }, 
                "HEAL" => {
                    if tx_player.send(Events::PlayerHeal).is_err() {
                        break;
                    }
                },
                "SHIELD" => {
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
                    Events::PlayerHeal => self.player.heal(),
                    Events::PlayerShield => shield = true,
                };
            }
        }

        
    }

    pub fn set_floor(&mut self, f: i8) {

        match f {
            0..=2 => {
                self.floor = f;
                self.log.info(format!("Floor set to {f}"));
            },
            3.. => self.floor = 0,
            _ => unreachable!()
        }


    }

    pub fn get_floor(&self) -> i8 {
        self.floor
    }

    pub fn set_room(&mut self, r: i8) {
        
        match r {
            0..=2 => {
                self.room = r;
                self.log.info(format!("Room set to {r}"));
            },
            3.. => self.room = 0,
            _ => unreachable!()
        }
    }

    pub fn get_room(&self) -> i8 {
        self.room
    }


}
