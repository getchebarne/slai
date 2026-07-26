#![allow(dead_code)]

use pyo3::prelude::*;

mod action;
mod cards;
mod character;
mod consts;
mod effect;
mod engine;
mod entity;
mod events;
mod ffi;
mod game;
mod map;
mod modifier;
mod monsters;
mod potions;
mod relics;
mod types;
mod utils;

use ffi::PyAction;
use ffi::PyGameState;
use ffi::from_internal_action;
use ffi::snapshot_state;
use ffi::to_internal_action;
use game::create_game_state;
use game::step;

// GameEnv

#[pyclass(module = "slai.slai")]
struct GameEnv {
    state: game::GameState,
}

#[pymethods]
impl GameEnv {
    // Game-shape constants — mirror of `crate::consts` for encoders/wrappers
    #[classattr]
    const MAX_MONSTERS: usize = consts::MAX_MONSTERS;
    #[classattr]
    const MAX_SIZE_HAND: usize = consts::MAX_SIZE_HAND;
    #[classattr]
    const MAX_COMBAT_CARD_REWARD: usize = consts::MAX_COMBAT_CARD_REWARD;
    #[classattr]
    const CARDS_DRAWN_PER_TURN: u16 = consts::CARDS_DRAWN_PER_TURN;
    #[classattr]
    const NIGHTMARE_COPIES: u8 = consts::NIGHTMARE_COPIES;
    #[classattr]
    const MAX_BLOCK: u16 = consts::MAX_BLOCK;
    #[classattr]
    const MAP_HEIGHT: usize = consts::MAP_HEIGHT;
    #[classattr]
    const MAP_WIDTH: usize = consts::MAP_WIDTH;

    #[new]
    #[pyo3(signature = (ascension=0, fast_mode=false))]
    fn new(ascension: u8, fast_mode: bool) -> Self {
        // Placeholder seed; consumers must call `reset(seed=...)` before stepping (gymnasium convention)
        let state = create_game_state(ascension, 0, fast_mode);
        GameEnv { state }
    }

    // Start a fresh run. Returns the initial obs
    #[pyo3(signature = (seed=42))]
    fn reset(&mut self, seed: u64) -> PyGameState {
        let asc = self.state.ascension;
        self.state = create_game_state(asc, seed, self.state.fast_mode);
        snapshot_state(&self.state)
    }

    // Apply an action. Returns `(obs, terminated)`
    fn step(&mut self, action: PyAction) -> PyResult<(PyGameState, bool)> {
        let internal =
            to_internal_action(action).map_err(pyo3::exceptions::PyValueError::new_err)?;
        step(&mut self.state, internal).map_err(pyo3::exceptions::PyValueError::new_err)?;
        let obs = snapshot_state(&self.state);
        Ok((obs, self.state.game_over))
    }

    // Cached at every settle point; empty when game_over
    fn get_legal_actions(&self) -> Vec<PyAction> {
        self.state
            .legal_actions
            .iter()
            .cloned()
            .map(from_internal_action)
            .collect()
    }
}

