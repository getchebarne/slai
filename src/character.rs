use crate::cards::get_card;
use crate::consts::CARD_REWARD_ROLL_OFFSET_BASE;
use crate::consts::STARTING_GOLD;
use crate::entity::ENTITY_ZERO;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::types::CardName;
use crate::types::Vitals;

pub fn spawn_silent(ascension: u8) -> Entity {
    let (health, health_max) = silent_health(ascension);
    make_entity_character(
        "Silent",
        Vitals {
            health,
            health_max,
            block: 0,
        },
        CARD_REWARD_ROLL_OFFSET_BASE,
        STARTING_GOLD,
    )
}

pub fn get_silent_starter_deck(ascension: u8) -> Vec<Entity> {
    let mut deck = vec![
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
    ];
    if ascension >= 10 {
        deck.push(get_card(CardName::AscendersBane, false));
    }
    deck
}

fn silent_health(ascension: u8) -> (u16, u16) {
    let mut health_max: u16 = 70;
    let mut health: u16 = health_max;

    if ascension >= 14 {
        health_max -= 4;
        health = health_max;
    }
    if ascension >= 6 {
        health = (0.90 * health as f32).round() as u16;
    }

    (health, health_max)
}

// Constructors
pub const fn make_entity_character(
    name: &'static str,
    vitals: Vitals,
    character_reward_roll_offset: i8,
    character_gold: u16,
) -> Entity {
    Entity {
        kind: EntityKind::Character,
        vitals,
        character_name: name,
        character_reward_roll_offset,
        character_gold,
        ..ENTITY_ZERO
    }
}
