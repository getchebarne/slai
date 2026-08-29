use pyo3::prelude::*;
use strum::IntoEnumIterator;

use crate::action::Action;

// `PyActionType` is the discriminant for the flat `PyAction` struct below
#[pyclass(
    from_py_object,
    eq,
    eq_int,
    frozen,
    name = "ActionType",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::IntoStaticStr, strum::EnumIter)]
pub enum PyActionType {
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
    PickSkip,
    RestDig,
    RestLift,
    RestToke,
    RestSmith,
    RewardSingingBowl,
    EffectPendingResolve,
}

#[pymethods]
impl PyActionType {
    // Declaration order, which is also int() order
    #[staticmethod]
    fn members() -> Vec<PyActionType> {
        PyActionType::iter().collect()
    }

    // variant name for the action-spec registry (raw pyo3 enums have no .name)
    #[getter]
    fn name(&self) -> &'static str {
        self.into()
    }

    // hash by discriminant so eq and hash agree (see impl_discriminant_hash below)
    fn __hash__(&self) -> isize {
        *self as isize
    }
}

#[pyclass(
    from_py_object,
    eq,
    hash,
    frozen,
    name = "Action",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyAction {
    #[pyo3(get)]
    pub action_type: PyActionType,
    #[pyo3(get)]
    pub idxs: Vec<usize>,
}

#[pymethods]
impl PyAction {
    #[new]
    #[pyo3(signature = (action_type, idxs))]
    fn new(action_type: PyActionType, idxs: Vec<usize>) -> Self {
        Self { action_type, idxs }
    }

    fn __repr__(&self) -> String {
        format!("PyAction({:?}, {:?})", self.action_type, self.idxs)
    }
}

pub fn to_internal_action(action: PyAction) -> Result<Action, String> {
    let idxs = &action.idxs;
    let i0 = idxs.first().copied().unwrap_or(0);
    let i1 = idxs.get(1).copied().unwrap_or(0);
    let internal = match action.action_type {
        PyActionType::CardPlay => Action::CardPlay {
            idx_card: i0,
            idx_monster: idxs.get(1).copied(),
        },
        PyActionType::TurnEnd => Action::TurnEnd,
        PyActionType::PickSkip => Action::PickSkip,
        PyActionType::EffectPendingResolve => Action::EffectPendingResolve { idx: i0 },
        PyActionType::RoomSelect => Action::RoomSelect { idx: i0 },
        PyActionType::ShopBuyCard => Action::ShopBuyCard { idx: i0 },
        PyActionType::ShopBuyPotion => Action::ShopBuyPotion { idx: i0 },
        PyActionType::ShopBuyRelic => Action::ShopBuyRelic { idx: i0 },
        PyActionType::ShopPurge => Action::ShopPurge,
        PyActionType::Rest => Action::Rest,
        PyActionType::RestDig => Action::RestDig,
        PyActionType::RestLift => Action::RestLift,
        PyActionType::RestToke => Action::RestToke,
        PyActionType::RestSmith => Action::RestSmith,
        PyActionType::RewardSingingBowl => Action::RewardSingingBowl { idx_bundle: i0 },
        PyActionType::RoomExit => Action::RoomExit,
        PyActionType::ChestOpen => Action::ChestOpen,
        PyActionType::PotionUse => Action::PotionUse {
            idx_potion: i0,
            idx_monster: idxs.get(1).copied(),
        },
        PyActionType::PotionDiscard => Action::PotionDiscard { idx: i0 },
        PyActionType::RewardTakeCard => Action::RewardTakeCard {
            idx_bundle: i0,
            idx_card: i1,
        },
        PyActionType::RewardTakeRelic => Action::RewardTakeRelic { idx: i0 },
        PyActionType::RewardTakePotion => Action::RewardTakePotion { idx: i0 },
        PyActionType::RewardTakeGold => Action::RewardTakeGold,
        PyActionType::EventOptionSelect => Action::EventOptionSelect { idx: i0 },
    };

    // Round-trip is the arity gate: a wrong-length idxs list cannot rebuild itself
    if from_internal_action(internal.clone()).idxs != *idxs {
        return Err(format!(
            "{:?} got malformed idxs {idxs:?}",
            action.action_type
        ));
    }
    Ok(internal)
}

pub fn from_internal_action(action: Action) -> PyAction {
    let (action_type, idxs) = match action {
        Action::CardPlay {
            idx_card,
            idx_monster: None,
        } => (PyActionType::CardPlay, vec![idx_card]),
        Action::CardPlay {
            idx_card,
            idx_monster: Some(m),
        } => (PyActionType::CardPlay, vec![idx_card, m]),
        Action::TurnEnd => (PyActionType::TurnEnd, vec![]),
        Action::PickSkip => (PyActionType::PickSkip, vec![]),
        Action::EffectPendingResolve { idx } => (PyActionType::EffectPendingResolve, vec![idx]),
        Action::RoomSelect { idx } => (PyActionType::RoomSelect, vec![idx]),
        Action::Rest => (PyActionType::Rest, vec![]),
        Action::RestDig => (PyActionType::RestDig, vec![]),
        Action::RestLift => (PyActionType::RestLift, vec![]),
        Action::RestToke => (PyActionType::RestToke, vec![]),
        Action::RestSmith => (PyActionType::RestSmith, vec![]),
        Action::RewardSingingBowl { idx_bundle } => {
            (PyActionType::RewardSingingBowl, vec![idx_bundle])
        }
        Action::RoomExit => (PyActionType::RoomExit, vec![]),
        Action::ShopBuyCard { idx } => (PyActionType::ShopBuyCard, vec![idx]),
        Action::ShopBuyPotion { idx } => (PyActionType::ShopBuyPotion, vec![idx]),
        Action::ShopBuyRelic { idx } => (PyActionType::ShopBuyRelic, vec![idx]),
        Action::ShopPurge => (PyActionType::ShopPurge, vec![]),
        Action::ChestOpen => (PyActionType::ChestOpen, vec![]),
        Action::PotionUse {
            idx_potion,
            idx_monster: None,
        } => (PyActionType::PotionUse, vec![idx_potion]),
        Action::PotionUse {
            idx_potion,
            idx_monster: Some(m),
        } => (PyActionType::PotionUse, vec![idx_potion, m]),
        Action::PotionDiscard { idx } => (PyActionType::PotionDiscard, vec![idx]),
        Action::RewardTakeCard {
            idx_bundle,
            idx_card,
        } => (PyActionType::RewardTakeCard, vec![idx_bundle, idx_card]),
        Action::RewardTakeRelic { idx } => (PyActionType::RewardTakeRelic, vec![idx]),
        Action::RewardTakePotion { idx } => (PyActionType::RewardTakePotion, vec![idx]),
        Action::RewardTakeGold => (PyActionType::RewardTakeGold, vec![]),
        Action::EventOptionSelect { idx } => (PyActionType::EventOptionSelect, vec![idx]),
    };
    PyAction { action_type, idxs }
}
