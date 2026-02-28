use crate::cards::{Card, get_card};
use crate::modifier::{Modifiers, modifiers_new};
use crate::state::Vitals;
use crate::types::CardName;

#[derive(Debug, Clone, Copy)]
pub struct Character {
    pub vitals: Vitals,
    pub modifiers: Modifiers,
    pub reward_roll_offset: i8,
}

pub fn spawn_silent(ascension: u8) -> Character {
    let (health, health_max) = silent_health(ascension);
    Character {
        vitals: Vitals { health, health_max, block: 0 },
        modifiers: modifiers_new(),
        reward_roll_offset: 5,
    }
}

pub fn silent_starter_deck() -> Vec<Card> {
    vec![
        get_card(CardName::Strike, false),
        get_card(CardName::Strike, false),
        get_card(CardName::Strike, false),
        get_card(CardName::Strike, false),
        get_card(CardName::Strike, false),
        get_card(CardName::Defend, false),
        get_card(CardName::Defend, false),
        get_card(CardName::Defend, false),
        get_card(CardName::Defend, false),
        get_card(CardName::Defend, false),
        get_card(CardName::Survivor, false),
        get_card(CardName::Neutralize, false),
    ]
}

fn silent_health(ascension: u8) -> (u16, u16) {
    let mut health_max: u16 = 70;
    let mut health: u16 = health_max;

    if ascension >= 14 {
        health_max -= 4;
        health = health_max;
    }
    if ascension >= 6 {
        health = (0.90 * health as f32) as u16;
    }

    (health, health_max)
}
