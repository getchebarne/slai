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

#[pyclass(module = "slai._slai")]
struct GameEnv {
    state: game::GameState,
}

#[pymethods]
impl GameEnv {
    // Game-shape constants — mirror of `crate::consts` for encoders/wrappers
    #[classattr]
    const ENERGY_MAX_BASE: u16 = consts::ENERGY_MAX_BASE;
    #[classattr]
    const MAX_MOVE_HISTORY: usize = consts::MAX_MOVE_HISTORY;
    #[classattr]
    const NUM_ENCOUNTERS_EASY_ACT1: usize = consts::NUM_ENCOUNTERS_EASY_ACT1;
    #[classattr]
    const TIME_WARP_PLAYS: i16 = consts::TIME_WARP_PLAYS;
    #[classattr]
    const PANACHE_PLAYS: u8 = consts::PANACHE_PLAYS;
    #[classattr]
    const MAX_NORMALITY_PLAYS: u16 = consts::NORMALITY_PLAYS_MAX;
    #[classattr]
    const MAX_VELVET_CHOKER_PLAYS: i16 = consts::VELVET_CHOKER_PLAYS_MAX;
    #[classattr]
    const MAX_POCKETWATCH_PLAYS: i16 = consts::POCKETWATCH_PLAYS_MAX;
    #[classattr]
    const ORICHALCUM_BLOCK: u16 = consts::ORICHALCUM_BLOCK;
    #[classattr]
    const MIN_TORII_DAMAGE: u16 = consts::TORII_DAMAGE_MIN;
    #[classattr]
    const MAX_TORII_DAMAGE: u16 = consts::TORII_DAMAGE_MAX;
    #[classattr]
    const BURN_DAMAGE: u16 = consts::BURN_DAMAGE;
    #[classattr]
    const BURN_DAMAGE_UPGRADED: u16 = consts::BURN_DAMAGE_UPGRADED;
    #[classattr]
    const DECAY_DAMAGE: u16 = consts::DECAY_DAMAGE;
    #[classattr]
    const EXPLOSIVE_DAMAGE: u16 = consts::EXPLOSIVE_DAMAGE;
    #[classattr]
    const MAX_MONSTERS: usize = consts::MAX_MONSTERS;
    #[classattr]
    const MAX_SIZE_CARD_PILE_HAND: usize = consts::MAX_SIZE_CARD_PILE_HAND;
    #[classattr]
    const MAX_COMBAT_CARD_REWARD: usize = consts::MAX_COMBAT_CARD_REWARD;
    #[classattr]
    const MAX_REWARD_CARD_BUNDLES: usize = consts::MAX_REWARD_CARD_BUNDLES;
    #[classattr]
    const MAX_CARD_REWARD_ROLL: usize = consts::MAX_CARD_REWARD_ROLL;
    #[classattr]
    const NUM_CAULDRON_POTIONS: usize = consts::CAULDRON_POTION_COUNT;
    #[classattr]
    const MAX_SIZE_CARD_PILE_EVENT_ROLL: usize = consts::MAX_SIZE_CARD_PILE_EVENT_ROLL;
    #[classattr]
    const MAX_SIZE_RELIC_EVENT_ROLL: usize = consts::MAX_SIZE_RELIC_EVENT_ROLL;
    #[classattr]
    const MAX_SIZE_POTION_EVENT_ROLL: usize = consts::MAX_SIZE_POTION_EVENT_ROLL;
    #[classattr]
    const MAX_POTION_SLOTS: usize = consts::POTION_SLOTS_MAX;
    #[classattr]
    const NUM_BOSS_RELIC_REWARDS: usize = consts::BOSS_RELIC_REWARD_COUNT;
    #[classattr]
    const SHOP_SLOTS_CARD_TOTAL: usize = consts::SHOP_SLOTS_CARD_TOTAL;
    #[classattr]
    const SHOP_SLOTS_RELIC: usize = consts::SHOP_SLOTS_RELIC;
    #[classattr]
    const SHOP_SLOTS_POTION: usize = consts::SHOP_SLOTS_POTION;
    #[classattr]
    const NUM_DISCOVER_ROLLS: u8 = consts::DISCOVER_ROLL_COUNT;
    #[classattr]
    const BOMB_FUSE_TURNS: u8 = consts::BOMB_FUSE_TURNS;
    #[classattr]
    const FACTOR_VULN: f32 = consts::FACTOR_VULN;
    #[classattr]
    const HEXAGHOST_DIVIDER_HITS: u8 = consts::HEXAGHOST_DIVIDER_HITS;
    #[classattr]
    const HEXAGHOST_DIVIDER_DIVISOR: u16 = consts::HEXAGHOST_DIVIDER_DIVISOR;
    #[classattr]
    const MAP_HEIGHT: usize = consts::MAP_HEIGHT;
    #[classattr]
    const ACT_FINAL: u8 = consts::ACT_FINAL;
    #[classattr]
    const MAX_ASCENSION: u8 = consts::ASCENSION_MAX;
    #[classattr]
    const MAP_WIDTH: usize = consts::MAP_WIDTH;
    #[classattr]
    const MAP_ROW_TREASURE: usize = consts::MAP_ROW_TREASURE;
    #[classattr]
    const CARD_REWARD_ROLL_OFFSET_BASE: i8 = consts::CARD_REWARD_ROLL_OFFSET_BASE;
    #[classattr]
    const MIN_CARD_REWARD_ROLL_OFFSET: i8 = consts::CARD_REWARD_ROLL_OFFSET_MIN;
    #[classattr]
    const POTION_DROP_CHANCE_BASE: i32 = consts::POTION_DROP_CHANCE_BASE;
    #[classattr]
    const MATCH_AND_KEEP_ATTEMPTS: u8 = consts::MATCH_AND_KEEP_ATTEMPTS;
    #[classattr]
    const NUM_MATCH_AND_KEEP_CARDS: usize = consts::MATCH_AND_KEEP_CARD_COUNT;

