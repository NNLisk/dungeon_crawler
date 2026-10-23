use std::fmt;

use crate::game::game_instance::PlayerError;
use crate::logger::logger::{LogLevel, Logger};

use super::item::Potion;
use super::enemy::Enemy;

pub struct Player {
    name: String,
    hp: i64,
    max_hp: i64,
    damage: i64,
    speed: i64,
    luck: f64,
    range: i64,
    item_capacity: usize,
    items: Vec<Potion>,
    level: i64,
    xp: i64,
    alive: bool,
    
    log: Logger,
} 

impl Player {

    pub fn new(name: String, hp: i64, range: i64, speed: i64, damage: i64, luck: f64, item_capacity: usize) -> Self {
        Player {
            name,
            hp,
            max_hp: hp,
            damage,
            speed, 
            luck, 
            range, 
            item_capacity, 
            items: vec![], 
            level: 0, 
            xp: 0,
            alive: true,

            log: Logger::new(LogLevel::Info, "Player"),
        }
    }

    pub fn print_items(&self) {
        for item in &self.items {
            println!("Name: {}, Description: {}", item.get_name(), item.get_desc());
        }
    }


    pub fn heal(&mut self, potions: i64) {

        if potions > 0 {
            _ = self.change_hp(200, false);
            return;
        }

        self.log.info("No Potions Left!");

    }

    pub fn attack(&self, enemy: &mut Enemy) -> Result<(), PlayerError> {

        // same as enemy, luck and speed change attack, and cahnce to miss
        let base = 0.85;
        let speed_factor = (self.speed - enemy.get_speed()) as f64 * 0.005;
        let luck_factor = self.luck as f64 * 0.02;

        let final_hit_chance = (base + speed_factor + luck_factor).clamp(0.50, 0.98);
        let roll: f64 = rand::random();

        if roll < final_hit_chance {

            self.log.info(format!("Enemy hit for {}", self.damage));
            self.log.debug(enemy.get_hp());
            return enemy.change_hp(self.damage*-1);
        } else {
            self.log.info("Miss!");
            Ok(())
        }
    }

    pub fn add_potion(&mut self, p: Potion) {
        if self.items.len() < self.item_capacity {
            self.items.push(p);
            return;
        }

        println!("Items are full!");
    }

    // GETTERS AND SETTERS

    pub fn get_name(&self) -> &str {
        self.name.trim()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_hp(&self, max: bool) {
        match max {
            true => self.max_hp,
            false => self.hp,
        };
    }

    pub fn change_hp(&mut self, change: i64, max: bool) -> Result<(), PlayerError> {
        match max {
            true => self.max_hp += change,
            false => self.hp += change,
        };

        if self.hp <= 0 {
            self.hp = 0;
            Err(PlayerError::Dead)
        } else {
            Ok(())
        }
    }

    pub fn restore_hp(&mut self) {
        self.hp = self.max_hp;
    }

    pub fn use_potion(&mut self, choice: usize) {
        let potion = self.items.remove(choice-1);

        if potion.get_max_hp() > 0 {
            self.log.info("Max health increased!");
        }
        if potion.get_damage() > 0 {
            self.log.info("Damage increased!");
        }
        if potion.get_luck() > 0.0 {
            self.log.info("Luck increased!");
        }
        if potion.get_speed() > 0 {
            self.log.info("Speed increased!");
        }
        

        self.max_hp += potion.get_max_hp();
        self.speed += potion.get_speed();
        self.damage += potion.get_damage();
        self.luck += potion.get_luck();
    }


    pub fn get_dmg(&self) -> i64 {
        self.damage
    }

    pub fn change_dmg(&mut self, change: i64) {
        self.damage += change;
    }


    pub fn get_level(&self) -> i64 {
        self.level
    }

    pub fn get_next_required_xp(&self) -> i64 {
        // next required xp i used geometric sequence
        let base: f64 = 500.0;
        let increase_fact: f64 = 1.05;

        let n = (self.level + 1) as i32;

        // sum of xp until players level
        (base * (increase_fact.powi(n) - 1.0) / (increase_fact - 1.0)) as i64
    }

    pub fn get_xp(&self) -> i64 {
        self.xp
    }


    pub fn get_luck(&self) -> f64 {
        self.luck
    }

    pub fn change_luck(&mut self, change: f64) {
        self.luck += change;
    }

    pub fn get_range(&self) -> i64 {
        self.range
    }
    pub fn change_range(&mut self, change: i64) {
        self.range += change;
    }

    pub fn get_item_capacity(&self) -> usize {
        self.item_capacity
    }

    pub fn get_item_count(&mut self) -> usize {
        self.items.len()
    }

    pub fn add_xp(&mut self, xp: i64) {
        let required_xp = self.get_next_required_xp();
        self.xp += xp;

        if self.xp > required_xp {
            self.level_up();
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.max_hp = (self.max_hp as f64 * 1.02) as i64;
        self.damage = (self.damage as f64 * 1.02) as i64;
        self.range = (self.range as f64 * 1.02) as i64;
        self.speed = (self.speed as f64 * 1.02) as i64;

        self.log.info(format!("Level Up! Reached level {}", self.level));
    }


    pub fn get_speed(&self) -> i64 {
        self.speed
    }

    pub fn change_speed(&mut self, change: i64) {
        self.speed += change;
    }

    pub fn set_alive(&mut self, b: bool) {
        self.alive = b;
    }

    pub fn get_alive(&mut self) -> bool {
        self.alive
    }

}


// dont know if some sources are needed but this from here
// https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = write!(f, 
            "[{}],\nHP: {}/{}\nDMG {}, RNG {}, SPD {}\nXP: {}, Until next level: {}", 
            self.get_name(),
            self.hp, self.max_hp,
            self.get_dmg(), self.get_range(), self.get_speed(),
            self.get_xp(), self.get_next_required_xp() - self.get_xp());
        r
    }
}
