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
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::EventName;
use crate::types::RelicName;

pub const EVENT_END_EFFECT: Effect = Effect::direct(EffectKind::EventEnd, None, None);

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

pub fn card_is_upgradable(entity: &Entity) -> bool {
    if entity.kind != EntityKind::Card {
        return false;
    }
    if entity.card_upgraded {
        return false;
    }
    !matches!(entity.card_kind, CardKind::Curse | CardKind::Status)
}

pub fn card_is_purgeable(entity: &Entity) -> bool {
    if entity.kind != EntityKind::Card {
        return false;
    }
    !matches!(entity.card_name, CardName::AscendersBane)
}

pub fn card_in_deck_filter(entity: &Entity, kind: crate::types::DeckSelectKind) -> bool {
    use crate::types::DeckSelectKind;
    match kind {
        DeckSelectKind::Remove => card_is_purgeable(entity),
        DeckSelectKind::DuplicateAny => entity.kind == EntityKind::Card,
        DeckSelectKind::UpgradeAny => card_is_upgradable(entity),
        DeckSelectKind::TransformOne => {
            entity.kind == EntityKind::Card
                && entity.card_rarity != crate::types::CardRarity::Basic
                && entity.card_kind != CardKind::Curse
        }
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
    &cleric::CLERIC,
    &designer::DESIGNER,
    &duplicator::DUPLICATOR,
    &gold_shrine::GOLD_SHRINE,
    &golden_idol_event::GOLDEN_IDOL_EVENT,
    &golden_wing::GOLDEN_WING,
    &goop_puddle::GOOP_PUDDLE,
    &living_wall::LIVING_WALL,
    &purification_shrine::PURIFICATION_SHRINE,
    &scrap_ooze::SCRAP_OOZE,
    &shining_light::SHINING_LIGHT,
    &sssserpent::SSSSERPENT,
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

pub fn get_event(name: EventName) -> Entity {
    match name {
        EventName::BigFish => big_fish::BIG_FISH,
        EventName::Cleric => cleric::CLERIC,
        EventName::Designer => designer::DESIGNER,
        EventName::Duplicator => duplicator::DUPLICATOR,
        EventName::GoldShrine => gold_shrine::GOLD_SHRINE,
        EventName::GoldenIdolEvent => golden_idol_event::GOLDEN_IDOL_EVENT,
        EventName::GoldenWing => golden_wing::GOLDEN_WING,
        EventName::GoopPuddle => goop_puddle::GOOP_PUDDLE,
        EventName::LivingWall => living_wall::LIVING_WALL,
        EventName::PurificationShrine => purification_shrine::PURIFICATION_SHRINE,
        EventName::ScrapOoze => scrap_ooze::SCRAP_OOZE,
        EventName::ShiningLight => shining_light::SHINING_LIGHT,
        EventName::Sssserpent => sssserpent::SSSSERPENT,
        EventName::Transmogrifier => transmogrifier::TRANSMOGRIFIER,
        EventName::UpgradeShrine => upgrade_shrine::UPGRADE_SHRINE,
        EventName::WeMeetAgain => we_meet_again::WE_MEET_AGAIN,
    }
}

pub fn spawn_event(name: EventName, _rng: &mut impl Rng) -> Entity {
    get_event(name)
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
