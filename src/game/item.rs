

pub struct Potion {
    name: String,
    desc: String,

    max_health: i64,
    luck: f64,
    speed: i64,
}

impl Potion {

    pub fn new(name: String, desc: String, max_health: i64, luck: f64, speed: i64) -> Self {
        
        Potion {
            name: name,
            desc: desc,
            max_health: max_health,
            luck: luck,
            speed: speed,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_desc(&self) -> String {
        self.desc.clone()
    }
}

pub fn initiate_potion_options() -> Vec<Potion> {

    vec![
        Potion::new(String::from("Max health"), String::from("Increases max health"), 300, 0.0, 0),
        Potion::new(String::from("Speed"), String::from("Increases speed"), 0, 0.0, 25),
        Potion::new(String::from("Luck"), String::from("Increases luck"), 300, 0.25, 0),
    ]
}