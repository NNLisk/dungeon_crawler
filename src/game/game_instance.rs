use rand::seq::{IndexedRandom};
use std::io::Write;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::time::{Duration, Instant};
use std::{io, thread, unreachable};
use std::sync::{mpsc};

use crate::game::item::{Potion, initiate_potion_options};
use crate::game::{player::Player};
use crate::game::enemy::{Enemy, initiate_boss_types, initiate_enemy_types};
use crate::logger::logger::{Logger, LogLevel};

enum Events {
    EnemyHit,
    Line(String),
}

pub enum PlayerError {
    Dead,
}
pub struct Game {
    player: Player,
    floor: i8,
    room: i8,
    enemies: Vec<Enemy>,
    bosses: Vec<Enemy>,
    shop_items: Vec<Potion>,
    tx: Sender<Events>,
    rx: Receiver<Events>,
    log: Logger,
}

impl Game {

    pub fn new(p: Player) -> Self {

        let (tx, rx) = mpsc::channel();
        let tx_input = tx.clone();
        thread::spawn(move || {
            let stdin = io::stdin();
            loop {
                let mut line = String::new();
                if stdin.read_line(&mut line).is_err()
                    || tx_input.send(Events::Line(line)).is_err()
                {
                    break;
                }
            }
        });

        Game {
            player: p,
            floor: 1,
            room: 1,
            enemies: initiate_enemy_types(),
            bosses: initiate_boss_types(),
            shop_items: initiate_potion_options(),
            tx,
            rx,
            log: Logger::new(LogLevel::Debug, "Game"),
        }
    }

    pub fn read_input(&self) -> String {
        loop {
            match self.rx.recv().expect("ChannelDied") {
                Events::Line(s) => return s.trim().to_string(),
                Events::EnemyHit => continue,
            }
        }
    }

    pub fn get_player_ref(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn get_enemy(&self, boss: bool) -> Enemy {
        let mut rng = rand::rng();

        let mut enemy = match boss {
            true => {
                self.bosses.choose(&mut rng).expect("ChooseError").clone()

            },
            false => {
                self.enemies.choose(&mut rng).expect("ChooseError").clone()
            }
        };
        
        println!("#############################################################");
        self.log.info(format!("You walk into a room and find a {}", enemy.get_name()));
        println!("#############################################################");


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

    pub fn fight(&mut self, boss: bool) -> Result<(), PlayerError> {

        let mut enemy = self.get_enemy(boss);

        let mut shield = false;
        let mut player_potions = 3;

        self.player.restore_hp();      

        potion_fight_menu(self);  

        while self.rx.try_recv().is_ok() {}
        
        println!("======== Your turn ========");
        println!("(hit) Attack | (heal) Heal | (shield) shield ");
        

        let (stop_tx, stop_rx) = mpsc::channel::<()>();
        let tx_enemy = self.tx.clone();

        thread::spawn(move || loop {
            match stop_rx.recv_timeout(Duration::from_secs(3)) {
                Err(RecvTimeoutError::Timeout) => {
                    if tx_enemy.send(Events::EnemyHit).is_err() {
                        break;
                    }
                }
                _ => break, // stop signal or sender dropped
            }
        });

        loop {
            match self.rx.recv().expect("ChannelDied") {
                Events::EnemyHit => {
                    if enemy.attack(&mut self.player, shield).is_err() {
                        self.log.info("Player died");
                        return Err(PlayerError::Dead);
                    }
                }
                Events::Line(line) => match line.trim() {
                    "hit" | "slam" => {
                        if self.player.attack(&mut enemy).is_err() {
                            self.log.info("Enemy has died");
                            self.player.add_xp(enemy.get_xp_drop());
                            return Ok(());
                        }
                    }
                    "heal" if player_potions > 0 => {
                        self.player.heal(player_potions);
                        player_potions -= 1;
                    }
                    "heal" => println!("No heals left!"),
                    "shield" => shield = true,
                    _ => {}
                },
            }
        }        
    }


    pub fn shop(&mut self) {

        let mut index = 1;

        println!("Hello there! Remember me? Arnham, the crawler! I have some items, I can give you one if you'd like?");

        for item in &self.shop_items {
            println!("{}) {} || {}", index, item.get_name(), item.get_desc());
            index += 1;
        }

        println!("Which one would you like?");
        
        io::stdout().flush().expect("Failed to flush stdout");

        let input = self.read_input();

        self.log.debug("Input read");

        let choice = input.trim().parse().expect("ParsingError");
        self.log.debug("Input parsed");

        match choice {
            1..=4 => {
                self.player.add_potion(self.shop_items[choice-1].clone());
                self.log.info("Potion selected");
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

    let player_item_count = g.get_player_ref().get_item_count();


    println!("======== Potions ========");
    println!("You may use a potion before the fight (0 to skip)");
    g.player.print_items();


    let input = g.read_input();

    let choice: usize = input.trim().parse().expect("ParsingError");

    if choice > 0 && choice < player_item_count {
        g.player.use_potion(choice)
    }
}
