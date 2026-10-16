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
    SelectionSkip,
    RestDig,
    RestLift,
    RestToke,
    RestSmith,
    RewardSingingBowl,
    RewardTakeGoldStolen,
    BonfireOffer,
    CardAdopt,
    CardBottle,
    CardDiscard,
    CardDiscoverSelect,
    CardDuplicate,
    CardExhaust,
    CardMove,
    CardNightmareSelect,
    CardPurge,
    CardRetain,
    CardSetupSelect,
    CardTransform,
    CardUpgrade,
    MatchFlipSeen,
    RelicLose,
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
    pub ids: Vec<usize>,
}

#[pymethods]
impl PyAction {
    #[new]
    #[pyo3(signature = (action_type, ids))]
    fn new(action_type: PyActionType, ids: Vec<usize>) -> Self {
        Self { action_type, ids }
    }

    fn __repr__(&self) -> String {
        format!("Action({:?}, {:?})", self.action_type, self.ids)
    }
}

pub fn to_internal_action(action: PyAction) -> Result<Action, String> {
    let ids = &action.ids;
    let i0 = ids.first().copied().unwrap_or(0);
    let internal = match action.action_type {
        PyActionType::CardPlay => Action::CardPlay {
            id_card: i0,
            id_monster: ids.get(1).copied(),
        },
        PyActionType::TurnEnd => Action::TurnEnd,
        PyActionType::SelectionSkip => Action::SelectionSkip,
        PyActionType::RoomSelect => Action::RoomSelect { id_room: i0 },
        PyActionType::ShopBuyCard => Action::ShopBuyCard { id_card: i0 },
        PyActionType::ShopBuyPotion => Action::ShopBuyPotion { id_potion: i0 },
        PyActionType::ShopBuyRelic => Action::ShopBuyRelic { id_relic: i0 },
        PyActionType::ShopPurge => Action::ShopPurge,
        PyActionType::Rest => Action::Rest,
        PyActionType::RestDig => Action::RestDig,
        PyActionType::RestLift => Action::RestLift,
        PyActionType::RestToke => Action::RestToke,
        PyActionType::RestSmith => Action::RestSmith,
        PyActionType::RewardSingingBowl => Action::RewardSingingBowl { id_card: i0 },
        PyActionType::RoomExit => Action::RoomExit,
        PyActionType::ChestOpen => Action::ChestOpen,
        PyActionType::PotionUse => Action::PotionUse {
            id_potion: i0,
            id_monster: ids.get(1).copied(),
        },
        PyActionType::PotionDiscard => Action::PotionDiscard { id_potion: i0 },
        PyActionType::RewardTakeCard => Action::RewardTakeCard { id_card: i0 },
        PyActionType::RewardTakeRelic => Action::RewardTakeRelic { id_relic: i0 },
        PyActionType::RewardTakePotion => Action::RewardTakePotion { id_potion: i0 },
        PyActionType::RewardTakeGold => Action::RewardTakeGold,
        PyActionType::RewardTakeGoldStolen => Action::RewardTakeGoldStolen,
        PyActionType::EventOptionSelect => Action::EventOptionSelect { id_option: i0 },
        PyActionType::BonfireOffer => Action::BonfireOffer { id_card: i0 },
        PyActionType::CardAdopt => Action::CardAdopt { id_card: i0 },
        PyActionType::CardBottle => Action::CardBottle { id_card: i0 },
        PyActionType::CardDiscard => Action::CardDiscard { id_card: i0 },
        PyActionType::CardDiscoverSelect => Action::CardDiscoverSelect { id_card: i0 },
        PyActionType::CardDuplicate => Action::CardDuplicate { id_card: i0 },
        PyActionType::CardExhaust => Action::CardExhaust { id_card: i0 },
        PyActionType::CardMove => Action::CardMove { id_card: i0 },
        PyActionType::CardNightmareSelect => Action::CardNightmareSelect { id_card: i0 },
        PyActionType::CardPurge => Action::CardPurge { id_card: i0 },
        PyActionType::CardRetain => Action::CardRetain { id_card: i0 },
        PyActionType::CardSetupSelect => Action::CardSetupSelect { id_card: i0 },
        PyActionType::CardTransform => Action::CardTransform { id_card: i0 },
        PyActionType::CardUpgrade => Action::CardUpgrade { id_card: i0 },
        PyActionType::MatchFlipSeen => Action::MatchFlipSeen { id_card: i0 },
        PyActionType::RelicLose => Action::RelicLose { id_relic: i0 },
    };

    // Round-trip is the arity gate: a wrong-length ids list cannot rebuild itself
    if from_internal_action(internal.clone()).ids != *ids {
        return Err(format!(
            "{:?} got malformed ids {ids:?}",
            action.action_type
        ));
    }
    Ok(internal)
}

