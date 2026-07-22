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
use crate::entity::make_entity_event_option;
use crate::game::GameState;
use crate::types::EventName;
use crate::types::Mode;
use crate::utils::card_is_non_basic_non_curse;
use crate::utils::card_is_purgeable;
use crate::utils::card_is_upgradable;
use crate::utils::push_entity;

pub const EVENT_CONSUME_EFFECT: Effect = Effect {
    kind: EffectKind::EventConsume,
    id_source: None,
    target: Target::Direct(None),
};

#[derive(Debug, Clone, Copy)]
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
        found_gold: bool,
        found_nothing: bool,
        found_relic: bool,
        searches: u8,
    },
}

// Spawns `name` as the active mode; entry rolls land in the payload
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
        EventName::DeadAdventurer => EventPayload::DeadAdventurer {
            found_gold: false,
            found_nothing: false,
            found_relic: false,
            searches: 0,
        },
    };
    let id_options = bake_event_options(state, payload);
    state.mode = Mode::Event {
        name,
        payload,
        consumed: false,
        id_options,
    };
}

// One Entity per option, baked once at spawn from the (label, effects) table
pub fn bake_options(state: &mut GameState, options: &[(&'static str, &[Effect])]) -> Vec<usize> {
    let mut id_options = Vec::with_capacity(options.len());
    for &(label, effects) in options {
        let entity = make_entity_event_option(label, effects);
        id_options.push(push_entity(&mut state.entities, entity));
    }
    id_options
}

// Option lists encode only spawn-time state; mid-event dynamism lives in processors
fn bake_event_options(state: &mut GameState, payload: EventPayload) -> Vec<usize> {
    let ascension = state.ascension;
    match payload {
        EventPayload::BigFish => bake_options(state, big_fish::OPTIONS),
        EventPayload::TheCleric => bake_options(state, the_cleric::options(ascension)),
        EventPayload::Duplicator => bake_options(state, duplicator::OPTIONS),
        EventPayload::GoldenShrine => bake_options(state, golden_shrine::options(ascension)),
        EventPayload::WingStatue => bake_options(state, wing_statue::OPTIONS),
        EventPayload::WorldOfGoop => bake_options(state, world_of_goop::options(ascension)),
        EventPayload::LivingWall => bake_options(state, living_wall::OPTIONS),
        EventPayload::Purifier => bake_options(state, purifier::OPTIONS),
        EventPayload::ShiningLight => bake_options(state, shining_light::options(ascension)),
        EventPayload::TheSsssserpent => bake_options(state, the_ssssserpent::options(ascension)),
        EventPayload::Transmogrifier => bake_options(state, transmogrifier::OPTIONS),
        EventPayload::UpgradeShrine => bake_options(state, upgrade_shrine::OPTIONS),
        EventPayload::TheDivineFountain => bake_options(state, the_divine_fountain::OPTIONS),
        EventPayload::TheLab => bake_options(state, the_lab::options(ascension)),
        EventPayload::TheWomanInBlue => bake_options(state, the_woman_in_blue::options(ascension)),
        EventPayload::WheelOfChange => bake_options(state, wheel_of_change::OPTIONS),
        EventPayload::BonfireSpirits => bake_options(state, bonfire_spirits::OPTIONS),
        EventPayload::OminousForge => bake_options(state, ominous_forge::OPTIONS),
        EventPayload::FaceTrader => bake_options(state, face_trader::options(ascension)),
        EventPayload::Mushrooms => bake_options(state, mushrooms::OPTIONS),
        EventPayload::GoldenIdol { .. } => bake_options(state, golden_idol::options(ascension)),
        EventPayload::ScrapOoze { .. } => bake_options(state, scrap_ooze::options(ascension)),
        EventPayload::WeMeetAgain {
            id_card,
            id_potion,
            gold_ask,
        } => we_meet_again::bake(state, id_card, id_potion, gold_ask),
        EventPayload::DeadAdventurer { .. } => bake_options(state, dead_adventurer::OPTIONS),
    }
}

// Per-event availability checks
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

// Shared deck-scan predicates

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
    state
        .id_deck
        .iter()
        .any(|&id| card_is_non_basic_non_curse(&state.entities[id]))
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
