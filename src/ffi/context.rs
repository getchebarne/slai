use pyo3::prelude::*;

use crate::game::GameState;
use crate::types::ChestKind;
use crate::types::EventName;

use super::card::PyCard;
use super::card::snapshot_card;
use super::card::snapshot_card_combat;
use super::effect::PyEffect;
use super::effect::snapshot_effect;
use super::event::PyEventName;
use super::macros::mirror_enum;
use super::monster::PyMonster;
use super::monster::PyMonsterEncounter;
use super::monster::snapshot_monsters;
use super::potion::PyPotion;
use super::potion::snapshot_potion;
use super::relic::PyRelic;
use super::relic::snapshot_relic;

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

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "Combat",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCombat {
    pub card_pile_hand: Vec<PyCard>,
    pub card_pile_draw: Vec<PyCard>,
    pub card_pile_discard: Vec<PyCard>,
    pub card_pile_exhaust: Vec<PyCard>,
    pub card_play_queue: Vec<PyCard>,
    pub energy: PyEnergy,
    pub monsters: Vec<PyMonster>,
    pub card_pile_stasis: Vec<Option<PyCard>>, // Parallel to `monsters`: the Card each one holds in Stasis
    pub card_pile_discover: Vec<PyCard>,
    pub bombs: Vec<(u8, u16)>,
    pub card_pile_nightmare: Vec<PyCard>, // Each arrives NIGHTMARE_COPIES times next turn
    pub panache_countdown: u8,            // Plays left until Panache's hit
    pub this_turn_discards: u16, // Cards discarded this turn (Sneaky Strike's refund, Eviscerate's discount)
    pub this_turn_attacks: u8,   // Attacks played this turn (Finisher's hits, Art of War's energy)
    pub this_turn_cards_played: u8, // Cards played this turn (Normality's cap)
    pub orange_pellets_played_attack: bool, // Card kinds played since Orange Pellets last fired this turn
    pub orange_pellets_played_skill: bool,
    pub orange_pellets_played_power: bool,
    pub this_combat_thief_escaped: bool, // A Smoke Bomb then still keeps the reward
    pub this_combat_monster_died: bool, // Monsters escaping afterwards no longer cost a normal fight its gold and Potion roll
    pub gold_stolen: u16, // The killed thieves' purse, claimed apart from the room's gold
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "Reward",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyReward {
    pub cards: Vec<Vec<PyCard>>,
    pub relics: Vec<PyRelic>,
    pub potions: Vec<PyPotion>,
    pub gold: Option<u16>,
    pub gold_stolen: Option<u16>, // The thieves' purse, claimed apart from the room's gold
    pub relics_exclusive: bool,   // The boss chest's Relics are mutually exclusive
    pub cards_forced: bool,       // Cannot skip Card rewards (The Library)
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "Shop",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyShop {
    pub cards: Vec<PyCard>,
    pub card_prices: Vec<u16>,
    pub relics: Vec<PyRelic>,
    pub relic_prices: Vec<u16>,
    pub potions: Vec<PyPotion>,
    pub potion_prices: Vec<u16>,
    pub purge_cost: u16,
    pub purged: bool,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "Event",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEvent {
    pub name: PyEventName,
    pub consumed: bool,
    pub stage: u8,
    pub options: Vec<Vec<PyEffect>>,
    pub health_max_at_open: u16,
    pub card_pile_event_roll: Vec<PyCard>,
    pub relic_event_roll: Vec<PyRelic>,
    pub potion_event_roll: Vec<PyPotion>,
    pub adventurer_elite: Option<PyMonsterEncounter>, // Only while the event is Dead Adventurer
    pub found_gold: bool,
    pub found_nothing: bool,
    pub found_relic: bool,

    // Match and Keep!: the face-up first flip, the count of Cards never flipped, one Card per matched pair, the attempts left
    pub card_match_flipped: Option<PyCard>,
    pub card_match_unseen_count: u8,
    pub card_match_pairs: Vec<PyCard>,
    pub match_attempts: u8,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "RestSite",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyRestSite {
    pub done: bool,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "Chest",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyChest {
    pub kind: PyChestKind,
    pub opened: bool,
}

pub(crate) fn snapshot_combat(state: &GameState) -> PyCombat {
    let combat = &state.combat;

    // A Card played now has Mind Blast hit for the draw pile as it stands; a waiting play keeps the size it was queued with
    let draw_pile_size = combat.id_card_pile_draw.len() as u16;
    PyCombat {
        card_pile_hand: combat
            .id_card_pile_hand
            .iter()
            .map(|&id| snapshot_card_combat(state, id, &combat.id_card_pile_draw, draw_pile_size))
            .collect(),

        // A draw-pile Card is judged as if it had left the draw pile
        card_pile_draw: combat
            .id_card_pile_draw
            .iter()
            .map(|&id| {
                let id_card_pile_draw_rest: Vec<usize> = combat
                    .id_card_pile_draw
                    .iter()
                    .copied()
                    .filter(|&id_other| id_other != id)
                    .collect();
                snapshot_card_combat(state, id, &id_card_pile_draw_rest, draw_pile_size - 1)
            })
            .collect(),
        card_pile_discard: combat
            .id_card_pile_discard
            .iter()
            .map(|&id| snapshot_card_combat(state, id, &combat.id_card_pile_draw, draw_pile_size))
            .collect(),
        card_pile_exhaust: combat
            .id_card_pile_exhaust
            .iter()
            .map(|&id| snapshot_card_combat(state, id, &combat.id_card_pile_draw, draw_pile_size))
            .collect(),
        card_play_queue: state
            .card_play_queue
            .iter()
            .map(|card_play| {
                snapshot_card_combat(
                    state,
                    card_play.id_card,
                    &combat.id_card_pile_draw,
                    card_play.draw_pile_size,
                )
            })
            .collect(),
        energy: PyEnergy {
            energy_current: combat.energy.energy_current,
            energy_max: combat.energy.energy_max,
        },
        monsters: snapshot_monsters(state),
        // Walks the filled roster slots as `monsters` does, so the two line up
        card_pile_stasis: combat
            .id_monsters
            .iter()
            .zip(&combat.id_card_pile_stasis)
            .filter_map(|(&id_monster, &id_card)| {
                id_monster.map(|_| {
                    id_card.map(|id_card| {
                        snapshot_card_combat(
                            state,
                            id_card,
                            &combat.id_card_pile_draw,
                            draw_pile_size,
                        )
                    })
                })
            })
            .collect(),
        card_pile_discover: combat
            .id_card_pile_discover
            .iter()
            .map(|&id| snapshot_card_combat(state, id, &combat.id_card_pile_draw, draw_pile_size))
            .collect(),
        bombs: combat
            .bombs
            .iter()
            .map(|&(turns, damage, _)| (turns, damage))
            .collect(),
        card_pile_nightmare: combat
            .id_card_pile_nightmare
            .iter()
            .map(|&(id, _)| {
                snapshot_card_combat(state, id, &combat.id_card_pile_draw, draw_pile_size)
            })
            .collect(),
        panache_countdown: combat.panache_countdown,
        this_turn_discards: combat.this_turn_discards,
        this_turn_attacks: combat.this_turn_attacks,
        this_turn_cards_played: combat.this_turn_cards_played,
        orange_pellets_played_attack: combat.orange_pellets_played_attack,
        orange_pellets_played_skill: combat.orange_pellets_played_skill,
        orange_pellets_played_power: combat.orange_pellets_played_power,
        this_combat_thief_escaped: combat.this_combat_thief_escaped,
        this_combat_monster_died: combat.this_combat_monster_died,
        gold_stolen: combat.gold_stolen,
    }
}

pub(crate) fn snapshot_reward(state: &GameState) -> PyReward {
    let reward = &state.reward;
    PyReward {
        cards: reward
            .id_cards
            .iter()
            .map(|bundle| bundle.iter().map(|&id| snapshot_card(state, id)).collect())
            .collect(),
        relics: reward
            .id_relics
            .iter()
            .map(|&id| snapshot_relic(id, &state.entities[id]))
            .collect(),
        potions: reward
            .id_potions
            .iter()
            .map(|&id| snapshot_potion(id, &state.entities[id]))
            .collect(),
        gold: reward.gold,
        gold_stolen: reward.gold_stolen,
        relics_exclusive: reward.relics_exclusive,
        cards_forced: reward.cards_forced,
    }
}

pub(crate) fn snapshot_shop(state: &GameState) -> PyShop {
    let shop = &state.shop;
    PyShop {
        cards: shop
            .id_cards_price
            .iter()
            .map(|&(id, _)| snapshot_card(state, id))
            .collect(),
        card_prices: shop
            .id_cards_price
            .iter()
            .map(|&(_, price)| price)
            .collect(),
        relics: shop
            .id_relics_price
            .iter()
            .map(|&(id, _)| snapshot_relic(id, &state.entities[id]))
            .collect(),
        relic_prices: shop
            .id_relics_price
            .iter()
            .map(|&(_, price)| price)
            .collect(),
        potions: shop
            .id_potions_price
            .iter()
            .map(|&(id, _)| snapshot_potion(id, &state.entities[id]))
            .collect(),
        potion_prices: shop
            .id_potions_price
            .iter()
            .map(|&(_, price)| price)
            .collect(),
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
        options: event
            .id_event_options
            .iter()
            .map(|&id| {
                let option = &state.entities[id];
                option.event_option_effects[..option.event_option_effects_len as usize]
                    .iter()
                    .map(snapshot_effect)
                    .collect()
            })
            .collect(),
        health_max_at_open: event.health_max_at_open,
        consumed: event.consumed,
        adventurer_elite: (event.name == EventName::DeadAdventurer)
            .then_some(event.adventurer_elite.into()),
        card_pile_event_roll: event
            .id_card_pile_event_roll
            .iter()
            .map(|&id| snapshot_card(state, id))
            .collect(),
        relic_event_roll: event
            .id_relic_event_roll
            .iter()
            .map(|&id| snapshot_relic(id, &state.entities[id]))
            .collect(),
        potion_event_roll: event
            .id_potion_event_roll
            .iter()
            .map(|&id| snapshot_potion(id, &state.entities[id]))
            .collect(),
        card_match_flipped: event
            .id_card_match_flipped
            .map(|id| snapshot_card(state, id)),
        card_match_unseen_count: event.id_card_match_unseen.len() as u8,
        card_match_pairs: event
            .id_card_match_pairs
            .iter()
            .map(|&id| snapshot_card(state, id))
            .collect(),
        match_attempts: event.match_attempts,
    }
}

pub(crate) fn snapshot_rest_site(state: &GameState) -> PyRestSite {
    PyRestSite {
        done: state.rest_site.consumed,
    }
}

pub(crate) fn snapshot_chest(state: &GameState) -> PyChest {
    PyChest {
        kind: state.chest.chest_kind.into(),
        opened: state.chest.chest_opened,
    }
}
