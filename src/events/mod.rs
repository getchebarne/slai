mod big_fish;
mod bonfire_spirits;
mod dead_adventurer;
mod duplicator;
mod face_trader;
mod golden_idol;
mod golden_shrine;
mod living_wall;
mod mushrooms;
mod ominous_forge;
mod purifier;
mod scrap_ooze;
mod shining_light;
mod the_cleric;
mod the_divine_fountain;
mod the_lab;
mod the_ssssserpent;
mod the_woman_in_blue;
mod transmogrifier;
mod upgrade_shrine;
mod we_meet_again;
mod wheel_of_change;
mod wing_statue;
mod world_of_goop;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::types::CardKind;
use crate::types::CardRarity;
use crate::types::EventName;
use crate::types::MonsterEncounter;
use crate::utils::card_is_purgeable;
use crate::utils::card_is_upgradable;

pub const EVENT_CONSUME_EFFECT: Effect = Effect {
    kind: EffectKind::EventConsume,
    id_source: None,
    target: Target::Direct(None),
};

// The active event: typed rolled state per event, plus the consumed flag.
// Statics never hold entity ids; the ids here are runtime leaf data validated at use
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Event {
    pub name: EventName,
    pub payload: EventPayload,
    pub consumed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventPayload {
    BigFish,
    TheCleric,
    Duplicator,
    GoldenShrine,
    WingStatue,
    WorldOfGoop,
    LivingWall,
    Purifier,
    ShiningLight,
    TheSsssserpent,
    Transmogrifier,
    UpgradeShrine,
    TheDivineFountain,
    TheLab,
    TheWomanInBlue,
    WheelOfChange,
    BonfireSpirits,
    OminousForge,
    FaceTrader,
    Mushrooms,
    GoldenIdol {
        stage: u8,
    },
    ScrapOoze {
        attempts: u8,
    },
    WeMeetAgain {
        id_card: Option<usize>,
        id_potion: Option<usize>,
        gold_ask: Option<u16>,
    },
    DeadAdventurer {
        encounter: MonsterEncounter,
        rewards: [DeadAdventurerReward; 3],
        rewards_len: u8,
        searches: u8,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeadAdventurerReward {
    Gold,
    Nothing,
    Relic,
}

// Spawns `name` and registers it as the active event; entry rolls land in the payload
pub fn spawn_event(state: &mut GameState, name: EventName) {
    let payload = match name {
        EventName::BigFish => EventPayload::BigFish,
        EventName::TheCleric => EventPayload::TheCleric,
        EventName::Duplicator => EventPayload::Duplicator,
        EventName::GoldenShrine => EventPayload::GoldenShrine,
        EventName::GoldenIdol => EventPayload::GoldenIdol { stage: 0 },
        EventName::WingStatue => EventPayload::WingStatue,
        EventName::WorldOfGoop => EventPayload::WorldOfGoop,
        EventName::LivingWall => EventPayload::LivingWall,
        EventName::Purifier => EventPayload::Purifier,
        EventName::ScrapOoze => EventPayload::ScrapOoze { attempts: 0 },
        EventName::ShiningLight => EventPayload::ShiningLight,
        EventName::TheSsssserpent => EventPayload::TheSsssserpent,
        EventName::Transmogrifier => EventPayload::Transmogrifier,
        EventName::UpgradeShrine => EventPayload::UpgradeShrine,
        EventName::TheDivineFountain => EventPayload::TheDivineFountain,
        EventName::TheLab => EventPayload::TheLab,
        EventName::TheWomanInBlue => EventPayload::TheWomanInBlue,
        EventName::WheelOfChange => EventPayload::WheelOfChange,
        EventName::BonfireSpirits => EventPayload::BonfireSpirits,
        EventName::OminousForge => EventPayload::OminousForge,
        EventName::FaceTrader => EventPayload::FaceTrader,
        EventName::WeMeetAgain => we_meet_again::spawn_event_we_meet_again(state),
        EventName::Mushrooms => EventPayload::Mushrooms,
        EventName::DeadAdventurer => dead_adventurer::spawn_event_dead_adventurer(state),
    };
    state.event = Some(Event {
        name,
        payload,
        consumed: false,
    });
}

// Builds the concrete effects for option `idx`; rolled payloads bake their values in
pub fn push_event_option_effects(
    buf: &mut Vec<Effect>,
    payload: EventPayload,
    ascension: u8,
    idx: usize,
) {
    match payload {
        EventPayload::BigFish => big_fish::push_option_effects(buf, idx),
        EventPayload::TheCleric => the_cleric::push_option_effects(buf, ascension, idx),
        EventPayload::Duplicator => duplicator::push_option_effects(buf, idx),
        EventPayload::GoldenShrine => golden_shrine::push_option_effects(buf, ascension, idx),
        EventPayload::WingStatue => wing_statue::push_option_effects(buf, idx),
        EventPayload::WorldOfGoop => world_of_goop::push_option_effects(buf, ascension, idx),
        EventPayload::LivingWall => living_wall::push_option_effects(buf, idx),
        EventPayload::Purifier => purifier::push_option_effects(buf, idx),
        EventPayload::ShiningLight => shining_light::push_option_effects(buf, ascension, idx),
        EventPayload::TheSsssserpent => the_ssssserpent::push_option_effects(buf, ascension, idx),
        EventPayload::Transmogrifier => transmogrifier::push_option_effects(buf, idx),
        EventPayload::UpgradeShrine => upgrade_shrine::push_option_effects(buf, idx),
        EventPayload::TheDivineFountain => the_divine_fountain::push_option_effects(buf, idx),
        EventPayload::TheLab => the_lab::push_option_effects(buf, ascension, idx),
        EventPayload::TheWomanInBlue => the_woman_in_blue::push_option_effects(buf, ascension, idx),
        EventPayload::WheelOfChange => wheel_of_change::push_option_effects(buf, idx),
        EventPayload::BonfireSpirits => bonfire_spirits::push_option_effects(buf, idx),
        EventPayload::OminousForge => ominous_forge::push_option_effects(buf, idx),
        EventPayload::FaceTrader => face_trader::push_option_effects(buf, ascension, idx),
        EventPayload::Mushrooms => mushrooms::push_option_effects(buf, idx),
        EventPayload::GoldenIdol { .. } => golden_idol::push_option_effects(buf, ascension, idx),
        EventPayload::ScrapOoze { .. } => scrap_ooze::push_option_effects(buf, ascension, idx),
        EventPayload::WeMeetAgain {
            id_card,
            id_potion,
            gold_ask,
        } => we_meet_again::push_option_effects(buf, id_card, id_potion, gold_ask, idx),
        EventPayload::DeadAdventurer { .. } => dead_adventurer::push_option_effects(buf, idx),
    }
}

// Replaces the old gate interpreter: plain per-event availability checks
pub fn event_option_available(state: &GameState, payload: EventPayload, idx: usize) -> bool {
    match payload {
        EventPayload::BigFish
        | EventPayload::Duplicator
        | EventPayload::GoldenShrine
        | EventPayload::WorldOfGoop
        | EventPayload::TheSsssserpent
        | EventPayload::TheDivineFountain
        | EventPayload::TheLab
        | EventPayload::TheWomanInBlue
        | EventPayload::WheelOfChange
        | EventPayload::BonfireSpirits
        | EventPayload::FaceTrader
        | EventPayload::Mushrooms
        | EventPayload::DeadAdventurer { .. } => true,
        EventPayload::TheCleric => the_cleric::option_available(state, idx),
        EventPayload::WingStatue => wing_statue::option_available(state, idx),
        EventPayload::LivingWall => living_wall::option_available(state, idx),
        EventPayload::Purifier => purifier::option_available(state, idx),
        EventPayload::ShiningLight => shining_light::option_available(state, idx),
        EventPayload::Transmogrifier => transmogrifier::option_available(state, idx),
        EventPayload::UpgradeShrine => upgrade_shrine::option_available(state, idx),
        EventPayload::OminousForge => ominous_forge::option_available(state, idx),
        EventPayload::GoldenIdol { stage } => golden_idol::option_available(stage, idx),
        EventPayload::ScrapOoze { attempts } => scrap_ooze::option_available(attempts, idx),
        EventPayload::WeMeetAgain {
            id_card,
            id_potion,
            gold_ask,
        } => we_meet_again::option_available(state, id_card, id_potion, gold_ask, idx),
    }
}

pub fn event_option_labels(payload: EventPayload, ascension: u8) -> &'static [&'static str] {
    match payload {
        EventPayload::BigFish => big_fish::LABELS,
        EventPayload::TheCleric => the_cleric::labels(ascension),
        EventPayload::Duplicator => duplicator::LABELS,
        EventPayload::GoldenShrine => golden_shrine::labels(ascension),
        EventPayload::WingStatue => wing_statue::LABELS,
        EventPayload::WorldOfGoop => world_of_goop::labels(ascension),
        EventPayload::LivingWall => living_wall::LABELS,
        EventPayload::Purifier => purifier::LABELS,
        EventPayload::ShiningLight => shining_light::labels(ascension),
        EventPayload::TheSsssserpent => the_ssssserpent::labels(ascension),
        EventPayload::Transmogrifier => transmogrifier::LABELS,
        EventPayload::UpgradeShrine => upgrade_shrine::LABELS,
        EventPayload::TheDivineFountain => the_divine_fountain::LABELS,
        EventPayload::TheLab => the_lab::labels(ascension),
        EventPayload::TheWomanInBlue => the_woman_in_blue::labels(ascension),
        EventPayload::WheelOfChange => wheel_of_change::LABELS,
        EventPayload::BonfireSpirits => bonfire_spirits::LABELS,
        EventPayload::OminousForge => ominous_forge::LABELS,
        EventPayload::FaceTrader => face_trader::labels(ascension),
        EventPayload::Mushrooms => mushrooms::LABELS,
        EventPayload::GoldenIdol { .. } => golden_idol::labels(ascension),
        EventPayload::ScrapOoze { .. } => scrap_ooze::labels(ascension),
        EventPayload::WeMeetAgain { .. } => we_meet_again::LABELS,
        EventPayload::DeadAdventurer { .. } => dead_adventurer::LABELS,
    }
}

// Shared deck-scan predicates (the old EventGate arms as plain functions)

pub fn deck_has_upgradable(state: &GameState) -> bool {
    state
        .id_deck
        .iter()
        .any(|&id| card_is_upgradable(&state.entities[id]))
}

pub fn deck_has_purgeable(state: &GameState) -> bool {
    state
        .id_deck
        .iter()
        .any(|&id| card_is_purgeable(&state.entities[id]))
}

pub fn deck_has_non_basic_non_curse(state: &GameState) -> bool {
    state.id_deck.iter().any(|&id| {
        let entity = &state.entities[id];
        entity.card_rarity != CardRarity::Basic && entity.card_kind != CardKind::Curse
    })
}

pub fn deck_has_damage_card(state: &GameState, min_base: u16) -> bool {
    state
        .id_deck
        .iter()
        .any(|&id| card_has_damage_at_least(&state.entities[id], min_base))
}

fn card_has_damage_at_least(entity: &Entity, min_base: u16) -> bool {
    if entity.kind != EntityKind::Card {
        return false;
    }
    for effect in entity.card_effects[..entity.card_effects_len as usize].iter() {
        let amount = match effect.kind {
            EffectKind::DamagePhysical { amount } => amount,
            EffectKind::DamagePhysicalIfPoisoned { amount } => amount,
            _ => 0,
        };
        if amount >= min_base {
            return true;
        }
    }
    false
}

// Mushrooms and Dead Adventurer are draw-gated in `draw_event` (floor 7+)
pub const POOL_ACT1_EVENT: &[EventName] = &[
    EventName::BigFish,
    EventName::Mushrooms,
    EventName::DeadAdventurer,
    EventName::TheCleric,
    EventName::GoldenIdol,
    EventName::WingStatue,
    EventName::WorldOfGoop,
    EventName::LivingWall,
    EventName::ScrapOoze,
    EventName::ShiningLight,
    EventName::TheSsssserpent,
];

// Shrines and one-time specials roll together at EVENT_SPECIAL_CHANCE
pub const POOL_ACT1_EVENT_SPECIAL: &[EventName] = &[
    EventName::GoldenShrine,
    EventName::Purifier,
    EventName::Transmogrifier,
    EventName::UpgradeShrine,
    EventName::TheDivineFountain,
    EventName::TheLab,
    EventName::TheWomanInBlue,
    EventName::WheelOfChange,
    EventName::BonfireSpirits,
    EventName::OminousForge,
    EventName::FaceTrader,
    EventName::WeMeetAgain,
];
