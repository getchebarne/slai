#![allow(dead_code)]
// type_hint_union! folds one CTFE frame per PyEffect variant; the default 128 overflows
#![recursion_limit = "512"]

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
    const MAX_REWARD_CARD_BUNDLES: usize = consts::MAX_REWARD_CARD_BUNDLES;
    #[classattr]
    const ORRERY_BUNDLE_COUNT: usize = consts::ORRERY_BUNDLE_COUNT;
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
    #[pyo3(signature = (ascension=0, fast_mode=false, neow=false))]
    fn new(ascension: u8, fast_mode: bool, neow: bool) -> Self {
        // Placeholder seed; consumers must call `reset(seed=...)` before stepping (gymnasium convention)
        let state = create_game_state(ascension, 0, fast_mode, neow);
        GameEnv { state }
    }

    // Start a fresh run. Returns the initial obs
    #[pyo3(signature = (seed=42))]
    fn reset(&mut self, seed: u64) -> PyGameState {
        let asc = self.state.ascension;
        self.state = create_game_state(asc, seed, self.state.fast_mode, self.state.neow);
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
    use super::ffi::PyAction;
    #[pymodule_export]
    use super::ffi::PyActionType;
    // Snapshot views
    #[pymodule_export]
    use super::ffi::PyCard;
    #[pymodule_export]
    use super::ffi::PyCharacter;
    #[pymodule_export]
    use super::ffi::PyEnergy;
    #[pymodule_export]
    use super::ffi::PyGameState;
    #[pymodule_export]
    use super::ffi::PyIntent;
    #[pymodule_export]
    use super::ffi::PyMap;
    #[pymodule_export]
    use super::ffi::PyModifier;
    #[pymodule_export]
    use super::ffi::PyMonster;
    #[pymodule_export]
    use super::ffi::PyPotion;
    #[pymodule_export]
    use super::ffi::PyRelic;
    #[pymodule_export]
    use super::ffi::PyRoom;
    #[pymodule_export]
    use super::ffi::PyTarget;
    // Unit enums
    #[pymodule_export]
    use super::ffi::PyCandidateFilter;
    #[pymodule_export]
    use super::ffi::PyCardColor;
    #[pymodule_export]
    use super::ffi::PyCardKind;
    #[pymodule_export]
    use super::ffi::PyCardName;
    #[pymodule_export]
    use super::ffi::PyCardRarity;
    #[pymodule_export]
    use super::ffi::PyDeltaSign;
    #[pymodule_export]
    use super::ffi::PyIntentKind;
    #[pymodule_export]
    use super::ffi::PyModifierKind;
    #[pymodule_export]
    use super::ffi::PyMonsterEncounter;
    #[pymodule_export]
    use super::ffi::PyMonsterName;
    #[pymodule_export]
    use super::ffi::PyPlayRestriction;
    #[pymodule_export]
    use super::ffi::PyPotionName;
    #[pymodule_export]
    use super::ffi::PyPotionRarity;
    #[pymodule_export]
    use super::ffi::PyRelicName;
    #[pymodule_export]
    use super::ffi::PyRelicTier;
    #[pymodule_export]
    use super::ffi::PyRoomKind;
    // Flat variant classes (Python-side union aliases live in python/slai/__init__.py)
    #[pymodule_export]
    use super::ffi::PyAmountAbsolute;
    #[pymodule_export]
    use super::ffi::PyAmountEventGoldAsk;
    #[pymodule_export]
    use super::ffi::PyAmountRange;
    #[pymodule_export]
    use super::ffi::PyAmountRelative;
    #[pymodule_export]
    use super::ffi::PyCandidatePoolCharacter;
    #[pymodule_export]
    use super::ffi::PyCandidatePoolDeck;
    #[pymodule_export]
    use super::ffi::PyCandidatePoolDiscover;
    #[pymodule_export]
    use super::ffi::PyCandidatePoolEventPickCard;
    #[pymodule_export]
    use super::ffi::PyCandidatePoolEventPickPotion;
    #[pymodule_export]
    use super::ffi::PyCandidatePoolHand;
    #[pymodule_export]
    use super::ffi::PyCandidatePoolMonsters;
    #[pymodule_export]
    use super::ffi::PyCandidatePoolPileDiscard;
    #[pymodule_export]
    use super::ffi::PyCandidatePoolPileDraw;
    #[pymodule_export]
    use super::ffi::PyCandidatePoolPileExhaust;
    #[pymodule_export]
    use super::ffi::PyCandidatePoolSource;
    #[pymodule_export]
    use super::ffi::PyCardCostKindFixed;
    #[pymodule_export]
    use super::ffi::PyCardCostKindGrowsOnDamageInstanceTaken;
    #[pymodule_export]
    use super::ffi::PyCardCostKindMinusDiscardsThisTurn;
    #[pymodule_export]
    use super::ffi::PyCardCostKindXCost;
    #[pymodule_export]
    use super::ffi::PyCardPile;
    #[pymodule_export]
    use super::ffi::PyChestKind;
    #[pymodule_export]
    use super::ffi::PyCostScope;
    #[pymodule_export]
    use super::ffi::PyEffectAdventurerSearch;
    #[pymodule_export]
    use super::ffi::PyEffectBlockGain;
    #[pymodule_export]
    use super::ffi::PyEffectBonfireOffer;
    #[pymodule_export]
    use super::ffi::PyEffectCardAdd;
    #[pymodule_export]
    use super::ffi::PyEffectCardAddRandom;
    #[pymodule_export]
    use super::ffi::PyEffectCardBottle;
    #[pymodule_export]
    use super::ffi::PyEffectCardDiscard;
    #[pymodule_export]
    use super::ffi::PyEffectCardDiscoverPick;
    #[pymodule_export]
    use super::ffi::PyEffectCardDiscoverRoll;
    #[pymodule_export]
    use super::ffi::PyEffectCardDraw;
    #[pymodule_export]
    use super::ffi::PyEffectCardDrawIfNoAttacks;
    #[pymodule_export]
    use super::ffi::PyEffectCardDrawUpTo;
    #[pymodule_export]
    use super::ffi::PyEffectCardDuplicate;
    #[pymodule_export]
    use super::ffi::PyEffectCardExhaust;
    #[pymodule_export]
    use super::ffi::PyEffectCardMove;
    #[pymodule_export]
    use super::ffi::PyEffectCardNightmarePick;
    #[pymodule_export]
    use super::ffi::PyEffectCardPlayFromDrawTop;
    #[pymodule_export]
    use super::ffi::PyEffectCardPurge;
    #[pymodule_export]
    use super::ffi::PyEffectCardRetain;
    #[pymodule_export]
    use super::ffi::PyEffectCardSetupPick;
    #[pymodule_export]
    use super::ffi::PyEffectCardTransform;
    #[pymodule_export]
    use super::ffi::PyEffectCardUpgrade;
    #[pymodule_export]
    use super::ffi::PyEffectCombatEnd;
    #[pymodule_export]
    use super::ffi::PyEffectCombatStart;
    #[pymodule_export]
    use super::ffi::PyEffectDamageFinisher;
    #[pymodule_export]
    use super::ffi::PyEffectDamageFlechettes;
    #[pymodule_export]
    use super::ffi::PyEffectDamageMindBlast;
    #[pymodule_export]
    use super::ffi::PyEffectDamagePhysical;
    #[pymodule_export]
    use super::ffi::PyEffectDamagePhysicalIfPoisoned;
    #[pymodule_export]
    use super::ffi::PyEffectDistractionAdd;
    #[pymodule_export]
    use super::ffi::PyEffectEnergyDelta;
    #[pymodule_export]
    use super::ffi::PyEffectEscapePlanCheck;
    #[pymodule_export]
    use super::ffi::PyEffectEventAdvanceState;
    #[pymodule_export]
    use super::ffi::PyEffectEventConsume;
    #[pymodule_export]
    use super::ffi::PyEffectFaceTrade;
    #[pymodule_export]
    use super::ffi::PyEffectGamble;
    #[pymodule_export]
    use super::ffi::PyEffectGlassKnifeDecay;
    #[pymodule_export]
    use super::ffi::PyEffectGoldDelta;
    #[pymodule_export]
    use super::ffi::PyEffectHandOfGreedProc;
    #[pymodule_export]
    use super::ffi::PyEffectHealthDelta;
    #[pymodule_export]
    use super::ffi::PyEffectHeelHookProc;
    #[pymodule_export]
    use super::ffi::PyEffectJoustBet;
    #[pymodule_export]
    use super::ffi::PyEffectKnowingSkullAsk;
    #[pymodule_export]
    use super::ffi::PyEffectMausoleumOpen;
    #[pymodule_export]
    use super::ffi::PyEffectMaxHealthDelta;
    #[pymodule_export]
    use super::ffi::PyEffectModifierGain;
    #[pymodule_export]
    use super::ffi::PyEffectModifierMultiply;
    #[pymodule_export]
    use super::ffi::PyEffectModifierRemove;
    #[pymodule_export]
    use super::ffi::PyEffectMonsterSpawn;
    #[pymodule_export]
    use super::ffi::PyEffectPotionAddRandom;
    #[pymodule_export]
    use super::ffi::PyEffectPotionDiscard;
    #[pymodule_export]
    use super::ffi::PyEffectRelicGrantRandom;
    #[pymodule_export]
    use super::ffi::PyEffectRelicGrantSpecific;
    #[pymodule_export]
    use super::ffi::PyEffectRelicLose;
    #[pymodule_export]
    use super::ffi::PyEffectRewardRollLibraryCards;
    #[pymodule_export]
    use super::ffi::PyEffectRewardRollNeowCards;
    #[pymodule_export]
    use super::ffi::PyEffectRewardRollPotions;
    #[pymodule_export]
    use super::ffi::PyEffectScrapOozeReach;
    #[pymodule_export]
    use super::ffi::PyEffectSetCostOverride;
    #[pymodule_export]
    use super::ffi::PyEffectShuffleDiscardPileIntoDrawPile;
    #[pymodule_export]
    use super::ffi::PyEffectSneakyStrikeProc;
    #[pymodule_export]
    use super::ffi::PyEffectStormOfSteelProc;
    #[pymodule_export]
    use super::ffi::PyEffectStrengthLoseTemp;
    #[pymodule_export]
    use super::ffi::PyEffectUnloadDiscard;
    #[pymodule_export]
    use super::ffi::PyEffectWheelSpin;
    #[pymodule_export]
    use super::ffi::PyEventKindAddict;
    #[pymodule_export]
    use super::ffi::PyEventKindBackToBasics;
    #[pymodule_export]
    use super::ffi::PyEventKindBeggar;
    #[pymodule_export]
    use super::ffi::PyEventKindBigFish;
    #[pymodule_export]
    use super::ffi::PyEventKindBonfireSpirits;
    #[pymodule_export]
    use super::ffi::PyEventKindColosseum;
    #[pymodule_export]
    use super::ffi::PyEventKindDeadAdventurer;
    #[pymodule_export]
    use super::ffi::PyEventKindDesigner;
    #[pymodule_export]
    use super::ffi::PyEventKindDuplicator;
    #[pymodule_export]
    use super::ffi::PyEventKindFaceTrader;
    #[pymodule_export]
    use super::ffi::PyEventKindGhosts;
    #[pymodule_export]
    use super::ffi::PyEventKindGoldenIdol;
    #[pymodule_export]
    use super::ffi::PyEventKindGoldenShrine;
    #[pymodule_export]
    use super::ffi::PyEventKindKnowingSkull;
    #[pymodule_export]
    use super::ffi::PyEventKindLivingWall;
    #[pymodule_export]
    use super::ffi::PyEventKindMaskedBandits;
    #[pymodule_export]
    use super::ffi::PyEventKindMushrooms;
    #[pymodule_export]
    use super::ffi::PyEventKindNeow;
    #[pymodule_export]
    use super::ffi::PyEventKindOminousForge;
    #[pymodule_export]
    use super::ffi::PyEventKindPurifier;
    #[pymodule_export]
    use super::ffi::PyEventKindScrapOoze;
    #[pymodule_export]
    use super::ffi::PyEventKindShiningLight;
    #[pymodule_export]
    use super::ffi::PyEventKindTheCleric;
    #[pymodule_export]
    use super::ffi::PyEventKindTheDivineFountain;
    #[pymodule_export]
    use super::ffi::PyEventKindTheJoust;
    #[pymodule_export]
    use super::ffi::PyEventKindTheLab;
    #[pymodule_export]
    use super::ffi::PyEventKindTheLibrary;
    #[pymodule_export]
    use super::ffi::PyEventKindTheMausoleum;
    #[pymodule_export]
    use super::ffi::PyEventKindTheSsssserpent;
    #[pymodule_export]
    use super::ffi::PyEventKindTheWomanInBlue;
    #[pymodule_export]
    use super::ffi::PyEventKindTransmogrifier;
    #[pymodule_export]
    use super::ffi::PyEventKindUpgradeShrine;
    #[pymodule_export]
    use super::ffi::PyEventKindVampires;
    #[pymodule_export]
    use super::ffi::PyEventKindWeMeetAgain;
    #[pymodule_export]
    use super::ffi::PyEventKindWheelOfChange;
    #[pymodule_export]
    use super::ffi::PyEventKindWingStatue;
    #[pymodule_export]
    use super::ffi::PyEventKindWorldOfGoop;
    #[pymodule_export]
    use super::ffi::PyFrameChest;
    #[pymodule_export]
    use super::ffi::PyFrameCombat;
    #[pymodule_export]
    use super::ffi::PyFrameEvent;
    #[pymodule_export]
    use super::ffi::PyFrameMap;
    #[pymodule_export]
    use super::ffi::PyFrameRestSite;
    #[pymodule_export]
    use super::ffi::PyFrameReward;
    #[pymodule_export]
    use super::ffi::PyFrameShop;
    #[pymodule_export]
    use super::ffi::PyKnowingSkullWish;
    #[pymodule_export]
    use super::ffi::PySelectionKindAll;
    #[pymodule_export]
    use super::ffi::PySelectionKindInput;
    #[pymodule_export]
    use super::ffi::PySelectionKindInputUpTo;
    #[pymodule_export]
    use super::ffi::PySelectionKindRandom;
    #[pymodule_export]
    use super::ffi::PySelectionKindSingle;
}
