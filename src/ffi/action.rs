use pyo3::prelude::*;
use strum::IntoEnumIterator;

use crate::action::Action;
use crate::action::ActionKind;

use super::effect::PyEffect;
use super::effect::snapshot_effect;

// `PyActionKind` is the discriminant for the flat `PyAction` struct below
#[pyclass(
    from_py_object,
    eq,
    eq_int,
    frozen,
    name = "ActionKind",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::IntoStaticStr, strum::EnumIter)]
pub enum PyActionKind {
    CardPlay,
    ChestOpen,
    EventOptionSelect,
    PotionDiscard,
    PotionUse,
    Rest,
    RewardTakeCard,
    RewardTakeGold,
    RewardTakePotion,
    RewardTakeRelic,
    RoomExit,
    RoomSelect,
    ShopBuyCard,
    ShopBuyPotion,
    ShopBuyRelic,
    ShopPurge,
    TurnEnd,
    SelectionSkip,
    RestDig,
    RestLift,
    RestToke,
    RestSmith,
    RewardSingingBowl,
    EffectPendingResolve,
}

#[pymethods]
impl PyActionKind {
    // Declaration order, which is also int() order
    #[staticmethod]
    fn members() -> Vec<PyActionKind> {
        PyActionKind::iter().collect()
    }

    // Variant name (raw pyo3 enums have no .name)
    #[getter]
    fn name(&self) -> &'static str {
        self.into()
    }

    // hash by discriminant so eq and hash agree (see impl_discriminant_hash below)
    fn __hash__(&self) -> isize {
        *self as isize
    }
}

// A legal action: its kind, the answers to its chain's picks in order, and that chain. The
// engine lists these; the client picks one by index and never builds one
#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "Action",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyAction {
    pub action_kind: PyActionKind,
    pub id_input: Vec<usize>,
    pub effects: Vec<PyEffect>,
}

#[pymethods]
impl PyAction {
    fn __repr__(&self) -> String {
        format!("PyAction({:?}, {:?})", self.action_kind, self.id_input)
    }
}

pub fn from_internal_action(action: &Action) -> PyAction {
    PyAction {
        action_kind: action.kind.into(),
        id_input: action.id_input.clone(),
        effects: action.effects.iter().map(snapshot_effect).collect(),
    }
}

// Declared in the same order; the kind is the int
impl From<PyActionKind> for ActionKind {
    fn from(action_kind: PyActionKind) -> Self {
        let kind = ActionKind::iter()
            .nth(action_kind as usize)
            .expect("Python ActionKind outside the engine's");
        debug_assert!(
            PyActionKind::from(kind) == action_kind,
            "engine ActionKind order != Python ActionKind order"
        );
        kind
    }
}

impl From<ActionKind> for PyActionKind {
    fn from(kind: ActionKind) -> Self {
        match kind {
            ActionKind::CardPlay => Self::CardPlay,
            ActionKind::ChestOpen => Self::ChestOpen,
            ActionKind::EventOptionSelect => Self::EventOptionSelect,
            ActionKind::PotionDiscard => Self::PotionDiscard,
            ActionKind::PotionUse => Self::PotionUse,
            ActionKind::Rest => Self::Rest,
            ActionKind::RewardTakeCard => Self::RewardTakeCard,
            ActionKind::RewardTakeGold => Self::RewardTakeGold,
            ActionKind::RewardTakePotion => Self::RewardTakePotion,
            ActionKind::RewardTakeRelic => Self::RewardTakeRelic,
            ActionKind::RoomExit => Self::RoomExit,
            ActionKind::RoomSelect => Self::RoomSelect,
            ActionKind::ShopBuyCard => Self::ShopBuyCard,
            ActionKind::ShopBuyPotion => Self::ShopBuyPotion,
            ActionKind::ShopBuyRelic => Self::ShopBuyRelic,
            ActionKind::ShopPurge => Self::ShopPurge,
            ActionKind::TurnEnd => Self::TurnEnd,
            ActionKind::SelectionSkip => Self::SelectionSkip,
            ActionKind::RestDig => Self::RestDig,
            ActionKind::RestLift => Self::RestLift,
            ActionKind::RestToke => Self::RestToke,
            ActionKind::RestSmith => Self::RestSmith,
            ActionKind::RewardSingingBowl => Self::RewardSingingBowl,
            ActionKind::EffectPendingResolve => Self::EffectPendingResolve,
        }
    }
}