pub fn from_internal_action(action: Action) -> PyAction {
    let (action_type, ids) = match action {
        Action::CardPlay {
            id_card,
            id_monster: None,
        } => (PyActionType::CardPlay, vec![id_card]),
        Action::CardPlay {
            id_card,
            id_monster: Some(id_monster),
        } => (PyActionType::CardPlay, vec![id_card, id_monster]),
        Action::TurnEnd => (PyActionType::TurnEnd, vec![]),
        Action::SelectionSkip => (PyActionType::SelectionSkip, vec![]),
        Action::RoomSelect { id_room } => (PyActionType::RoomSelect, vec![id_room]),
        Action::Rest => (PyActionType::Rest, vec![]),
        Action::RestDig => (PyActionType::RestDig, vec![]),
        Action::RestLift => (PyActionType::RestLift, vec![]),
        Action::RestToke => (PyActionType::RestToke, vec![]),
        Action::RestSmith => (PyActionType::RestSmith, vec![]),
        Action::RewardSingingBowl { id_card } => (PyActionType::RewardSingingBowl, vec![id_card]),
        Action::RoomExit => (PyActionType::RoomExit, vec![]),
        Action::ShopBuyCard { id_card } => (PyActionType::ShopBuyCard, vec![id_card]),
        Action::ShopBuyPotion { id_potion } => (PyActionType::ShopBuyPotion, vec![id_potion]),
        Action::ShopBuyRelic { id_relic } => (PyActionType::ShopBuyRelic, vec![id_relic]),
        Action::ShopPurge => (PyActionType::ShopPurge, vec![]),
        Action::ChestOpen => (PyActionType::ChestOpen, vec![]),
        Action::PotionUse {
            id_potion,
            id_monster: None,
        } => (PyActionType::PotionUse, vec![id_potion]),
        Action::PotionUse {
            id_potion,
            id_monster: Some(id_monster),
        } => (PyActionType::PotionUse, vec![id_potion, id_monster]),
        Action::PotionDiscard { id_potion } => (PyActionType::PotionDiscard, vec![id_potion]),
        Action::RewardTakeCard { id_card } => (PyActionType::RewardTakeCard, vec![id_card]),
        Action::RewardTakeRelic { id_relic } => (PyActionType::RewardTakeRelic, vec![id_relic]),
        Action::RewardTakePotion { id_potion } => (PyActionType::RewardTakePotion, vec![id_potion]),
        Action::RewardTakeGold => (PyActionType::RewardTakeGold, vec![]),
        Action::RewardTakeGoldStolen => (PyActionType::RewardTakeGoldStolen, vec![]),
        Action::EventOptionSelect { id_option } => {
            (PyActionType::EventOptionSelect, vec![id_option])
        }
        Action::BonfireOffer { id_card } => (PyActionType::BonfireOffer, vec![id_card]),
        Action::CardAdopt { id_card } => (PyActionType::CardAdopt, vec![id_card]),
        Action::CardBottle { id_card } => (PyActionType::CardBottle, vec![id_card]),
        Action::CardDiscard { id_card } => (PyActionType::CardDiscard, vec![id_card]),
        Action::CardDiscoverSelect { id_card } => (PyActionType::CardDiscoverSelect, vec![id_card]),
        Action::CardDuplicate { id_card } => (PyActionType::CardDuplicate, vec![id_card]),
        Action::CardExhaust { id_card } => (PyActionType::CardExhaust, vec![id_card]),
        Action::CardMove { id_card } => (PyActionType::CardMove, vec![id_card]),
        Action::CardNightmareSelect { id_card } => {
            (PyActionType::CardNightmareSelect, vec![id_card])
        }
        Action::CardPurge { id_card } => (PyActionType::CardPurge, vec![id_card]),
        Action::CardRetain { id_card } => (PyActionType::CardRetain, vec![id_card]),
        Action::CardSetupSelect { id_card } => (PyActionType::CardSetupSelect, vec![id_card]),
        Action::CardTransform { id_card } => (PyActionType::CardTransform, vec![id_card]),
        Action::CardUpgrade { id_card } => (PyActionType::CardUpgrade, vec![id_card]),
        Action::MatchFlipSeen { id_card } => (PyActionType::MatchFlipSeen, vec![id_card]),
        Action::RelicLose { id_relic } => (PyActionType::RelicLose, vec![id_relic]),
    };
    PyAction { action_type, ids }
}
