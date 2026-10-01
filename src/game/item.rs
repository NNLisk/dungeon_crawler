

pub struct Item {
    name: String,
    desc: String,

    health: i8,
    max_health: i8,
    luck: i8,
    speed: i8,
    defence: i8,
}

impl Item {

    pub fn new(name: String, desc: String, health: i8, max_health: i8, luck: i8, speed: i8, defence: i8) -> Self {
        Item {
            name: name,
            desc: desc,
            health: health,
            max_health: max_health,
            luck: luck,
            speed: speed,
            defence: defence,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_desc(&self) -> String {
        self.desc.clone()
    }
}