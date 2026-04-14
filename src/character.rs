use crate::cards::get_card;
use crate::consts::CARD_REWARD_ROLL_OFFSET_BASE;
use crate::entities::{Card, Character};
use crate::modifier::modifiers_new;
use crate::types::CardName;
use crate::types::Vitals;

pub fn spawn_silent(ascension: u8) -> Character {
    let (health, health_max) = silent_health(ascension);
    Character {
        name: "Silent",
        vitals: Vitals {
            health,
            health_max,
            block: 0,
        },
        modifiers: modifiers_new(),
        reward_roll_offset: CARD_REWARD_ROLL_OFFSET_BASE,
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
