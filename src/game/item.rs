#[derive(Clone)]
pub enum PotionTypes {
    MaxHP,
    Damage,
    Speed,
    Luck,
}
#[derive(Clone)]
pub struct Potion {
    t: PotionTypes,
    name: String,
    desc: String,

    max_health: i64,
    luck: f64,
    speed: i64,
    damage: i64,
}

impl Potion {

    pub fn new(t: PotionTypes, name: String, desc: String, max_health: i64, luck: f64, speed: i64, damage: i64) -> Self {
        
        Potion {
            t: t,
            name: name,
            desc: desc,
            max_health: max_health,
            luck: luck,
            speed: speed,
            damage: damage,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_desc(&self) -> String {
        self.desc.clone()
    }

    pub fn get_max_hp(&self) -> i64 {
        self.max_health
    }

    pub fn get_speed(&self) -> i64 {
        self.speed
    }

    pub fn get_damage(&self) -> i64 {
        self.damage
    }

    pub fn get_luck(&self) -> f64 {
        self.luck
    }
}

pub fn initiate_potion_options() -> Vec<Potion> {

    vec![
        Potion::new(PotionTypes::MaxHP, String::from("Maximum health"), String::from("Increases max health"), 300, 0.0, 0, 0),
        Potion::new(PotionTypes::Speed, String::from("Maximum health"), String::from("Increases speed"), 0, 0.0, 25, 0),
        Potion::new(PotionTypes::Luck, String::from("Maximum health"), String::from("Increases luck"), 300, 0.25, 0, 0),
        Potion::new(PotionTypes::Damage, String::from("Maximum health"), String::from("Increases damage"), 300, 0.25, 0, 200),

    ]
}