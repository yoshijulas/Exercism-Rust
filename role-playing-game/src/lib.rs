// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
// #![allow(unused)]

// I need to stop using if and start using match more!!!

use std::cmp::min;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub const fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Self {
                health: 100,
                mana: if self.level >= 10 { Some(100) } else { None },
                level: self.level,
            }),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) if mana < mana_cost => 0,
            Some(mana) => {
                self.mana = Some(mana - mana_cost);
                mana_cost * 2
            }
            None => {
                self.health -= min(self.health, mana_cost);
                0
            }
        }
    }
}
