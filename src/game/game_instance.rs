use crate::game::player::Player;

use super::player;

pub struct Game {
    player: player::Player,
    floor: i8,
    room: i8,
}

impl Game {

    pub fn new(p: Player) -> Self {
        Game {
            player: p,
            floor: 1,
            room: 1,
        }
    }

    pub fn get_player_ref(&mut self) -> &mut Player {
        &mut self.player
    }
}