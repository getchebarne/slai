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
use crate::types::EventName;
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
pub enum EventKind {
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

// Builds the event: entry rolls land in the kind, options bake into the arena
pub fn spawn_event(state: &mut GameState, name: EventName) -> (EventKind, Vec<usize>) {
    let kind = match name {
        EventName::BigFish => EventKind::BigFish,
        EventName::TheCleric => EventKind::TheCleric,
        EventName::Duplicator => EventKind::Duplicator,
        EventName::GoldenShrine => EventKind::GoldenShrine,
        EventName::GoldenIdol => EventKind::GoldenIdol { stage: 0 },
        EventName::WingStatue => EventKind::WingStatue,
        EventName::WorldOfGoop => EventKind::WorldOfGoop,
        EventName::LivingWall => EventKind::LivingWall,
        EventName::Purifier => EventKind::Purifier,
        EventName::ScrapOoze => EventKind::ScrapOoze { attempts: 0 },
        EventName::ShiningLight => EventKind::ShiningLight,
        EventName::TheSsssserpent => EventKind::TheSsssserpent,
        EventName::Transmogrifier => EventKind::Transmogrifier,
        EventName::UpgradeShrine => EventKind::UpgradeShrine,
        EventName::TheDivineFountain => EventKind::TheDivineFountain,
        EventName::TheLab => EventKind::TheLab,
        EventName::TheWomanInBlue => EventKind::TheWomanInBlue,
        EventName::WheelOfChange => EventKind::WheelOfChange,
        EventName::BonfireSpirits => EventKind::BonfireSpirits,
        EventName::OminousForge => EventKind::OminousForge,
        EventName::FaceTrader => EventKind::FaceTrader,
        EventName::WeMeetAgain => we_meet_again::spawn_event_we_meet_again(state),
        EventName::Mushrooms => EventKind::Mushrooms,
        EventName::DeadAdventurer => EventKind::DeadAdventurer {
            found_gold: false,
            found_nothing: false,
            found_relic: false,
            searches: 0,
        },
    };
    let id_options = bake_event_options(state, kind);
    (kind, id_options)
}

impl EventKind {
    pub fn name(&self) -> EventName {
        match self {
            EventKind::BigFish => EventName::BigFish,
            EventKind::TheCleric => EventName::TheCleric,
            EventKind::Duplicator => EventName::Duplicator,
            EventKind::GoldenShrine => EventName::GoldenShrine,
            EventKind::WingStatue => EventName::WingStatue,
            EventKind::WorldOfGoop => EventName::WorldOfGoop,
            EventKind::LivingWall => EventName::LivingWall,
            EventKind::Purifier => EventName::Purifier,
            EventKind::ShiningLight => EventName::ShiningLight,
            EventKind::TheSsssserpent => EventName::TheSsssserpent,
            EventKind::Transmogrifier => EventName::Transmogrifier,
            EventKind::UpgradeShrine => EventName::UpgradeShrine,
            EventKind::TheDivineFountain => EventName::TheDivineFountain,
            EventKind::TheLab => EventName::TheLab,
            EventKind::TheWomanInBlue => EventName::TheWomanInBlue,
            EventKind::WheelOfChange => EventName::WheelOfChange,
            EventKind::BonfireSpirits => EventName::BonfireSpirits,
            EventKind::OminousForge => EventName::OminousForge,
            EventKind::FaceTrader => EventName::FaceTrader,
            EventKind::Mushrooms => EventName::Mushrooms,
            EventKind::GoldenIdol { .. } => EventName::GoldenIdol,
            EventKind::ScrapOoze { .. } => EventName::ScrapOoze,
            EventKind::WeMeetAgain { .. } => EventName::WeMeetAgain,
            EventKind::DeadAdventurer { .. } => EventName::DeadAdventurer,
        }
    }
}

// One Entity per option, copied into the arena at spawn (the card-spawn idiom)
pub fn bake_options(state: &mut GameState, options: &[Entity]) -> Vec<usize> {
    let mut id_options = Vec::with_capacity(options.len());
    for &option in options {
        id_options.push(push_entity(&mut state.entities, option));
    }
    id_options
}

// Option lists encode only spawn-time state; mid-event dynamism lives in processors
fn bake_event_options(state: &mut GameState, kind: EventKind) -> Vec<usize> {
    let ascension = state.ascension;
    match kind {
        EventKind::BigFish => bake_options(state, big_fish::OPTIONS),
        EventKind::TheCleric => bake_options(state, the_cleric::options(ascension)),
        EventKind::Duplicator => bake_options(state, duplicator::OPTIONS),
        EventKind::GoldenShrine => bake_options(state, golden_shrine::options(ascension)),
        EventKind::WingStatue => bake_options(state, wing_statue::OPTIONS),
        EventKind::WorldOfGoop => bake_options(state, world_of_goop::options(ascension)),
        EventKind::LivingWall => bake_options(state, living_wall::OPTIONS),
        EventKind::Purifier => bake_options(state, purifier::OPTIONS),
        EventKind::ShiningLight => bake_options(state, shining_light::options(ascension)),
        EventKind::TheSsssserpent => bake_options(state, the_ssssserpent::options(ascension)),
        EventKind::Transmogrifier => bake_options(state, transmogrifier::OPTIONS),
        EventKind::UpgradeShrine => bake_options(state, upgrade_shrine::OPTIONS),
        EventKind::TheDivineFountain => bake_options(state, the_divine_fountain::OPTIONS),
        EventKind::TheLab => bake_options(state, the_lab::options(ascension)),
        EventKind::TheWomanInBlue => bake_options(state, the_woman_in_blue::options(ascension)),
        EventKind::WheelOfChange => bake_options(state, wheel_of_change::OPTIONS),
        EventKind::BonfireSpirits => bake_options(state, bonfire_spirits::OPTIONS),
        EventKind::OminousForge => bake_options(state, ominous_forge::OPTIONS),
        EventKind::FaceTrader => bake_options(state, face_trader::options(ascension)),
        EventKind::Mushrooms => bake_options(state, mushrooms::OPTIONS),
        EventKind::GoldenIdol { .. } => bake_options(state, golden_idol::options(ascension)),
        EventKind::ScrapOoze { .. } => bake_options(state, scrap_ooze::options(ascension)),
        EventKind::WeMeetAgain { .. } => bake_options(state, we_meet_again::OPTIONS),
        EventKind::DeadAdventurer { .. } => bake_options(state, dead_adventurer::OPTIONS),
    }
}

// Per-event availability checks
pub fn event_option_available(state: &GameState, kind: EventKind, idx: usize) -> bool {
    match kind {
        EventKind::BigFish
        | EventKind::Duplicator
        | EventKind::GoldenShrine
        | EventKind::WorldOfGoop
        | EventKind::TheSsssserpent
        | EventKind::TheDivineFountain
        | EventKind::TheLab
        | EventKind::TheWomanInBlue
        | EventKind::WheelOfChange
        | EventKind::BonfireSpirits
        | EventKind::FaceTrader
        | EventKind::Mushrooms
        | EventKind::DeadAdventurer { .. } => true,
        EventKind::TheCleric => the_cleric::option_available(state, idx),
        EventKind::WingStatue => wing_statue::option_available(state, idx),
        EventKind::LivingWall => living_wall::option_available(state, idx),
        EventKind::Purifier => purifier::option_available(state, idx),
        EventKind::ShiningLight => shining_light::option_available(state, idx),
        EventKind::Transmogrifier => transmogrifier::option_available(state, idx),
        EventKind::UpgradeShrine => upgrade_shrine::option_available(state, idx),
        EventKind::OminousForge => ominous_forge::option_available(state, idx),
        EventKind::GoldenIdol { stage } => golden_idol::option_available(stage, idx),
        EventKind::ScrapOoze { attempts } => scrap_ooze::option_available(attempts, idx),
        EventKind::WeMeetAgain {
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
