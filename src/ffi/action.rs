use pyo3::prelude::*;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::IntoStaticStr)]
pub enum PyActionType {
    CardDiscard,
    CardDiscover,
    CardDuplicate,
    CardNightmare,
    CardPlay,
    CardPurge,
    CardRetain,
    CardSetup,
    CardTransform,
    CardUpgrade,
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
    CardExhaust,
    CardMoveToHand,
    PickSkip,
    RestDig,
    RestLift,
    RestToke,
    RewardSingingBowl,
    CardBottle,
}

#[pymethods]
impl PyActionType {
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
        PyActionType::CardDiscard => Action::CardDiscard { idx: i0 },
        PyActionType::CardExhaust => Action::CardExhaust { idx: i0 },
        PyActionType::CardMoveToHand => Action::CardMoveToHand { idx: i0 },
        PyActionType::PickSkip => Action::PickSkip,
        PyActionType::CardRetain => Action::CardRetain { idx: i0 },
        PyActionType::CardSetup => Action::CardSetup { idx: i0 },
        PyActionType::CardNightmare => Action::CardNightmare { idx: i0 },
        PyActionType::RoomSelect => Action::RoomSelect { idx: i0 },
        PyActionType::ShopBuyCard => Action::ShopBuyCard { idx: i0 },
        PyActionType::ShopBuyPotion => Action::ShopBuyPotion { idx: i0 },
        PyActionType::ShopBuyRelic => Action::ShopBuyRelic { idx: i0 },
        PyActionType::ShopPurge => Action::ShopPurge { idx: i0 },
        PyActionType::Rest => Action::Rest,
        PyActionType::RestDig => Action::RestDig,
        PyActionType::RestLift => Action::RestLift,
        PyActionType::RestToke => Action::RestToke,
        PyActionType::CardBottle => Action::CardBottle { idx: i0 },
        PyActionType::RewardSingingBowl => Action::RewardSingingBowl { idx_bundle: i0 },
        PyActionType::RoomExit => Action::RoomExit,
        PyActionType::ChestOpen => Action::ChestOpen,
        PyActionType::PotionUse => Action::PotionUse {
            idx_potion: i0,
            idx_monster: idxs.get(1).copied(),
        },
        PyActionType::PotionDiscard => Action::PotionDiscard { idx: i0 },
        PyActionType::CardDiscover => Action::CardDiscover { idx: i0 },
        PyActionType::RewardTakeCard => Action::RewardTakeCard {
            idx_bundle: i0,
            idx_card: i1,
        },
        PyActionType::RewardTakeRelic => Action::RewardTakeRelic { idx: i0 },
        PyActionType::RewardTakePotion => Action::RewardTakePotion { idx: i0 },
        PyActionType::RewardTakeGold => Action::RewardTakeGold,
        PyActionType::EventOptionSelect => Action::EventOptionSelect { idx: i0 },
        PyActionType::CardPurge => Action::CardPurge { idx: i0 },
        PyActionType::CardUpgrade => Action::CardUpgrade { idx: i0 },
        PyActionType::CardDuplicate => Action::CardDuplicate { idx: i0 },
        PyActionType::CardTransform => Action::CardTransform { idx: i0 },
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
        Action::CardDiscard { idx } => (PyActionType::CardDiscard, vec![idx]),
        Action::CardExhaust { idx } => (PyActionType::CardExhaust, vec![idx]),
        Action::CardMoveToHand { idx } => (PyActionType::CardMoveToHand, vec![idx]),
        Action::CardBottle { idx } => (PyActionType::CardBottle, vec![idx]),
        Action::PickSkip => (PyActionType::PickSkip, vec![]),
        Action::CardRetain { idx } => (PyActionType::CardRetain, vec![idx]),
        Action::CardSetup { idx } => (PyActionType::CardSetup, vec![idx]),
        Action::CardNightmare { idx } => (PyActionType::CardNightmare, vec![idx]),
        Action::RoomSelect { idx } => (PyActionType::RoomSelect, vec![idx]),
        Action::Rest => (PyActionType::Rest, vec![]),
        Action::RestDig => (PyActionType::RestDig, vec![]),
        Action::RestLift => (PyActionType::RestLift, vec![]),
        Action::RestToke => (PyActionType::RestToke, vec![]),
        Action::RewardSingingBowl { idx_bundle } => {
            (PyActionType::RewardSingingBowl, vec![idx_bundle])
        }
        Action::RoomExit => (PyActionType::RoomExit, vec![]),
        Action::ShopBuyCard { idx } => (PyActionType::ShopBuyCard, vec![idx]),
        Action::ShopBuyPotion { idx } => (PyActionType::ShopBuyPotion, vec![idx]),
        Action::ShopBuyRelic { idx } => (PyActionType::ShopBuyRelic, vec![idx]),
        Action::ShopPurge { idx } => (PyActionType::ShopPurge, vec![idx]),
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
        Action::CardDiscover { idx } => (PyActionType::CardDiscover, vec![idx]),
        Action::RewardTakeCard {
            idx_bundle,
            idx_card,
        } => (PyActionType::RewardTakeCard, vec![idx_bundle, idx_card]),
        Action::RewardTakeRelic { idx } => (PyActionType::RewardTakeRelic, vec![idx]),
        Action::RewardTakePotion { idx } => (PyActionType::RewardTakePotion, vec![idx]),
        Action::RewardTakeGold => (PyActionType::RewardTakeGold, vec![]),
        Action::EventOptionSelect { idx } => (PyActionType::EventOptionSelect, vec![idx]),
        Action::CardPurge { idx } => (PyActionType::CardPurge, vec![idx]),
        Action::CardUpgrade { idx } => (PyActionType::CardUpgrade, vec![idx]),
        Action::CardDuplicate { idx } => (PyActionType::CardDuplicate, vec![idx]),
        Action::CardTransform { idx } => (PyActionType::CardTransform, vec![idx]),
    };
    PyAction { action_type, idxs }
}
