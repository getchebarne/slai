use pyo3::prelude::*;

use crate::consts::MAX_MONSTERS;
use crate::game::GameState;
use crate::types::ChestKind;
use crate::types::Combat;
use crate::types::EventName;

use super::event::PyEventName;
use super::macros::mirror_enum;
use super::monster::PyMonsterEncounter;

mirror_enum!(PyChestKind from ChestKind, "ChestKind", {
    Small, Medium, Large, Boss,
});

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "Energy",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEnergy {
    pub energy_current: u16,
    pub energy_max: u16,
}

#[pyclass(eq, hash, frozen, get_all, name = "Combat", module = "slai.slai")]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PyCombat {
    pub id_card_pile_hand: Vec<usize>,

    // By ascending id, hiding the draw order; a Card whose depth is known shows it as draw_depth
    pub id_card_pile_draw: Vec<usize>,
    pub id_card_pile_discard: Vec<usize>,
    pub id_card_pile_exhaust: Vec<usize>,
    pub energy: PyEnergy,
    pub id_monsters: [Option<usize>; MAX_MONSTERS], // The roster's slots, None where empty

    // Slot-parallel to `id_monsters`: the Card each one holds in Stasis
    pub id_card_pile_stasis: [Option<usize>; MAX_MONSTERS],
    pub id_card_pile_discover: Vec<usize>,
    pub bombs: Vec<(u8, u16, u32)>, // Live Bombs: (turns left, damage, seq)

    // (id, seq), one per Nightmare play; each arrives NIGHTMARE_COPIES times next turn
    pub id_card_pile_nightmare: Vec<(usize, u32)>,
    pub panache_countdown: u8,       // Plays left until Panache's hit
    pub this_turn_discards: u16, // Cards discarded this turn (Sneaky Strike's refund, Eviscerate's discount)
    pub this_turn_attacks: u16,  // Attacks played this turn (Finisher's hits)
    pub this_turn_cards_played: u16, // Cards played this turn (Normality's cap)
    pub orange_pellets_played_attack: bool, // Card kinds played since Orange Pellets last fired this turn
    pub orange_pellets_played_skill: bool,
    pub orange_pellets_played_power: bool,
    pub turn_end_pending: bool, // From an early turn end to its first pass
    pub turn_end_started: bool, // From the turn end's first pass to the Character's next turn start
    pub turn_ended: bool, // From the first Monster turn start to the Character's next turn start
    pub this_combat_thief_escaped: bool, // A Smoke Bomb then still keeps the reward
    pub this_combat_monster_died: bool, // Monsters escaping afterwards no longer cost a normal fight its gold and Potion roll
    pub boss_fight: bool, // A boss Monster opened this fight; it stays set after the boss leaves the roster
    pub gold_stolen: u16, // The killed thieves' purse, claimed apart from the room's gold
}

#[pyclass(eq, hash, frozen, get_all, name = "Reward", module = "slai.slai")]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PyReward {
    pub id_cards: Vec<Vec<usize>>,
    pub id_relics: Vec<usize>,

    // The Relic the Sapphire Key is linked to, also in `id_relics`; None means no key on offer
    pub id_relic_linked: Option<usize>,
    pub id_potions: Vec<usize>,
    pub gold: Option<u16>,
    pub gold_stolen: Option<u16>, // The thieves' purse, claimed apart from the room's gold
    pub relics_exclusive: bool,   // The boss chest's Relics are mutually exclusive
    pub cards_forced: bool,       // Cannot skip Card rewards (The Library)
}

#[pyclass(eq, hash, frozen, get_all, name = "Shop", module = "slai.slai")]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PyShop {
    pub id_cards: Vec<usize>,
    pub id_relics: Vec<usize>,
    pub id_potions: Vec<usize>,
    pub purge_cost: u16,
    pub purged: bool,
}

#[pyclass(eq, hash, frozen, get_all, name = "Event", module = "slai.slai")]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PyEvent {
    pub name: PyEventName,
    pub consumed: bool,
    pub stage: u8,
    pub id_event_options: Vec<usize>,
    pub health_max_at_open: u16,
    pub id_card_pile_event_roll: Vec<usize>,
    pub id_relic_event_roll: Vec<usize>,
    pub id_potion_event_roll: Vec<usize>,
    pub adventurer_elite: Option<PyMonsterEncounter>, // Only while the event is Dead Adventurer
    pub found_gold: bool,
    pub found_nothing: bool,
    pub found_relic: bool,

    // Match and Keep!: the face-up first flip, the count of Cards never flipped, one Card per matched pair, the attempts left
    pub id_card_match_flipped: Option<usize>,
    pub card_match_unseen_count: u8,
    pub id_card_match_pairs: Vec<usize>,
    pub match_attempts: u8,
}

#[pyclass(eq, hash, frozen, get_all, name = "RestSite", module = "slai.slai")]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PyRestSite {
    pub consumed: bool,
}

#[pyclass(eq, hash, frozen, get_all, name = "Chest", module = "slai.slai")]
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
        energy: PyEnergy {
            energy_current: combat.energy.energy_current,
            energy_max: combat.energy.energy_max,
        },
        id_monsters: combat.id_monsters,
        id_card_pile_stasis: combat.id_card_pile_stasis,
        id_card_pile_discover: combat.id_card_pile_discover.clone(),
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
        this_combat_monster_died: combat.this_combat_monster_died,
        boss_fight: combat.boss_fight,
        gold_stolen: combat.gold_stolen,
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
        name: event.name.into(),
        stage: event.stage,
        found_gold: event.found_gold,
        found_nothing: event.found_nothing,
        found_relic: event.found_relic,
        id_event_options: event.id_event_options.clone(),
        health_max_at_open: event.health_max_at_open,
        consumed: event.consumed,
        adventurer_elite: (event.name == EventName::DeadAdventurer)
            .then_some(event.adventurer_elite.into()),
        id_card_pile_event_roll: event.id_card_pile_event_roll.clone(),
        id_relic_event_roll: event.id_relic_event_roll.clone(),
        id_potion_event_roll: event.id_potion_event_roll.clone(),
        id_card_match_flipped: event.id_card_match_flipped,
        card_match_unseen_count: event.id_card_match_unseen.len() as u8,
        id_card_match_pairs: event.id_card_match_pairs.clone(),
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
