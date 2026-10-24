#[derive(Clone)]
pub enum PotionTypes {
    MaxHP,
    Damage,
    Speed,
    Luck,
}
#[derive(Clone)]
pub struct Potion {
    _t: PotionTypes,
    name: String,
    desc: String,
    cost: i64,

    max_health: i64,
    luck: f64,
    speed: i64,
    damage: i64,
}

impl Potion {

    pub fn new(_t: PotionTypes, name: String, desc: String, cost: i64, max_health: i64, luck: f64, speed: i64, damage: i64) -> Self {
        
        Potion {
            _t,
            name,
            desc,
            cost,
            max_health,
            luck,
            speed,
            damage,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_desc(&self) -> String {
        self.desc.clone()
    }

    pub fn get_cost(&self) -> i64 {
        self.cost
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
        Potion::new(PotionTypes::MaxHP, String::from("Maximum health"), String::from("Increases max health"), 4, 300, 0.0, 0, 0),
        Potion::new(PotionTypes::Speed, String::from("Suspicious lotion"), String::from("Rubbing in your feet makes you faster"), 2, 0, 0.0, 25, 0),
        Potion::new(PotionTypes::Luck, String::from("Lucky Charm"), String::from("Makes you feel luckier"), 300, 2, 0.25, 0, 0),
        Potion::new(PotionTypes::Damage, String::from("Whetstone"), String::from("Sharpens the weapons"), 300, 3, 0.25, 0, 200),

    ]
}