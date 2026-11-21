use std::convert::Infallible;

use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::type_hint_subscript;
use pyo3::type_hint_union;
use pyo3::type_object::PyTypeInfo;
use pyo3::types::PyDict;

use crate::game::GameState;

use super::card::PyCard;
use super::card::snapshot_card;
use super::card::snapshot_card_combat;
use super::card::snapshot_card_combat_at_cost;
use super::character::PyCharacter;
use super::character::snapshot_character;
use super::context::PyChest;
use super::context::PyCombat;
use super::context::PyEvent;
use super::context::PyRestSite;
use super::context::PyReward;
use super::context::PyShop;
use super::context::id_card_pile_draw_sorted;
use super::context::snapshot_chest;
use super::context::snapshot_combat;
use super::context::snapshot_event;
use super::context::snapshot_rest_site;
use super::context::snapshot_reward;
use super::context::snapshot_shop;
use super::effect::PyCardPlay;
use super::effect::PyEffectPendingObject;
use super::effect::snapshot_card_play;
use super::effect::snapshot_effect_pending;
use super::event::PyEventName;
use super::event::PyEventOption;
use super::event::snapshot_event_option;
use super::map::PyMap;
use super::map::PyRoom;
use super::map::snapshot_map;
use super::map::snapshot_room;
use super::monster::PyMonster;
use super::monster::PyMonsterEncounter;
use super::monster::snapshot_monsters;
use super::potion::PyPotion;
use super::potion::snapshot_potion;
use super::relic::PyRelic;
use super::relic::snapshot_relic;

// The contexts, entities, Map, queued plays and pending Effect are Python objects built once
#[pyclass(frozen, get_all, name = "GameState", module = "slai.slai")]
#[derive(Debug)]
pub struct PyGameState {
    // Every Entity keyed by id
    pub entities: PyEntities,

    // Contexts: None while inactive; all None = Focus::Map
    pub combat: Option<Py<PyCombat>>,
    pub reward: Option<Py<PyReward>>,
    pub event: Option<Py<PyEvent>>,
    pub shop: Option<Py<PyShop>>,
    pub rest_site: Option<Py<PyRestSite>>,
    pub chest: Option<Py<PyChest>>,
    pub game_over: bool,
    pub ascension: u8,

    // Keys for Act IV
    pub key_emerald: bool,
    pub key_ruby: bool,
    pub key_sapphire: bool,

    // Current Act and floor
    pub act: u8,
    pub floor: u8,

    pub id_character: usize,
    pub id_card_pile_deck: Vec<usize>,
    pub id_relics: Vec<usize>, // In pickup order
    pub id_potions: Vec<usize>,
    pub potion_slots_max: u8,
    pub potion_drop_mod: i32, // A combat's potion drop chance is 40 plus this within 0..=100, 0 in an escaped normal fight, 100 with White Beast Statue, 0 at a burning elite with Black Star; a drop lowers it by 10, a miss raises it by 10
    pub shop_purge_cost_run: u16, // A Shop's Card removal price before Relic discounts: 75 plus 25 per shop purge
    pub unknown_chance_monster: f32, // A "?" Room's odds; the event chance is what the three leave
    pub unknown_chance_shop: f32,
    pub unknown_chance_treasure: f32,
    pub pool_events: Vec<PyEventName>, // Events a "?" can still become, regular then special; each is drawn without replacement, and both lists are empty in Act 4
    pub pool_event_special: Vec<PyEventName>,
    pub map: Py<PyMap>,

    // The act's boss as the map shows it; from the A20 stop's RoomExit on, the second boss
    pub encounter_boss: PyMonsterEncounter,

    // Boss fights after `encounter_boss`'s: 1 in Act 3 from A20 until the second one starts, else 0
    pub encounter_pool_boss_count: u8,
    pub effect_pending: Option<PyEffectPendingObject>,
    pub id_selected: Vec<usize>,
    pub card_play_queue: Vec<Py<PyCardPlay>>,
}

// The entity table as one Python dict, built once; every read returns the same dict
#[derive(Debug)]
pub struct PyEntities(Py<PyDict>);

