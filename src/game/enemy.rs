use std::ops::DerefMut;

use crate::{game::game_instance::PlayerError, logger::logger::{LogLevel, Logger}};

use super::player::Player;



#[derive(Clone, Debug)]
pub struct Enemy {
    name: String,
    hp: i64,
    damage: i64,
    range: i64,
    speed: i64,
    xp_drop: i64,
    log: Logger
}

impl Enemy {
    pub fn new(name: String, hp: i64, damage: i64, range: i64, speed: i64, xp_drop: i64) -> Self {
        Enemy { name, hp, damage, range, speed, xp_drop, log: Logger::new(LogLevel::Info, "ENEMY")}
    }

    pub fn attack(&self, p: &mut Player, shield: bool) -> Result<(), PlayerError> {

        // thought it would be fun if there was speed comparison
        // and player luck would affect the hit too
        // dont know if it will end up balanced

        // shield mechanic, passed in battle, reduces dmg by x%
        let base = 0.8;
        let speed_factor = (self.speed - p.get_speed()) as f64 * 0.005;
        let p_luck = p.get_luck() as f64 * 0.02;

        let final_hit_chance = (base + speed_factor - p_luck).clamp(0.40, 0.95);

        let roll: f64 = rand::random();

        let damage = match shield {
            true => (self.damage as f64 * -1.0 * 0.60) as i64,
            false => self.damage * -1
        };

        if roll >= final_hit_chance {
            println!("[{}] hits you with for {} DMG", self.name, damage);
            return p.change_hp(damage, false);
        }

        self.log.info("Miss!");
        Ok(())
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_hp(&self) -> i64 {
        self.hp
    }
    
    pub fn change_hp(&mut self, change: i64) -> Result<(), PlayerError> {
        self.hp += change;

        if self.hp <= 0 {
            self.hp = 0;
            Err(PlayerError::Dead)
        } else {
            Ok(())
        }
    }

    pub fn get_speed(&self) -> i64 {
        self.speed
    }

    pub fn adjust_to_player_level(&mut self, floor: i8, room: i8) -> &mut Self {
        
        // 20% stat mult per playerlevel
        let level_mult = 1.0 + (floor -1) as f64 + ((room -1) as f64 * 0.30);
        self.hp = (self.hp as f64 * level_mult) as i64;
        self.damage = (self.damage as f64 * level_mult) as i64;
        self.speed = (self.speed as f64 * level_mult) as i64;

        self
    }

}

pub fn initiate_enemy_types() -> Vec<Enemy> {

    let balrog = Enemy::new(
        String::from("Balrog"),
        2500,
        320,
        250,
        100,
        450
    );

    vec![balrog]
}
