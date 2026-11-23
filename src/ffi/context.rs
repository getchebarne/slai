use pyo3::prelude::*;

use crate::consts::MAX_MONSTERS;
use crate::game::GameState;
use crate::types::ChestKind;
use crate::types::Combat;
use crate::types::EventName;

use super::effect::snapshot_damage_card;
use super::event::PyEventName;
use super::macros::mirror_enum;
use super::monster::PyMonsterEncounter;

mirror_enum!(PyChestKind from ChestKind, "ChestKind", {
    Small, Medium, Large, Boss,
});

#[pyclass(eq, hash, frozen, get_all, name = "Energy", module = "slai._slai")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEnergy {
    pub energy_current: u16,
    pub energy_max: u16,
}

#[pyclass(eq, hash, frozen, get_all, name = "Combat", module = "slai._slai")]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PyCombat {
    pub id_card_pile_hand: Vec<usize>,
    pub id_card_pile_draw: Vec<usize>,
    pub id_card_pile_discard: Vec<usize>,
    pub id_card_pile_exhaust: Vec<usize>,
    pub id_card_pile_discover: Vec<usize>,
    pub id_card_pile_stasis: [Option<usize>; MAX_MONSTERS], // Slot-parallel to `id_monsters`
    pub id_monsters: [Option<usize>; MAX_MONSTERS],         // The roster's slots, None where empty

    // Aligned with `id_card_pile_hand`, slot-parallel to `id_monsters`: each Card's hit as hovered
    pub damage_card_pile_hand: Vec<[Option<u16>; MAX_MONSTERS]>,

    // Live Bombs: (turns left, damage, seq)
    pub bombs: Vec<(u8, u16, u32)>,

    // Nightmare'd Cards (id, seq)
    pub id_card_pile_nightmare: Vec<(usize, u32)>,

    // Plays left until Panache's hit
    pub panache_countdown: u8,

    // This turn counters
    pub this_turn_discards: u16,
    pub this_turn_attacks: u16,
    pub this_turn_cards_played: u16,

    // Orange pellets state
    pub orange_pellets_played_attack: bool,
    pub orange_pellets_played_skill: bool,
    pub orange_pellets_played_power: bool,

    // Turn end progress flags, for mid-turn-end halts
    pub turn_end_pending: bool,
    pub turn_end_started: bool,
    pub turn_ended: bool,

    // If a thief escaped, Smoke Bomb keeps the reward
    pub this_combat_thief_escaped: bool,

    // The killed thieves' purse, claimed apart from the room's gold
    pub gold_stolen: u16,

    // Monsters escaping afterwards no longer cost a normal fight its gold and Potion roll
    pub this_combat_monster_died: bool,

    // A boss Monster opened this fight; it stays set after the boss leaves the roster
    pub boss_fight: bool,

    // Energy
    pub energy: PyEnergy,
}

#[pyclass(eq, hash, frozen, get_all, name = "Reward", module = "slai._slai")]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PyReward {
    pub id_cards: Vec<Vec<usize>>,
    pub id_relics: Vec<usize>,
    pub id_relic_linked: Option<usize>, // The Relic the Sapphire Key is linked to
    pub id_potions: Vec<usize>,

    // Gold (organic and stolen)
    pub gold: Option<u16>,
    pub gold_stolen: Option<u16>,

    // A Boss chest's Relic picks are mutually exclusive
    pub relics_exclusive: bool,

    // Wether the Cards can be skipped (The Library)
    pub cards_forced: bool,
}

#[pyclass(eq, hash, frozen, get_all, name = "Shop", module = "slai._slai")]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PyShop {
    pub id_cards: Vec<usize>,
    pub id_relics: Vec<usize>,
    pub id_potions: Vec<usize>,

    // Purge data
    pub purge_cost: u16,
    pub purged: bool,
}

#[pyclass(eq, hash, frozen, get_all, name = "Event", module = "slai._slai")]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PyEvent {
    pub id_card_pile_event_roll: Vec<usize>,
    pub id_card_match_flipped: Option<usize>, // Match and Keep! only
    pub id_card_match_pairs: Vec<usize>,      // Match and Keep! only
    pub id_event_options: Vec<usize>,
    pub id_relic_event_roll: Vec<usize>,
    pub id_potion_event_roll: Vec<usize>,

    // The Event's name
    pub name: PyEventName,

    // Wether it has been consumed
    pub consumed: bool,

    // For Events that advance stage only (e.g., Golden Idol, Cursed Tome, etc.)
    pub stage: u8,

    // Some Events Options read the Character's max when they were sopened
    pub health_max_at_open: u16,

    // Dead Adventurer only
    pub adventurer_elite: Option<PyMonsterEncounter>,
    pub found_gold: bool,
    pub found_nothing: bool,
    pub found_relic: bool,

    // Match and Keep! only
    pub card_match_unseen_count: u8,
    pub match_attempts: u8,
}

