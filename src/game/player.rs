use std::fmt;

use super::item::Item;

pub struct Player {
    name: String,
    hp: i64,
    max_hp: i64,
    damage: i64,
    speed: i64,
    luck: i64,
    range: i64,
    item_capacity: i64,
    items: Vec<Item>,
    level: i64,
    xp: i64,
    killcount: i64,
    boss_killcount: i64,
} 

impl Player {

    pub fn new(name: String, hp: i64, range: i64, speed: i64, damage: i64, luck: i64, item_capacity: i64) -> Self {
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
            killcount: 0, 
            boss_killcount: 0, 
        }
    }

    pub fn print_items(&self) {
        for item in &self.items {
            println!("Name: {}, Description: {}", item.get_name(), item.get_desc());
        }
    }

    // GETTERS AND SETTERS

    pub fn get_name(&self) -> String {
        self.name.clone()
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

    pub fn change_hp(&mut self, change: i64, max: bool) {
        match max {
            true => self.max_hp += change,
            false => self.hp += change,
        };
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

    pub fn change_xp(&mut self, change: i64) {
        self.xp += change;
    }

    pub fn get_killcount(&self, boss: bool) {
        match boss {
            true => self.boss_killcount,
            false => self.killcount,
        };
    }

    pub fn up_killcount(&mut self, boss: bool) {
        match boss {
            true => {
                self.boss_killcount += 1;
                self.killcount += 1;
            },
            false => self.killcount += 1,
        }
    }

    pub fn get_luck(&self) -> i64 {
        self.luck
    }

    pub fn change_luck(&mut self, change: i64) {
        self.luck += change;
    }

    pub fn get_range(&self) -> i64 {
        self.range
    }
    pub fn change_range(&mut self, change: i64) {
        self.range += change;
    }

    pub fn get_item_capacity(&self) -> i64 {
        self.item_capacity
    }

    pub fn change_item_cap(&mut self, change: i64) {
        self.item_capacity += change;
    }

    pub fn get_speed(&self) -> i64 {
        self.speed
    }

    pub fn change_speed(&mut self, change: i64) {
        self.speed += change;
    }

}