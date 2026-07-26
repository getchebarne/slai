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

impl PyActionType {
    fn from_discriminant(discriminant: u8) -> Result<Self, String> {
        match discriminant {
            0 => Ok(Self::CardDiscard),
            1 => Ok(Self::CardDiscover),
            2 => Ok(Self::CardDuplicate),
            3 => Ok(Self::CardNightmare),
            4 => Ok(Self::CardPlay),
            5 => Ok(Self::CardPurge),
            6 => Ok(Self::CardRetain),
            7 => Ok(Self::CardSetup),
            8 => Ok(Self::CardTransform),
            9 => Ok(Self::CardUpgrade),
            10 => Ok(Self::ChestOpen),
            11 => Ok(Self::EventOptionSelect),
            12 => Ok(Self::PotionDiscard),
            13 => Ok(Self::PotionUse),
            14 => Ok(Self::Rest),
            15 => Ok(Self::RewardTakeCard),
            16 => Ok(Self::RewardTakeGold),
            17 => Ok(Self::RewardTakePotion),
            18 => Ok(Self::RewardTakeRelic),
            19 => Ok(Self::RoomExit),
            20 => Ok(Self::RoomSelect),
            21 => Ok(Self::ShopBuyCard),
            22 => Ok(Self::ShopBuyPotion),
            23 => Ok(Self::ShopBuyRelic),
            24 => Ok(Self::ShopPurge),
            25 => Ok(Self::TurnEnd),
            26 => Ok(Self::CardExhaust),
            27 => Ok(Self::CardMoveToHand),
            28 => Ok(Self::PickSkip),
            _ => Err(format!("PyActionType: invalid discriminant {discriminant}")),
        }
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
    #[pyo3(get)]
    pub kind: Option<u8>,
}

#[pymethods]
impl PyAction {
    #[new]
    #[pyo3(signature = (action_type, idxs, kind=None))]
    fn new(action_type: PyActionType, idxs: Vec<usize>, kind: Option<u8>) -> Self {
        Self {
            action_type,
            idxs,
            kind,
        }
    }