// Typed as a dict from each id to its Entity; the field's getter converts by reference
impl<'py> IntoPyObject<'py> for PyEntities {
    type Target = PyDict;
    type Output = Bound<'py, PyDict>;
    type Error = Infallible;
    const OUTPUT_TYPE: PyStaticExpr = type_hint_subscript!(
        PyDict::TYPE_HINT,
        <usize as IntoPyObject<'py>>::OUTPUT_TYPE,
        type_hint_union!(
            <PyCard as PyTypeInfo>::TYPE_HINT,
            <PyRelic as PyTypeInfo>::TYPE_HINT,
            <PyPotion as PyTypeInfo>::TYPE_HINT,
            <PyMonster as PyTypeInfo>::TYPE_HINT,
            <PyEventOption as PyTypeInfo>::TYPE_HINT,
            <PyCharacter as PyTypeInfo>::TYPE_HINT,
            <PyRoom as PyTypeInfo>::TYPE_HINT
        )
    );

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(self.0.into_bound(py))
    }
}

impl<'a, 'py> IntoPyObject<'py> for &'a PyEntities {
    type Target = PyDict;
    type Output = Borrowed<'a, 'py, PyDict>;
    type Error = Infallible;
    const OUTPUT_TYPE: PyStaticExpr = <PyEntities as IntoPyObject<'py>>::OUTPUT_TYPE;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(self.0.bind_borrowed(py))
    }
}

// Snapshot builders
pub fn snapshot_state(py: Python<'_>, state: &GameState) -> PyResult<PyGameState> {
    // Owned Relics in pickup order
    let mut id_relics: Vec<usize> = state.id_relics.iter().flatten().copied().collect();
    id_relics.sort_unstable_by_key(|&id| state.entities[id].relic_seq);
    Ok(PyGameState {
        combat: state
            .combat
            .active
            .then(|| Py::new(py, snapshot_combat(state)))
            .transpose()?,
        reward: state
            .reward
            .active
            .then(|| Py::new(py, snapshot_reward(state)))
            .transpose()?,
        event: state
            .event
            .active
            .then(|| Py::new(py, snapshot_event(state)))
            .transpose()?,
        shop: state
            .shop
            .active
            .then(|| Py::new(py, snapshot_shop(state)))
            .transpose()?,
        rest_site: state
            .rest_site
            .active
            .then(|| Py::new(py, snapshot_rest_site(state)))
            .transpose()?,
        chest: state
            .chest
            .active
            .then(|| Py::new(py, snapshot_chest(state)))
            .transpose()?,
        game_over: state.game_over,
        ascension: state.ascension,
        key_emerald: state.key_emerald,
        key_ruby: state.key_ruby,
        key_sapphire: state.key_sapphire,
        act: state.act,
        floor: state.floor,
        entities: snapshot_entities(py, state)?,
        id_character: state.id_character,
        id_card_pile_deck: state.id_card_pile_deck.clone(),
        id_relics,
        id_potions: state.id_potions.clone(),
        potion_slots_max: state.potion_slots_max,
        potion_drop_mod: state.potion_drop_mod,
        shop_purge_cost_run: state.shop_purge_cost_run,
        unknown_chance_monster: state.unknown_chance_monster,
        unknown_chance_shop: state.unknown_chance_shop,
        unknown_chance_treasure: state.unknown_chance_treasure,
        pool_events: state.pool_events.iter().map(|&name| name.into()).collect(),
        pool_event_special: state
            .pool_event_special
            .iter()
            .map(|&name| name.into())
            .collect(),
        map: Py::new(py, snapshot_map(state))?,
        encounter_boss: state.encounter_boss.into(),
        encounter_pool_boss_count: state.encounter_pool_boss.len() as u8,
        effect_pending: state
            .effect_pending
            .as_ref()
            .map(|effect| snapshot_effect_pending(py, effect))
            .transpose()?,
        id_selected: state.id_selected.clone(),
        card_play_queue: state
            .card_play_queue
            .iter()
            .map(|card_play| Py::new(py, snapshot_card_play(state, card_play)))
            .collect::<PyResult<_>>()?,
    })
}

