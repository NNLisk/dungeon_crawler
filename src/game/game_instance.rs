use rand::seq::IndexedRandom;
use std::time::Duration;
use std::{io, thread};
use std::sync::{mpsc};

use crate::game::{player::Player};
use crate::game::enemy::{self, Enemy, initiate_enemy_types};
use crate::logger::logger::{Logger, LogLevel};

enum Events {
    EnemyHit,
    PlayerHit,
    PlayerHeal,
    PlayerShield,
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

    pub fn get_enemy(&self) -> &Enemy {
        let mut rng = rand::rng();
        &self.enemies.choose(&mut rng).expect("ChooseFail")
    }

    pub fn fight(&self) {

        let enemy = self.get_enemy().clone();
        let p = self.get_player_ref();
        let mut input = String::new();

        let (tx, rx) = mpsc::channel();
        let tx_enemy = tx.clone();
        let tx_player = tx.clone();

        let mut shield = false;

        println!("======== Your turn ========");
        println!("1) Attack (27) | 2) Heal | 3) shield (10)");

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(3));
            if tx_enemy.send(Events::EnemyHit).is_err() {
                break;
            }
        });

        thread::spawn(move || loop {
            
            input.clear();

            if let Ok(i) = input.trim().parse::<i64>() {
                match i {
                    1 => {
                        if tx_player.send(Events::PlayerHit).is_err() {
                            break;
                        }
                    }, 
                    2 => {
                        if tx_player.send(Events::PlayerHit).is_err() {
                            break;
                        }
                    },
                    3 => {
                        if tx_player.send(Events::PlayerHit).is_err() {
                            break;
                        }
                    }
                    _ => continue,
                }
            }
        });

        loop {
            if let Ok(msg) = rx.recv() {
                match msg {
                    Events::EnemyHit => enemy.attack(p),
                    Events::PlayerHit => p.attack(enemy),
                    Events::PlayerHeal => p.heal(),
                    Events::PlayerShield => shield = true,
                };
            }
        }
        
    }
}