#[pyclass(eq, hash, frozen, get_all, name = "RestSite", module = "slai._slai")]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PyRestSite {
    pub consumed: bool,
}

#[pyclass(eq, hash, frozen, get_all, name = "Chest", module = "slai._slai")]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PyChest {
    pub chest_kind: PyChestKind,
    pub chest_opened: bool,
}

pub(crate) fn snapshot_combat(state: &GameState) -> PyCombat {
    let combat = &state.combat;
    PyCombat {
        id_card_pile_hand: combat.id_card_pile_hand.clone(),
        id_card_pile_draw: id_card_pile_draw_sorted(combat),
        id_card_pile_discard: combat.id_card_pile_discard.clone(),
        id_card_pile_exhaust: combat.id_card_pile_exhaust.clone(),
        id_card_pile_discover: combat.id_card_pile_discover.clone(),
        id_card_pile_stasis: combat.id_card_pile_stasis,
        id_monsters: combat.id_monsters,

        damage_card_pile_hand: combat
            .id_card_pile_hand
            .iter()
            .map(|&id_card| snapshot_damage_card(state, id_card))
            .collect(),

        bombs: combat.bombs.clone(),

        id_card_pile_nightmare: combat.id_card_pile_nightmare.clone(),

        panache_countdown: combat.panache_countdown,

        this_turn_discards: combat.this_turn_discards,
        this_turn_attacks: combat.this_turn_attacks,
        this_turn_cards_played: combat.this_turn_cards_played,

        orange_pellets_played_attack: combat.orange_pellets_played_attack,
        orange_pellets_played_skill: combat.orange_pellets_played_skill,
        orange_pellets_played_power: combat.orange_pellets_played_power,

        turn_end_pending: combat.turn_end_pending,
        turn_end_started: combat.turn_end_started,
        turn_ended: combat.turn_ended,

        this_combat_thief_escaped: combat.this_combat_thief_escaped,

        gold_stolen: combat.gold_stolen,

        this_combat_monster_died: combat.this_combat_monster_died,

        boss_fight: combat.boss_fight,

        energy: PyEnergy {
            energy_current: combat.energy.energy_current,
            energy_max: combat.energy.energy_max,
        },
    }
}

// The draw pile's ids by ascending id, so the draw order stays hidden
pub(crate) fn id_card_pile_draw_sorted(combat: &Combat) -> Vec<usize> {
    let mut id_card_pile_draw = combat.id_card_pile_draw.clone();
    id_card_pile_draw.sort_unstable();
    id_card_pile_draw
}

pub(crate) fn snapshot_reward(state: &GameState) -> PyReward {
    let reward = &state.reward;
    PyReward {
        id_cards: reward.id_cards.clone(),
        id_relics: reward.id_relics.clone(),
        id_relic_linked: reward.id_relic_linked,
        id_potions: reward.id_potions.clone(),

        gold: reward.gold,
        gold_stolen: reward.gold_stolen,

        relics_exclusive: reward.relics_exclusive,

        cards_forced: reward.cards_forced,
    }
}

pub(crate) fn snapshot_shop(state: &GameState) -> PyShop {
    let shop = &state.shop;
    PyShop {
        id_cards: shop.id_cards.clone(),
        id_relics: shop.id_relics.clone(),
        id_potions: shop.id_potions.clone(),

        purge_cost: shop.purge_cost,
        purged: shop.purged,
    }
}

pub(crate) fn snapshot_event(state: &GameState) -> PyEvent {
    let event = &state.event;
    PyEvent {
        id_card_pile_event_roll: event.id_card_pile_event_roll.clone(),
        id_card_match_flipped: event.id_card_match_flipped,
        id_card_match_pairs: event.id_card_match_pairs.clone(),
        id_event_options: event.id_event_options.clone(),
        id_relic_event_roll: event.id_relic_event_roll.clone(),
        id_potion_event_roll: event.id_potion_event_roll.clone(),

        name: event.name.into(),

        consumed: event.consumed,

        stage: event.stage,

        health_max_at_open: event.health_max_at_open,

        adventurer_elite: (event.name == EventName::DeadAdventurer)
            .then_some(event.adventurer_elite.into()),
        found_gold: event.found_gold,
        found_nothing: event.found_nothing,
        found_relic: event.found_relic,

        card_match_unseen_count: event.id_card_match_unseen.len() as u8,
        match_attempts: event.match_attempts,
    }
}

pub(crate) fn snapshot_rest_site(state: &GameState) -> PyRestSite {
    PyRestSite {
        consumed: state.rest_site.consumed,
    }
}

pub(crate) fn snapshot_chest(state: &GameState) -> PyChest {
    let chest = &state.chest;
    PyChest {
        chest_kind: chest.chest_kind.into(),
        chest_opened: chest.chest_opened,
    }
}
