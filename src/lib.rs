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
    const MAX_CARD_REWARD_ROLL: usize = consts::MAX_CARD_REWARD_ROLL;
    #[classattr]
    const CAULDRON_POTION_COUNT: usize = consts::CAULDRON_POTION_COUNT;
    #[classattr]
    const CARDS_DRAWN_PER_TURN: u16 = consts::CARDS_DRAWN_PER_TURN;
    #[classattr]
    const NIGHTMARE_COPIES: u8 = consts::NIGHTMARE_COPIES;
    #[classattr]
    const MAX_BLOCK: u16 = consts::MAX_BLOCK;
    #[classattr]
    const MAP_HEIGHT: usize = consts::MAP_HEIGHT;
    #[classattr]
    const ACT_FINAL: u8 = consts::ACT_FINAL;
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
    use super::ffi::PyPotionTemplate;
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
    use super::ffi::PyCandidatePoolEventRollCard;
    #[pymodule_export]
    use super::ffi::PyCandidatePoolEventRollPotion;
    #[pymodule_export]
    use super::ffi::PyCandidatePoolEventRollRelic;
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
    use super::ffi::PyChest;
    #[pymodule_export]
    use super::ffi::PyChestKind;
    #[pymodule_export]
    use super::ffi::PyCombat;
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
    use super::ffi::PyEffectKnowingSkullCostBump;
    #[pymodule_export]
    use super::ffi::PyEffectMausoleumOpen;
    #[pymodule_export]
    use super::ffi::PyEffectMaxHealthDelta;
    #[pymodule_export]
    use super::ffi::PyEffectModifierGain;
    #[pymodule_export]
    use super::ffi::PyEffectModifierMultiply;
    #[pymodule_export]
    use super::ffi::PyEffectMonsterSpawn;
    #[pymodule_export]
    use super::ffi::PyEffectPotionAddRandom;
    #[pymodule_export]
    use super::ffi::PyEffectPotionDiscard;
    #[pymodule_export]
    use super::ffi::PyEffectRelicGrantPool;
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
    use super::ffi::PyEffectRitualDaggerProc;
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
    use super::ffi::PyEvent;
    #[pymodule_export]
    use super::ffi::PyRestSite;
    #[pymodule_export]
    use super::ffi::PyReward;
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
    #[pymodule_export]
    use super::ffi::PyShop;
    // Content catalog: template classes + state-free enumeration functions
    #[pymodule_export]
    use super::ffi::PyCardTemplate;
    #[pymodule_export]
    use super::ffi::PyEventName;
    #[pymodule_export]
    use super::ffi::PyEventOptionTemplate;
    #[pymodule_export]
    use super::ffi::PyMonsterKind;
    #[pymodule_export]
    use super::ffi::PyMonsterTemplate;
    #[pymodule_export]
    use super::ffi::PyRelicTemplate;
    #[pymodule_export]
    use super::ffi::template::get_card_templates;
    #[pymodule_export]
    use super::ffi::template::get_event_option_templates;
    #[pymodule_export]
    use super::ffi::template::get_monster_templates;
    #[pymodule_export]
    use super::ffi::template::get_potion_templates;
    #[pymodule_export]
    use super::ffi::template::get_relic_templates;

    // Constants surface: load-bearing tunables + derived shop ceilings
    #[pymodule_export]
    const STARTING_GOLD: u16 = super::consts::STARTING_GOLD;
    #[pymodule_export]
    const MAX_GOLD: u16 = super::consts::MAX_GOLD;
    #[pymodule_export]
    const GOLD_MONSTER_MIN: u16 = super::consts::GOLD_MONSTER_MIN;
    #[pymodule_export]
    const GOLD_MONSTER_MAX: u16 = super::consts::GOLD_MONSTER_MAX;
    #[pymodule_export]
    const GOLD_ELITE_MIN: u16 = super::consts::GOLD_ELITE_MIN;
    #[pymodule_export]
    const GOLD_ELITE_MAX: u16 = super::consts::GOLD_ELITE_MAX;
    #[pymodule_export]
    const GOLD_BOSS_MIN: u16 = super::consts::GOLD_BOSS_MIN;
    #[pymodule_export]
    const GOLD_BOSS_MAX: u16 = super::consts::GOLD_BOSS_MAX;
    #[pymodule_export]
    const CHEST_SMALL_GOLD_CHANCE: u8 = super::consts::CHEST_SMALL_GOLD_CHANCE;
    #[pymodule_export]
    const CHEST_SMALL_GOLD_BASE: u16 = super::consts::CHEST_SMALL_GOLD_BASE;
    #[pymodule_export]
    const CHEST_MEDIUM_GOLD_CHANCE: u8 = super::consts::CHEST_MEDIUM_GOLD_CHANCE;
    #[pymodule_export]
    const CHEST_MEDIUM_GOLD_BASE: u16 = super::consts::CHEST_MEDIUM_GOLD_BASE;
    #[pymodule_export]
    const CHEST_LARGE_GOLD_CHANCE: u8 = super::consts::CHEST_LARGE_GOLD_CHANCE;
    #[pymodule_export]
    const CHEST_LARGE_GOLD_BASE: u16 = super::consts::CHEST_LARGE_GOLD_BASE;
    #[pymodule_export]
    const CHEST_GOLD_VARIANCE_MIN: f32 = super::consts::CHEST_GOLD_VARIANCE_MIN;
    #[pymodule_export]
    const CHEST_GOLD_VARIANCE_MAX: f32 = super::consts::CHEST_GOLD_VARIANCE_MAX;
    #[pymodule_export]
    const SHOP_PRICE_CARD_COMMON: u16 = super::consts::SHOP_PRICE_CARD_COMMON;
    #[pymodule_export]
    const SHOP_PRICE_CARD_UNCOMMON: u16 = super::consts::SHOP_PRICE_CARD_UNCOMMON;
    #[pymodule_export]
    const SHOP_PRICE_CARD_RARE: u16 = super::consts::SHOP_PRICE_CARD_RARE;
    #[pymodule_export]
    const SHOP_PRICE_COLORLESS_NUMER: u16 = super::consts::SHOP_PRICE_COLORLESS_NUMER;
    #[pymodule_export]
    const SHOP_PRICE_COLORLESS_DENOM: u16 = super::consts::SHOP_PRICE_COLORLESS_DENOM;
    #[pymodule_export]
    const SHOP_PRICE_CARD_VARIANCE_MIN: f32 = super::consts::SHOP_PRICE_CARD_VARIANCE_MIN;
    #[pymodule_export]
    const SHOP_PRICE_CARD_VARIANCE_MAX: f32 = super::consts::SHOP_PRICE_CARD_VARIANCE_MAX;
    #[pymodule_export]
    const SHOP_PRICE_POTION_COMMON: u16 = super::consts::SHOP_PRICE_POTION_COMMON;
    #[pymodule_export]
    const SHOP_PRICE_POTION_UNCOMMON: u16 = super::consts::SHOP_PRICE_POTION_UNCOMMON;
    #[pymodule_export]
    const SHOP_PRICE_POTION_RARE: u16 = super::consts::SHOP_PRICE_POTION_RARE;
    #[pymodule_export]
    const SHOP_PRICE_RELIC_COMMON: u16 = super::consts::SHOP_PRICE_RELIC_COMMON;
    #[pymodule_export]
    const SHOP_PRICE_RELIC_UNCOMMON: u16 = super::consts::SHOP_PRICE_RELIC_UNCOMMON;
    #[pymodule_export]
    const SHOP_PRICE_RELIC_RARE: u16 = super::consts::SHOP_PRICE_RELIC_RARE;
    #[pymodule_export]
    const SHOP_PRICE_RELIC_SHOP: u16 = super::consts::SHOP_PRICE_RELIC_SHOP;
    #[pymodule_export]
    const SHOP_PRICE_RELIC_POTION_VARIANCE_MIN: f32 =
        super::consts::SHOP_PRICE_RELIC_POTION_VARIANCE_MIN;
    #[pymodule_export]
    const SHOP_PRICE_RELIC_POTION_VARIANCE_MAX: f32 =
        super::consts::SHOP_PRICE_RELIC_POTION_VARIANCE_MAX;
    #[pymodule_export]
    const SHOP_SALE_DIVISOR: u16 = super::consts::SHOP_SALE_DIVISOR;
    #[pymodule_export]
    const SHOP_PURGE_COST_BASE: u16 = super::consts::SHOP_PURGE_COST_BASE;
    #[pymodule_export]
    const SHOP_PURGE_COST_INCREMENT: u16 = super::consts::SHOP_PURGE_COST_INCREMENT;
    #[pymodule_export]
    const ASCENSION_SHOP_PRICE_BUMP_LEVEL: u8 = super::consts::ASCENSION_SHOP_PRICE_BUMP_LEVEL;
    #[pymodule_export]
    const ASCENSION_SHOP_PRICE_BUMP_NUMER: u16 = super::consts::ASCENSION_SHOP_PRICE_BUMP_NUMER;
    #[pymodule_export]
    const ASCENSION_SHOP_PRICE_BUMP_DENOM: u16 = super::consts::ASCENSION_SHOP_PRICE_BUMP_DENOM;
    #[pymodule_export]
    const SHOP_PRICE_CARD_MAX: u16 = super::consts::SHOP_PRICE_CARD_MAX;
    #[pymodule_export]
    const SHOP_PRICE_RELIC_MAX: u16 = super::consts::SHOP_PRICE_RELIC_MAX;
    #[pymodule_export]
    const SHOP_PRICE_POTION_MAX: u16 = super::consts::SHOP_PRICE_POTION_MAX;
    #[pymodule_export]
    const WE_MEET_AGAIN_GOLD_ASK_MIN: u16 = super::consts::WE_MEET_AGAIN_GOLD_ASK_MIN;
    #[pymodule_export]
    const WE_MEET_AGAIN_GOLD_ASK_MAX: u16 = super::consts::WE_MEET_AGAIN_GOLD_ASK_MAX;
    #[pymodule_export]
    const NEOW_GOLD_SMALL: u16 = super::consts::NEOW_GOLD_SMALL;
    #[pymodule_export]
    const NEOW_GOLD_LARGE: u16 = super::consts::NEOW_GOLD_LARGE;
    #[pymodule_export]
    const NEOW_CARD_COUNT: usize = super::consts::NEOW_CARD_COUNT;
    #[pymodule_export]
    const NEOW_POTION_COUNT: u8 = super::consts::NEOW_POTION_COUNT;
    #[pymodule_export]
    const SILENT_HP_MAX_BASE: u16 = super::consts::SILENT_HP_MAX_BASE;
    #[pymodule_export]
    const SILENT_HP_MAX_A14_DELTA: u16 = super::consts::SILENT_HP_MAX_A14_DELTA;
    #[pymodule_export]
    const HP_START_FRACTION_A6: f32 = super::consts::HP_START_FRACTION_A6;
}
