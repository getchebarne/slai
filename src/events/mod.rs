mod big_fish;
mod cleric;
mod designer;
mod duplicator;
mod gold_shrine;
mod golden_idol_event;
mod golden_wing;
mod goop_puddle;
mod living_wall;
mod purification_shrine;
mod scrap_ooze;
mod shining_light;
mod sssserpent;
mod transmogrifier;
mod upgrade_shrine;
mod we_meet_again;

use rand::Rng;
use strum::EnumCount;

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

pub const EVENT_END_EFFECT: Effect = Effect {
    kind: EffectKind::EventEnd,
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
        EventGate::PotionBeltHasAny => character
            .potion_slots
            .iter()
            .take(character.potion_slots_max as usize)
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

pub const ALL_EVENTS: &[&'static Entity] = &[
    &big_fish::BIG_FISH,
    &cleric::CLERIC_BASE,
    &designer::DESIGNER_BASE,
    &duplicator::DUPLICATOR,
    &gold_shrine::GOLD_SHRINE_BASE,
    &golden_idol_event::GOLDEN_IDOL_EVENT,
    &golden_wing::GOLDEN_WING,
    &goop_puddle::GOOP_PUDDLE_BASE,
    &living_wall::LIVING_WALL,
    &purification_shrine::PURIFICATION_SHRINE,
    &scrap_ooze::SCRAP_OOZE_BASE,
    &shining_light::SHINING_LIGHT_BASE,
    &sssserpent::SSSSERPENT_BASE,
    &transmogrifier::TRANSMOGRIFIER,
    &upgrade_shrine::UPGRADE_SHRINE,
    &we_meet_again::WE_MEET_AGAIN,
];

const _: () = {
    assert!(ALL_EVENTS.len() == EventName::COUNT);
    let mut seen = [false; EventName::COUNT];
    let mut idx = 0;
    while idx < ALL_EVENTS.len() {
        let i = ALL_EVENTS[idx].event_name as usize;
        assert!(!seen[i], "ALL_EVENTS contains a duplicate EventName");
        seen[i] = true;
        idx += 1;
    }
};

pub fn get_event(name: EventName, ascension: u8) -> Entity {
    match name {
        EventName::BigFish => big_fish::BIG_FISH,
        EventName::Cleric => cleric::spawn_event_cleric(ascension),
        EventName::Designer => designer::spawn_event_designer(ascension),
        EventName::Duplicator => duplicator::DUPLICATOR,
        EventName::GoldShrine => gold_shrine::spawn_event_gold_shrine(ascension),
        EventName::GoldenIdolEvent => golden_idol_event::GOLDEN_IDOL_EVENT,
        EventName::GoldenWing => golden_wing::GOLDEN_WING,
        EventName::GoopPuddle => goop_puddle::spawn_event_goop_puddle(ascension),
        EventName::LivingWall => living_wall::LIVING_WALL,
        EventName::PurificationShrine => purification_shrine::PURIFICATION_SHRINE,
        EventName::ScrapOoze => scrap_ooze::spawn_event_scrap_ooze(ascension),
        EventName::ShiningLight => shining_light::spawn_event_shining_light(ascension),
        EventName::Sssserpent => sssserpent::spawn_event_sssserpent(ascension),
        EventName::Transmogrifier => transmogrifier::TRANSMOGRIFIER,
        EventName::UpgradeShrine => upgrade_shrine::UPGRADE_SHRINE,
        EventName::WeMeetAgain => we_meet_again::WE_MEET_AGAIN,
    }
}

pub fn spawn_event(name: EventName, ascension: u8, _rng: &mut impl Rng) -> Entity {
    get_event(name, ascension)
}

pub const POOL_ACT1_EVENT: &[EventName] = &[
    EventName::BigFish,
    EventName::Cleric,
    EventName::Designer,
    EventName::Duplicator,
    EventName::GoldShrine,
    EventName::GoldenIdolEvent,
    EventName::GoldenWing,
    EventName::GoopPuddle,
    EventName::LivingWall,
    EventName::PurificationShrine,
    EventName::ScrapOoze,
    EventName::ShiningLight,
    EventName::Sssserpent,
    EventName::Transmogrifier,
    EventName::UpgradeShrine,
    EventName::WeMeetAgain,
];