    #[new]
    #[pyo3(signature = (ascension=0, fast_mode=false, neow=false))]
    fn new(ascension: u8, fast_mode: bool, neow: bool) -> Self {
        // Placeholder seed; consumers must call `reset(seed=...)` before stepping (gymnasium convention)
        let state = create_game_state(ascension, 0, fast_mode, neow);
        GameEnv { state }
    }

    // Start a fresh run. Returns the initial obs
    #[pyo3(signature = (seed=42))]
    fn reset(&mut self, py: Python<'_>, seed: u64) -> PyResult<PyGameState> {
        let asc = self.state.ascension;
        self.state = create_game_state(asc, seed, self.state.fast_mode, self.state.neow);
        snapshot_state(py, &self.state)
    }

    // Apply an action. Returns `(obs, terminated)`; a won run terminates only after Act 3's last boss fight without all three keys, or after the Heart with them
    fn step(&mut self, py: Python<'_>, action: PyAction) -> PyResult<(PyGameState, bool)> {
        let internal =
            to_internal_action(action).map_err(pyo3::exceptions::PyValueError::new_err)?;
        step(&mut self.state, internal).map_err(pyo3::exceptions::PyValueError::new_err)?;
        let obs = snapshot_state(py, &self.state)?;
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
mod _slai {
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
    use super::ffi::PyCardPlay;
    #[pymodule_export]
    use super::ffi::PyCharacter;
    #[pymodule_export]
    use super::ffi::PyEnergy;
    #[pymodule_export]
    use super::ffi::PyEventOption;
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
    use super::ffi::PyPlaySource;
    #[pymodule_export]
    use super::ffi::PyPotionName;
    #[pymodule_export]
    use super::ffi::PyPotionRarity;
    #[pymodule_export]
    use super::ffi::PyPotionTemplate;
    #[pymodule_export]
    use super::ffi::PyReadAt;
    #[pymodule_export]
    use super::ffi::PyRelicExclusion;
    #[pymodule_export]
    use super::ffi::PyRelicName;
    #[pymodule_export]
    use super::ffi::PyRelicTier;
    #[pymodule_export]
    use super::ffi::PyRoomKind;
    #[pymodule_export]
    use super::ffi::PyRounding;
    // Flat variant classes (Python-side union aliases live in python/slai/__init__.py)
    #[pymodule_export]
    use super::ffi::PyAmountAbsolute;
    #[pymodule_export]
    use super::ffi::PyAmountRange;
    #[pymodule_export]
    use super::ffi::PyAmountRelative;
    #[pymodule_export]
    use super::ffi::PyCandidatePool;
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
    use super::ffi::PyEffectBombArm;
    #[pymodule_export]
    use super::ffi::PyEffectBonfireOffer;
    #[pymodule_export]
    use super::ffi::PyEffectCardAdd;
    #[pymodule_export]
    use super::ffi::PyEffectCardAddRandom;
    #[pymodule_export]
    use super::ffi::PyEffectCardAdopt;
    #[pymodule_export]
    use super::ffi::PyEffectCardBottle;
    #[pymodule_export]
    use super::ffi::PyEffectCardDiscard;
    #[pymodule_export]
    use super::ffi::PyEffectCardDiscoverRoll;
    #[pymodule_export]
    use super::ffi::PyEffectCardDiscoverSelect;
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
    use super::ffi::PyEffectCardNightmareSelect;
    #[pymodule_export]
    use super::ffi::PyEffectCardPileEventRollClear;
    #[pymodule_export]
    use super::ffi::PyEffectCardPlayFromCardPileDrawTop;
    #[pymodule_export]
    use super::ffi::PyEffectCardPurge;
    #[pymodule_export]
    use super::ffi::PyEffectCardRetain;
    #[pymodule_export]
    use super::ffi::PyEffectCardSetupSelect;
    #[pymodule_export]
    use super::ffi::PyEffectCardStasisSteal;
    #[pymodule_export]
    use super::ffi::PyEffectCardTransform;
    #[pymodule_export]
    use super::ffi::PyEffectCardUpgrade;
    #[pymodule_export]
    use super::ffi::PyEffectCombatEnd;
    #[pymodule_export]
    use super::ffi::PyEffectCombatStart;
    #[pymodule_export]
    use super::ffi::PyEffectDamageDeal;
    #[pymodule_export]
    use super::ffi::PyEffectDamageFinisher;
    #[pymodule_export]
    use super::ffi::PyEffectDamageFlechettes;
    #[pymodule_export]
    use super::ffi::PyEffectDamageMindBlast;
    #[pymodule_export]
    use super::ffi::PyEffectDamagePhysical;
    #[pymodule_export]
    use super::ffi::PyEffectDebuffsClear;
    #[pymodule_export]
    use super::ffi::PyEffectDistractionAdd;
    #[pymodule_export]
    use super::ffi::PyEffectEnergyDelta;
    #[pymodule_export]
    use super::ffi::PyEffectEscapePlanCheck;
    #[pymodule_export]
    use super::ffi::PyEffectEventAdvanceStage;
    #[pymodule_export]
    use super::ffi::PyEffectEventClose;
    #[pymodule_export]
    use super::ffi::PyEffectEventConsume;
    #[pymodule_export]
    use super::ffi::PyEffectGamble;
    #[pymodule_export]
    use super::ffi::PyEffectGlassKnifeDecay;
    #[pymodule_export]
    use super::ffi::PyEffectGoldDelta;
    #[pymodule_export]
    use super::ffi::PyEffectHalfDeadRebirth;
    #[pymodule_export]
    use super::ffi::PyEffectHandOfGreedProc;
    #[pymodule_export]
    use super::ffi::PyEffectHealthDelta;
    #[pymodule_export]
    use super::ffi::PyEffectHealthSet;
    #[pymodule_export]
    use super::ffi::PyEffectHeelHookProc;
    #[pymodule_export]
    use super::ffi::PyEffectHexaghostBurnIncrease;
    #[pymodule_export]
    use super::ffi::PyEffectJoustBet;
    #[pymodule_export]
    use super::ffi::PyEffectKnowingSkullCostBump;
    #[pymodule_export]
    use super::ffi::PyEffectLifestealHeal;
    #[pymodule_export]
    use super::ffi::PyEffectMatchFlipSeen;
    #[pymodule_export]
    use super::ffi::PyEffectMatchFlipUnseen;
    #[pymodule_export]
    use super::ffi::PyEffectMausoleumOpen;
    #[pymodule_export]
    use super::ffi::PyEffectMaxHealthDelta;
    #[pymodule_export]
    use super::ffi::PyEffectModifierDelta;
    #[pymodule_export]
    use super::ffi::PyEffectModifierMultiply;
    #[pymodule_export]
    use super::ffi::PyEffectModifierRemove;
    #[pymodule_export]
    use super::ffi::PyEffectModifierStrengthNegativeRemove;
    #[pymodule_export]
    use super::ffi::PyEffectMonsterEscape;
    #[pymodule_export]
    use super::ffi::PyEffectMonsterRemove;
    #[pymodule_export]
    use super::ffi::PyEffectMonsterSpawn;
    #[pymodule_export]
    use super::ffi::PyEffectMonsterSplit;
    #[pymodule_export]
    use super::ffi::PyEffectMonsterSummon;
    #[pymodule_export]
    use super::ffi::PyEffectNeowCardsRoll;
    #[pymodule_export]
    use super::ffi::PyEffectPhilosopherStoneProc;
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
    use super::ffi::PyEffectRestSmith;
    #[pymodule_export]
    use super::ffi::PyEffectRestToke;
    #[pymodule_export]
    use super::ffi::PyEffectRewardRollCards;
    #[pymodule_export]
    use super::ffi::PyEffectRewardRollGold;
    #[pymodule_export]
    use super::ffi::PyEffectRewardRollPotions;
    #[pymodule_export]
    use super::ffi::PyEffectRewardRollRelic;
    #[pymodule_export]
    use super::ffi::PyEffectRitualDaggerProc;
    #[pymodule_export]
    use super::ffi::PyEffectRoomEnter;
    #[pymodule_export]
    use super::ffi::PyEffectScrapOozeReach;
    #[pymodule_export]
    use super::ffi::PyEffectSetCostOverride;
    #[pymodule_export]
    use super::ffi::PyEffectShuffleCardPileDiscardIntoCardPileDraw;
    #[pymodule_export]
    use super::ffi::PyEffectSkewerProc;
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
    use super::ffi::PyLocationBossRoom;
    #[pymodule_export]
    use super::ffi::PyLocationBossTreasure;
    #[pymodule_export]
    use super::ffi::PyLocationOverworld;
    #[pymodule_export]
    use super::ffi::PyLocationStart;
    #[pymodule_export]
    use super::ffi::PyLocationVictory;
    #[pymodule_export]
    use super::ffi::PyRestSite;
    #[pymodule_export]
    use super::ffi::PyReward;
    #[pymodule_export]
    use super::ffi::PyRewardRollTrigger;
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
    use super::ffi::PyModifierSpawnTemplate;
    #[pymodule_export]
    use super::ffi::PyMonsterKind;
    #[pymodule_export]
    use super::ffi::PyMonsterMoveTemplate;
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
    const MAX_SHOP_PRICE_CARD: u16 = super::consts::SHOP_PRICE_CARD_MAX;
    #[pymodule_export]
    const MAX_SHOP_PRICE_RELIC: u16 = super::consts::SHOP_PRICE_RELIC_MAX;
    #[pymodule_export]
    const MAX_SHOP_PRICE_POTION: u16 = super::consts::SHOP_PRICE_POTION_MAX;
}
