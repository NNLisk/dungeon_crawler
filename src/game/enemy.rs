use super::player::Player;

use rand::Rng;

pub enum PlayerError {
    Dead,
}
pub struct Enemy {
    name: String,
    hp: i64,
    damage: i64,
    range: i64,
    speed: i64,
    xp_drop: i64,
}

impl Enemy {
    pub fn new(name: String, hp: i64, damage: i64, range: i64, speed: i64, xp_drop: i64) -> Self {
        Enemy { name, hp, damage, range, speed, xp_drop }
    }

    pub fn attack(&self, p: &mut Player) {

        // thought it would be fun if there was speed comparison
        // and player luck would affect the hit too
        // dont know if it will end up balanced
        let base = 0.8;
        let speed_factor = (self.speed - p.get_speed()) as f64 * 0.005;
        let p_luck = p.get_luck() as f64 * 0.02;

        let final_hit_chance = (base + speed_factor - p_luck).clamp(0.40, 0.95);

        let roll: f64 = rand::random();

        if roll >= final_hit_chance {
            println!("[{}] hits you with for {} DMG", self.name, self.damage);
            p.change_hp(self.damage*-1, false);
        }
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