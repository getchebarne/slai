use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::type_hint_union;
use pyo3::type_object::PyTypeInfo;

use crate::game::GameState;
use crate::types::ChestKind;
use crate::types::Frame;

use super::card::PyCard;
use super::card::snapshot_card;
use super::effect::PyEffect;
use super::effect::snapshot_effect;
use super::event::PyEventKind;
use super::event::snapshot_event_kind;
use super::macros::flat_variants;
use super::macros::mirror_enum;
use super::monster::PyMonster;
use super::monster::snapshot_monsters;
use super::potion::PyPotion;
use super::potion::snapshot_potion;
use super::relic::PyRelic;
use super::relic::snapshot_relic;

mirror_enum!(PyChestKind from ChestKind, "ChestKind", skip_from_py_object, {
    Small, Medium, Large,
});

flat_variants!(PyFrame {
    Map => PyFrameMap as "FrameMap",
    RestSite => PyFrameRestSite as "FrameRestSite" { done: bool },
    Chest => PyFrameChest as "FrameChest" { kind: PyChestKind, opened: bool },
    Combat => PyFrameCombat as "FrameCombat" { hand: Vec<PyCard>, pile_draw: Vec<PyCard>, pile_discard: Vec<PyCard>, pile_exhaust: Vec<PyCard>, pile_stasis: Vec<PyCard>, energy: PyEnergy, monsters: Vec<PyMonster>, discover: Vec<PyCard>, bomb_countdown: u8 },
    Reward => PyFrameReward as "FrameReward" { cards: Vec<Vec<PyCard>>, relics: Vec<PyRelic>, potions: Vec<PyPotion>, gold: Option<u16> },
    Shop => PyFrameShop as "FrameShop" { cards: Vec<PyCard>, card_prices: Vec<u16>, relics: Vec<PyRelic>, relic_prices: Vec<u16>, potions: Vec<PyPotion>, potion_prices: Vec<u16>, purge_cost: u16, purged: bool },
    Event => PyFrameEvent as "FrameEvent" { kind: PyEventKind, options: Vec<Vec<PyEffect>>, consumed: bool },
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
    pub energy_current: u8,
    pub energy_max: u8,
}

pub(crate) fn snapshot_frame(state: &GameState, frame: &Frame) -> PyFrame {
    match frame {
        Frame::Map => PyFrame::Map(PyFrameMap),
        Frame::RestSite { consumed } => PyFrame::RestSite(PyFrameRestSite { done: *consumed }),
        Frame::Chest {
            chest_kind,
            chest_opened,
        } => PyFrame::Chest(PyFrameChest {
            kind: (*chest_kind).into(),
            opened: *chest_opened,
        }),
        Frame::Combat {
            id_hand,
            id_pile_draw,
            id_pile_discard,
            id_pile_exhaust,
            id_stasis_cards,
            energy,
            id_discover,
            bomb_countdown,
            ..
        } => PyFrame::Combat(PyFrameCombat {
            hand: id_hand.iter().map(|&id| snapshot_card(state, id)).collect(),
            pile_draw: id_pile_draw
                .iter()
                .map(|&id| snapshot_card(state, id))
                .collect(),
            pile_discard: id_pile_discard
                .iter()
                .map(|&id| snapshot_card(state, id))
                .collect(),
            pile_exhaust: id_pile_exhaust
                .iter()
                .map(|&id| snapshot_card(state, id))
                .collect(),
            pile_stasis: id_stasis_cards
                .iter()
                .flatten()
                .map(|&id| snapshot_card(state, id))
                .collect(),
            energy: PyEnergy {
                energy_current: energy.energy_current,
                energy_max: energy.energy_max,
            },
            monsters: snapshot_monsters(state),
            discover: id_discover
                .iter()
                .map(|&id| snapshot_card(state, id))
                .collect(),
            bomb_countdown: *bomb_countdown,
        }),
        Frame::Reward {
            id_cards,
            id_relics,
            id_potions,
            gold,
            ..
        } => PyFrame::Reward(PyFrameReward {
            cards: id_cards
                .iter()
                .map(|bundle| bundle.iter().map(|&id| snapshot_card(state, id)).collect())
                .collect(),
            relics: id_relics
                .iter()
                .map(|&id| snapshot_relic(&state.entities[id]))
                .collect(),
            potions: id_potions
                .iter()
                .map(|&id| snapshot_potion(&state.entities[id]))
                .collect(),
            gold: *gold,
        }),
        Frame::Shop {
            cards,
            relics,
            potions,
            purge_cost,
            purged,
        } => PyFrame::Shop(PyFrameShop {
            cards: cards
                .iter()
                .map(|&(id, _)| snapshot_card(state, id))
                .collect(),
            card_prices: cards.iter().map(|&(_, price)| price).collect(),
            relics: relics
                .iter()
                .map(|&(id, _)| snapshot_relic(&state.entities[id]))
                .collect(),
            relic_prices: relics.iter().map(|&(_, price)| price).collect(),
            potions: potions
                .iter()
                .map(|&(id, _)| snapshot_potion(&state.entities[id]))
                .collect(),
            potion_prices: potions.iter().map(|&(_, price)| price).collect(),
            purge_cost: *purge_cost,
            purged: *purged,
        }),
        Frame::Event {
            kind,
            consumed,
            id_options,
        } => PyFrame::Event(PyFrameEvent {
            kind: snapshot_event_kind(state, *kind),
            options: id_options
                .iter()
                .map(|&id| {
                    let option = &state.entities[id];
                    option.event_option_effects[..option.event_option_effects_len as usize]
                        .iter()
                        .map(snapshot_effect)
                        .collect()
                })
                .collect(),
            consumed: *consumed,
        }),
    }
}
