use super::item::Item;

pub struct Player {
    name: String,
    hp: i64,
    max_hp: i64,
    speed: i64,
    luck: i64,
    range: i64,
    item_capacity: i64,
    items: Vec<Item>,
    level: i64,
    xp: i64,
    killcount: i64,
    boss_killcount: i64,
    dungeon_floor: i8,
} 

impl Player {

    pub fn new(name: String, max_hp: i64) -> Self {
        Player { 
            name: name, 
            hp: max_hp, 
            max_hp: max_hp, 
            speed: 0, 
            luck: 0, 
            range: 0, 
            item_capacity: 0, 
            items: vec![], 
            level: 0, 
            xp: 0, 
            killcount: 0, 
            boss_killcount: 0, 
            dungeon_floor: 0
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


    pub fn get_level(&self) -> i64 {
        self.level
    }

    pub fn get_xp(&self) -> i64 {
        self.xp
    }

    pub fn get_killcount(&self, boss: bool) {
        match boss {
            true => self.boss_killcount,
            false => self.killcount,
        };
    }

    pub fn get_current_floor(&self) -> i8 {
        self.dungeon_floor
    }

    pub fn get_luck(&self) -> i64 {
        self.luck
    }

    pub fn get_range(&self) -> i64 {
        self.range
    }

    pub fn get_item_capacity(&self) -> i64 {
        self.item_capacity
    }

    pub fn get_speed(&self) -> i64 {
        self.speed
    }

}