// Each Entity once: a Card from its combat pile, else its first queued play, else its own values
fn snapshot_entities(py: Python<'_>, state: &GameState) -> PyResult<PyEntities> {
    let entities = PyDict::new(py);
    insert_absent(&entities, [&state.id_character], |_| {
        snapshot_character(state)
    })?;
    insert_absent(
        &entities,
        state.map.id_rooms.iter().flatten().flatten(),
        |id| snapshot_room(state, id),
    )?;

    // A Card played now has Mind Blast hit for the draw pile as it stands
    let combat = &state.combat;
    if combat.active {
        let draw_pile_size = combat.id_card_pile_draw.len() as u16;
        let id_cards = combat
            .id_card_pile_hand
            .iter()
            .chain(&combat.id_card_pile_discard)
            .chain(&combat.id_card_pile_exhaust)
            .chain(combat.id_card_pile_stasis.iter().flatten())
            .chain(&combat.id_card_pile_discover)
            .chain(
                combat
                    .id_card_pile_nightmare
                    .iter()
                    .map(|(id_card, _)| id_card),
            );
        insert_absent(&entities, id_cards, |id| {
            snapshot_card_combat(state, id, draw_pile_size)
        })?;

        // The draw pile's Cards, judged as if drawn, go in by id so the dict hides the draw order
        insert_absent(&entities, &id_card_pile_draw_sorted(combat), |id| {
            snapshot_card_combat(state, id, draw_pile_size - 1)
        })?;
        for monster in snapshot_monsters(state) {
            if !entities.contains(monster.id)? {
                entities.set_item(monster.id, monster)?;
            }
        }
    }

    // A Card only in the play queue shows its first play's queued cost and draw pile size
    for card_play in &state.card_play_queue {
        if !entities.contains(card_play.id_card)? {
            let card = snapshot_card_combat_at_cost(
                state,
                card_play.id_card,
                card_play.cost_effective,
                card_play.draw_pile_size,
            );
            entities.set_item(card_play.id_card, card)?;
        }
    }

    // Every other Entity shows its own values; the selected pile adds no second snapshot
    let id_cards = state.id_card_pile_deck.iter().chain(&state.id_selected);
    insert_absent(&entities, id_cards, |id| snapshot_card(state, id))?;
    insert_absent(&entities, state.id_relics.iter().flatten(), |id| {
        snapshot_relic(id, &state.entities[id])
    })?;
    insert_absent(&entities, &state.id_potions, |id| {
        snapshot_potion(state, id)
    })?;

    // The open contexts' offers and stakes
    let reward = &state.reward;
    if reward.active {
        insert_absent(&entities, reward.id_cards.iter().flatten(), |id| {
            snapshot_card(state, id)
        })?;
        let id_relics = reward.id_relics.iter().chain(&reward.id_relic_linked);
        insert_absent(&entities, id_relics, |id| {
            snapshot_relic(id, &state.entities[id])
        })?;
        insert_absent(&entities, &reward.id_potions, |id| {
            snapshot_potion(state, id)
        })?;
    }
    let shop = &state.shop;
    if shop.active {
        insert_absent(&entities, &shop.id_cards, |id| snapshot_card(state, id))?;
        insert_absent(&entities, &shop.id_relics, |id| {
            snapshot_relic(id, &state.entities[id])
        })?;
        insert_absent(&entities, &shop.id_potions, |id| snapshot_potion(state, id))?;
    }
    let event = &state.event;
    if event.active {
        insert_absent(&entities, &event.id_event_options, |id| {
            snapshot_event_option(state, id)
        })?;
        let id_cards = event
            .id_card_pile_event_roll
            .iter()
            .chain(&event.id_card_match_flipped)
            .chain(&event.id_card_match_pairs);
        insert_absent(&entities, id_cards, |id| snapshot_card(state, id))?;
        insert_absent(&entities, &event.id_relic_event_roll, |id| {
            snapshot_relic(id, &state.entities[id])
        })?;
        insert_absent(&entities, &event.id_potion_event_roll, |id| {
            snapshot_potion(state, id)
        })?;
    }
    Ok(PyEntities(entities.unbind()))
}

// Adds each id's snapshot unless a collection read before it already did
fn insert_absent<'a, 'py, T: IntoPyObject<'py>>(
    entities: &Bound<'py, PyDict>,
    ids: impl IntoIterator<Item = &'a usize>,
    snapshot: impl Fn(usize) -> T,
) -> PyResult<()> {
    for &id in ids {
        if !entities.contains(id)? {
            entities.set_item(id, snapshot(id))?;
        }
    }
    Ok(())
}
