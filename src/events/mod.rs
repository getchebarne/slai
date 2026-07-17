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
use crate::types::RelicName;
use crate::utils::card_is_purgeable;
use crate::utils::card_is_upgradable;
use crate::utils::push_entity;

pub const EVENT_CONSUME_EFFECT: Effect = Effect {
    kind: EffectKind::EventConsume,
    id_source: None,
    target: Target::Direct(None),
};

#[derive(Debug, Clone, Copy)]
pub struct EventOption {
    pub label: &'static str,
    pub effects: &'static [Effect],
    pub gate: EventGate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventGate {
    None,
    GoldAtLeast(u16),
    HpAtLeast(u16),
    HasUpgradableInDeck,
    HasPurgeableInDeck,
    HasNonBasicNonCurseInDeck,
    HasDamageCardInDeck { min_base: u16 },
    HasRelicOwned(RelicName),
    PotionBeltHasAny,
    EventPickValid(EntityKind),
    EventStateEq(u8),
    All(&'static [EventGate]),
}

pub fn event_option_gate_satisfied(gate: EventGate, state: &GameState, id_event: usize) -> bool {
    let character = &state.entities[state.id_character];
    match gate {
        EventGate::None => true,
        EventGate::GoldAtLeast(amount) => character.character_gold >= amount,
        EventGate::HpAtLeast(amount) => character.vitals.health >= amount,
        EventGate::HasUpgradableInDeck => state
            .id_deck
            .iter()
            .any(|&id| card_is_upgradable(&state.entities[id])),
        EventGate::HasPurgeableInDeck => state
            .id_deck
            .iter()
            .any(|&id| card_is_purgeable(&state.entities[id])),
        EventGate::HasNonBasicNonCurseInDeck => state.id_deck.iter().any(|&id| {
            let entity = &state.entities[id];
            entity.card_rarity != CardRarity::Basic && entity.card_kind != CardKind::Curse
        }),
        EventGate::HasDamageCardInDeck { min_base } => state
            .id_deck
            .iter()
            .any(|&id| card_has_damage_at_least(&state.entities[id], min_base)),
        EventGate::HasRelicOwned(name) => state.id_relics[name as usize].is_some(),
        EventGate::PotionBeltHasAny => state
            .id_potions
            .iter()
            .take(state.potion_slots_max as usize)
            .any(|slot| slot.is_some()),
        EventGate::EventPickValid(kind) => state.id_event_picks.iter().any(|&id| {
            state.entities[id].kind == kind
                && match kind {
                    EntityKind::Card => state.id_deck.contains(&id),
                    EntityKind::Potion => state.id_potions.contains(&Some(id)),
                    _ => unreachable!("EventPickValid on unsupported kind: {kind:?}"),
                }
        }),
        EventGate::EventStateEq(value) => state.entities[id_event].event_state == value,
        EventGate::All(gates) => gates
            .iter()
            .all(|g| event_option_gate_satisfied(*g, state, id_event)),
    }
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

// Spawns `name`, registers it as the active event, and runs its entry rolls
pub fn spawn_event(state: &mut GameState, name: EventName) {
    let ascension = state.ascension;
    let event = match name {
        EventName::BigFish => big_fish::spawn_event_big_fish(),
        EventName::TheCleric => the_cleric::spawn_event_the_cleric(ascension),
        EventName::Duplicator => duplicator::spawn_event_duplicator(),
        EventName::GoldenShrine => golden_shrine::spawn_event_golden_shrine(ascension),
        EventName::GoldenIdol => golden_idol::spawn_event_golden_idol(ascension),
        EventName::WingStatue => wing_statue::spawn_event_wing_statue(),
        EventName::WorldOfGoop => world_of_goop::spawn_event_world_of_goop(ascension),
        EventName::LivingWall => living_wall::spawn_event_living_wall(),
        EventName::Purifier => purifier::spawn_event_purifier(),
        EventName::ScrapOoze => scrap_ooze::spawn_event_scrap_ooze(ascension),
        EventName::ShiningLight => shining_light::spawn_event_shining_light(ascension),
        EventName::TheSsssserpent => the_ssssserpent::spawn_event_the_ssssserpent(ascension),
        EventName::Transmogrifier => transmogrifier::spawn_event_transmogrifier(),
        EventName::UpgradeShrine => upgrade_shrine::spawn_event_upgrade_shrine(),
        EventName::TheDivineFountain => the_divine_fountain::spawn_event_the_divine_fountain(),
        EventName::TheLab => the_lab::spawn_event_the_lab(ascension),
        EventName::TheWomanInBlue => the_woman_in_blue::spawn_event_the_woman_in_blue(ascension),
        EventName::WheelOfChange => wheel_of_change::spawn_event_wheel_of_change(),
        EventName::BonfireSpirits => bonfire_spirits::spawn_event_bonfire_spirits(),
        EventName::OminousForge => ominous_forge::spawn_event_ominous_forge(),
        EventName::FaceTrader => face_trader::spawn_event_face_trader(ascension),
        EventName::WeMeetAgain => we_meet_again::spawn_event_we_meet_again(state),
        EventName::Mushrooms => mushrooms::spawn_event_mushrooms(),
        EventName::DeadAdventurer => dead_adventurer::spawn_event_dead_adventurer(state),
    };
    let id_event = push_entity(&mut state.entities, event);
    state.id_event = Some(id_event);
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

// Dead Adventurer entry rolls: [enemy, remaining rewards..]. Slots past
// ADVENTURER_IDX_REWARDS hold the not-yet-found rewards; finds swap-remove them
pub const ADVENTURER_IDX_ENEMY: usize = 0;
pub const ADVENTURER_IDX_REWARDS: usize = 1;
pub const ADVENTURER_REWARD_GOLD: u16 = 0;
pub const ADVENTURER_REWARD_NOTHING: u16 = 1;
pub const ADVENTURER_REWARD_RELIC: u16 = 2;

pub fn adventurer_enemy_encounter(roll: u16) -> MonsterEncounter {
    match roll {
        0 => MonsterEncounter::ThreeSentries,
        1 => MonsterEncounter::GremlinNob,
        2 => MonsterEncounter::Lagavulin,
        _ => unreachable!("adventurer enemy roll out of range: {roll}"),
    }
}