    fn __repr__(&self) -> String {
        match self.kind {
            Some(k) => format!(
                "PyAction({:?}, {:?}, kind={})",
                self.action_type, self.idxs, k
            ),
            None => format!("PyAction({:?}, {:?})", self.action_type, self.idxs),
        }
    }
}

pub fn to_internal_action(action: PyAction) -> Result<Action, String> {
    let idxs = &action.idxs;
    match action.action_type {
        PyActionType::CardPlay => match idxs.len() {
            1 => Ok(Action::CardPlay {
                idx_card: idxs[0],
                idx_monster: None,
            }),
            2 => Ok(Action::CardPlay {
                idx_card: idxs[0],
                idx_monster: Some(idxs[1]),
            }),
            n => Err(format!(
                "CardPlay expects [idx_card] or [idx_card, idx_monster], got {n} idxs"
            )),
        },
        PyActionType::TurnEnd => match idxs.len() {
            0 => Ok(Action::TurnEnd),
            n => Err(format!("TurnEnd expects [], got {n} idxs")),
        },
        PyActionType::CardDiscard => match idxs.len() {
            1 => Ok(Action::CardDiscard { idx: idxs[0] }),
            n => Err(format!("CardDiscard expects [idx_hand], got {n} idxs")),
        },
        PyActionType::CardExhaust => match idxs.len() {
            1 => Ok(Action::CardExhaust { idx: idxs[0] }),
            n => Err(format!("CardExhaust expects [idx_hand], got {n} idxs")),
        },
        PyActionType::CardMoveToHand => match idxs.len() {
            1 => Ok(Action::CardMoveToHand { idx: idxs[0] }),
            n => Err(format!("CardMoveToHand expects [idx_pile_draw], got {n} idxs")),
        },
        PyActionType::PickSkip => match idxs.len() {
            0 => Ok(Action::PickSkip),
            n => Err(format!("PickSkip expects [], got {n} idxs")),
        },
        PyActionType::CardRetain => match idxs.len() {
            1 => Ok(Action::CardRetain { idx: idxs[0] }),
            n => Err(format!("CardRetain expects [idx_hand], got {n} idxs")),
        },
        PyActionType::CardSetup => match idxs.len() {
            1 => Ok(Action::CardSetup { idx: idxs[0] }),
            n => Err(format!("CardSetup expects [idx_hand], got {n} idxs")),
        },
        PyActionType::CardNightmare => match idxs.len() {
            1 => Ok(Action::CardNightmare { idx: idxs[0] }),
            n => Err(format!("CardNightmare expects [idx_hand], got {n} idxs")),
        },
        PyActionType::RoomSelect => match idxs.len() {
            1 => Ok(Action::RoomSelect { idx: idxs[0] }),
            n => Err(format!("RoomSelect expects [idx], got {n} idxs")),
        },
        PyActionType::ShopBuyCard => match idxs.len() {
            1 => Ok(Action::ShopBuyCard { idx: idxs[0] }),
            n => Err(format!("ShopBuyCard expects [idx], got {n} idxs")),
        },
        PyActionType::ShopBuyPotion => match idxs.len() {
            1 => Ok(Action::ShopBuyPotion { idx: idxs[0] }),
            n => Err(format!("ShopBuyPotion expects [idx], got {n} idxs")),
        },
        PyActionType::ShopBuyRelic => match idxs.len() {
            1 => Ok(Action::ShopBuyRelic { idx: idxs[0] }),
            n => Err(format!("ShopBuyRelic expects [idx], got {n} idxs")),
        },
        PyActionType::ShopPurge => match idxs.len() {
            1 => Ok(Action::ShopPurge { idx: idxs[0] }),
            n => Err(format!("ShopPurge expects [idx], got {n} idxs")),
        },
        PyActionType::Rest => match idxs.len() {
            0 => Ok(Action::Rest),
            n => Err(format!("Rest expects [], got {n} idxs")),
        },
        PyActionType::RoomExit => match idxs.len() {
            0 => Ok(Action::RoomExit),
            n => Err(format!("RoomExit expects [], got {n} idxs")),
        },
        PyActionType::ChestOpen => match idxs.len() {
            0 => Ok(Action::ChestOpen),
            n => Err(format!("ChestOpen expects [], got {n} idxs")),
        },
        PyActionType::PotionUse => match idxs.len() {
            1 => Ok(Action::PotionUse {
                idx_potion: idxs[0],
                idx_monster: None,
            }),
            2 => Ok(Action::PotionUse {
                idx_potion: idxs[0],
                idx_monster: Some(idxs[1]),
            }),
            n => Err(format!(
                "PotionUse expects [idx_potion] or [idx_potion, idx_monster], got {n} idxs"
            )),
        },
        PyActionType::PotionDiscard => match idxs.len() {
            1 => Ok(Action::PotionDiscard { idx: idxs[0] }),
            n => Err(format!("PotionDiscard expects [idx_slot], got {n} idxs")),
        },
        PyActionType::CardDiscover => match idxs.len() {
            1 => Ok(Action::CardDiscover { idx: idxs[0] }),
            n => Err(format!("CardDiscover expects [idx], got {n} idxs")),
        },
        PyActionType::RewardTakeCard => match idxs.len() {
            1 => Ok(Action::RewardTakeCard { idx: idxs[0] }),
            n => Err(format!("RewardTakeCard expects [idx], got {n} idxs")),
        },
        PyActionType::RewardTakeRelic => match idxs.len() {
            0 => Ok(Action::RewardTakeRelic),
            n => Err(format!("RewardTakeRelic expects [], got {n} idxs")),
        },
        PyActionType::RewardTakePotion => match idxs.len() {
            1 => Ok(Action::RewardTakePotion { idx: idxs[0] }),
            n => Err(format!("RewardTakePotion expects [idx], got {n} idxs")),
        },
        PyActionType::RewardTakeGold => match idxs.len() {
            0 => Ok(Action::RewardTakeGold),
            n => Err(format!("RewardTakeGold expects [], got {n} idxs")),
        },
        PyActionType::EventOptionSelect => match idxs.len() {
            1 => Ok(Action::EventOptionSelect { idx: idxs[0] }),
            n => Err(format!("EventOptionSelect expects [idx], got {n} idxs")),
        },
        PyActionType::CardPurge => match idxs.len() {
            1 => Ok(Action::CardPurge { idx: idxs[0] }),
            n => Err(format!("CardPurge expects [idx], got {n} idxs")),
        },
        PyActionType::CardUpgrade => match idxs.len() {
            1 => Ok(Action::CardUpgrade { idx: idxs[0] }),
            n => Err(format!("CardUpgrade expects [idx], got {n} idxs")),
        },
        PyActionType::CardDuplicate => match idxs.len() {
            1 => Ok(Action::CardDuplicate { idx: idxs[0] }),
            n => Err(format!("CardDuplicate expects [idx], got {n} idxs")),
        },
        PyActionType::CardTransform => match idxs.len() {
            1 => Ok(Action::CardTransform { idx: idxs[0] }),
            n => Err(format!("CardTransform expects [idx], got {n} idxs")),
        },
    }
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
        Action::PickSkip => (PyActionType::PickSkip, vec![]),
        Action::CardRetain { idx } => (PyActionType::CardRetain, vec![idx]),
        Action::CardSetup { idx } => (PyActionType::CardSetup, vec![idx]),
        Action::CardNightmare { idx } => (PyActionType::CardNightmare, vec![idx]),
        Action::RoomSelect { idx } => (PyActionType::RoomSelect, vec![idx]),
        Action::Rest => (PyActionType::Rest, vec![]),
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
        Action::RewardTakeCard { idx } => (PyActionType::RewardTakeCard, vec![idx]),
        Action::RewardTakeRelic => (PyActionType::RewardTakeRelic, vec![]),
        Action::RewardTakePotion { idx } => (PyActionType::RewardTakePotion, vec![idx]),
        Action::RewardTakeGold => (PyActionType::RewardTakeGold, vec![]),
        Action::EventOptionSelect { idx } => (PyActionType::EventOptionSelect, vec![idx]),
        Action::CardPurge { idx } => (PyActionType::CardPurge, vec![idx]),
        Action::CardUpgrade { idx } => (PyActionType::CardUpgrade, vec![idx]),
        Action::CardDuplicate { idx } => (PyActionType::CardDuplicate, vec![idx]),
        Action::CardTransform { idx } => (PyActionType::CardTransform, vec![idx]),
    };
    PyAction {
        action_type,
        idxs,
        kind: None,
    }
}
