mod big_fish;
mod bonfire_spirits;
mod duplicator;
mod face_trader;
mod golden_idol;
mod golden_shrine;
mod living_wall;
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
mod wheel_of_change;
mod wing_statue;
mod world_of_goop;

use rand::Rng;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::types::CardKind;
use crate::types::CardRarity;
use crate::types::EventName;
use crate::types::RelicName;
use crate::utils::card_is_purgeable;
use crate::utils::card_is_upgradable;

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

pub fn get_event(name: EventName, ascension: u8) -> Entity {
    match name {
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
    }
}

pub fn spawn_event(name: EventName, ascension: u8, _rng: &mut impl Rng) -> Entity {
    get_event(name, ascension)
}

pub const POOL_ACT1_EVENT: &[EventName] = &[
    EventName::BigFish,
    EventName::TheCleric,
    EventName::GoldenIdol,
    EventName::WingStatue,
    EventName::WorldOfGoop,
    EventName::LivingWall,
    EventName::ScrapOoze,
    EventName::ShiningLight,
    EventName::TheSsssserpent,
];

// Shrines roll separately at EVENT_SHRINE_CHANCE; Duplicator is Act 2 and 3 only.
// Fountain and Woman in Blue are draw-gated in `draw_shrine` (curse / gold)
pub const POOL_ACT1_SHRINE: &[EventName] = &[
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
];