#[pymodule]
mod slai {
    #[pymodule_export]
    use super::GameEnv;
    // Action layer
    #[pymodule_export]
    use super::ffi::{PyAction, PyActionType};
    // Snapshot views
    #[pymodule_export]
    use super::ffi::{
        PyCard, PyCharacter, PyEnergy, PyGameState, PyIntent, PyMap, PyModifier, PyMonster,
        PyPotion, PyRelic, PyRoom, PyTarget,
    };
    // Unit enums
    #[pymodule_export]
    use super::ffi::{
        PyCandidatePoolCardFilter, PyCandidatePoolMonstersFilter, PyCardColor, PyCardKind,
        PyCardName, PyCardRarity, PyDeltaSign, PyIntentKind, PyModifierKind, PyMonsterEncounter,
        PyMonsterName, PyPlayRestriction, PyPotionName, PyPotionRarity, PyRelicName, PyRelicTier,
        PyRoomKind,
    };
    // Flat variant classes (Python-side union aliases live in python/slai/__init__.py)
    #[pymodule_export]
    use super::ffi::{PyAmountAbsolute, PyAmountEventGoldAsk, PyAmountRange, PyAmountRelative};
    #[pymodule_export]
    use super::ffi::{
        PyCandidatePoolCharacter, PyCandidatePoolDeck, PyCandidatePoolDiscover,
        PyCandidatePoolEventPickCard, PyCandidatePoolEventPickPotion, PyCandidatePoolHand,
        PyCandidatePoolMonsters, PyCandidatePoolSource, PySelectionKindAll, PySelectionKindInput,
        PySelectionKindRandom, PySelectionKindSingle,
    };
    #[pymodule_export]
    use super::ffi::{
        PyCardCostKindFixed, PyCardCostKindGrowsOnDamageInstanceTaken,
        PyCardCostKindMinusDiscardsThisTurn, PyCardCostKindXCost,
    };
    #[pymodule_export]
    use super::ffi::{
        PyEffectAdventurerSearch, PyEffectBlockGain, PyEffectBonfireOffer,
        PyEffectCalculatedGamble, PyEffectCardAddToDeck, PyEffectCardAddToHand,
        PyEffectCardDiscard, PyEffectCardDiscoverPick, PyEffectCardDiscoverRoll, PyEffectCardDraw,
        PyEffectCardDrawUpTo, PyEffectCardDuplicate, PyEffectCardNightmarePick, PyEffectCardPurge,
        PyEffectCardRetain, PyEffectCardSetupPick, PyEffectCardTransform, PyEffectCardUpgrade,
        PyEffectCombatStart, PyEffectDamageFinisher, PyEffectDamageFlechettes,
        PyEffectDamageMindBlast, PyEffectDamagePhysical, PyEffectDamagePhysicalIfPoisoned,
        PyEffectDistractionAdd, PyEffectEnergyGain, PyEffectEscapePlanCheck,
        PyEffectEventAdvanceState, PyEffectEventConsume, PyEffectFaceTrade,
        PyEffectGlassKnifeDecay, PyEffectGoldDelta, PyEffectHealthDelta, PyEffectHeelHookProc,
        PyEffectMaxHealthDelta, PyEffectModifierGain, PyEffectModifierMultiply,
        PyEffectModifierRemove, PyEffectMonsterSpawn, PyEffectPotionAddRandom,
        PyEffectPotionDiscard, PyEffectRelicGrantRandom, PyEffectRelicGrantSpecific,
        PyEffectRewardRollPotions, PyEffectScrapOozeReach, PyEffectSetCostOverride,
        PyEffectShuffleDiscardPileIntoDrawPile, PyEffectSneakyStrikeProc, PyEffectStormOfSteelProc,
        PyEffectUnloadDiscard, PyEffectWheelSpin,
    };
    #[pymodule_export]
    use super::ffi::{
        PyEventKindBigFish, PyEventKindBonfireSpirits, PyEventKindDeadAdventurer,
        PyEventKindDuplicator, PyEventKindFaceTrader, PyEventKindGoldenIdol,
        PyEventKindGoldenShrine, PyEventKindLivingWall, PyEventKindMushrooms,
        PyEventKindOminousForge, PyEventKindPurifier, PyEventKindScrapOoze,
        PyEventKindShiningLight, PyEventKindTheCleric, PyEventKindTheDivineFountain,
        PyEventKindTheLab, PyEventKindTheSsssserpent, PyEventKindTheWomanInBlue,
        PyEventKindTransmogrifier, PyEventKindUpgradeShrine, PyEventKindWeMeetAgain,
        PyEventKindWheelOfChange, PyEventKindWingStatue, PyEventKindWorldOfGoop,
    };
    #[pymodule_export]
    use super::ffi::{
        PyModeChest, PyModeChestOpened, PyModeCombat, PyModeCombatEnded, PyModeEvent, PyModeMap,
        PyModeRestSite, PyModeReward, PyModeShop,
    };
}
