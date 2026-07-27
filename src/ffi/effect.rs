use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::type_hint_union;
use pyo3::type_object::PyTypeInfo;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;

use super::amount::PyAmount;
use super::amount::PyDeltaSign;
use super::card::PyCardColor;
use super::card::PyCardKind;
use super::card::PyCardPile;
use super::card::PyCostScope;
use super::macros::variant_union;
use super::modifier::PyModifierKind;
use super::monster::PyMonsterName;
use super::relic::PyRelicName;
use super::target::PyTarget;

// Mirrors only EffectKind variants reachable from static card/monster defs; snapshot_effect panics on runtime-only variants
#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectDamagePhysical",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectDamagePhysical {
    pub amount: u16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectDamagePhysicalIfPoisoned",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectDamagePhysicalIfPoisoned {
    pub amount: u16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectHeelHookProc",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectHeelHookProc {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectEscapePlanCheck",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectEscapePlanCheck {
    pub block: u16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectGlassKnifeDecay",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectGlassKnifeDecay {
    pub delta: i16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardSetupPick",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardSetupPick {
    pub free: bool,
    pub bottom: bool,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardNightmarePick",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardNightmarePick {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectDistractionAdd",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectDistractionAdd {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectSetCostOverride",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectSetCostOverride {
    pub amount: u8,
    pub only_reduce: bool,
    pub scope: PyCostScope,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectDamageFinisher",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectDamageFinisher {
    pub damage: u16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectDamageFlechettes",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectDamageFlechettes {
    pub damage: u16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectUnloadDiscard",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectUnloadDiscard {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectStormOfSteelProc",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectStormOfSteelProc {
    pub upgraded: bool,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectSneakyStrikeProc",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectSneakyStrikeProc {
    pub energy: u8,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectBlockGain",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectBlockGain {
    pub amount: u16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectModifierGain",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectModifierGain {
    pub kind: PyModifierKind,
    pub stacks: i16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectModifierMultiply",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectModifierMultiply {
    pub kind: PyModifierKind,
    pub factor: u8,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectModifierRemove",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectModifierRemove {
    pub kind: PyModifierKind,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectEnergyDelta",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectEnergyDelta {
    pub sign: PyDeltaSign,
    pub amount: u16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardAdd",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardAdd {
    pub card_name: String,
    pub pile: PyCardPile,
    pub count: u16,
    pub upgraded: bool,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardDraw",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardDraw {
    pub count: u16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardDrawUpTo",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardDrawUpTo {
    pub amount: u8,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardDiscard",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardDiscard {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardRetain",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardRetain {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectDamageMindBlast",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectDamageMindBlast {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectShuffleDiscardPileIntoDrawPile",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectShuffleDiscardPileIntoDrawPile {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectMaxHealthDelta",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectMaxHealthDelta {
    pub sign: PyDeltaSign,
    pub amount: PyAmount,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectHealthDelta",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectHealthDelta {
    pub sign: PyDeltaSign,
    pub amount: PyAmount,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectPotionAddRandom",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectPotionAddRandom {
    pub limited: bool,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectPotionDiscard",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectPotionDiscard {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectRewardRollPotions",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectRewardRollPotions {
    pub count: u8,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardDiscoverRoll",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardDiscoverRoll {
    pub kind: Option<PyCardKind>,
    pub color: PyCardColor,
    pub count: u8,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectGoldDelta",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectGoldDelta {
    pub sign: PyDeltaSign,
    pub amount: PyAmount,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectRelicGrantRandom",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectRelicGrantRandom {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectWheelSpin",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectWheelSpin {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectBonfireOffer",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectBonfireOffer {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectFaceTrade",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectFaceTrade {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectMonsterSpawn",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectMonsterSpawn {
    pub name: PyMonsterName,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCombatStart",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCombatStart {
    pub event_gold: Option<PyAmount>,
    pub event_relic: Option<PyRelicName>,
    pub event_relic_roll: bool,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectAdventurerSearch",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectAdventurerSearch {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectRelicGrantSpecific",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectRelicGrantSpecific {
    pub name: PyRelicName,
    pub fallback_circlet: bool,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectEventAdvanceState",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectEventAdvanceState {
    pub delta: i8,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectScrapOozeReach",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectScrapOozeReach {
    pub dmg: u16,
    pub chance: u8,
    pub advance_on_miss: bool,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectEventConsume",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectEventConsume {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardDiscoverPick",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardDiscoverPick {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardPurge",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardPurge {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardUpgrade",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardUpgrade {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardDuplicate",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardDuplicate {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardTransform",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardTransform {
    pub target: Option<PyTarget>,
}

// NB: variant order matches the old complex enum — card_identity_hash depends on it
#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardAddRandom",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardAddRandom {
    pub color: PyCardColor,
    pub kind: Option<PyCardKind>,
    pub pile: PyCardPile,
    pub count: u8,
    pub cost_zero: Option<PyCostScope>,
    pub upgraded: bool,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardDrawIfNoAttacks",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardDrawIfNoAttacks {
    pub count: u16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectHandOfGreedProc",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectHandOfGreedProc {
    pub gold: u16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardExhaust",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardExhaust {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardMove",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardMove {
    pub pile: PyCardPile,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardPlayFromDrawTop",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardPlayFromDrawTop {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardCostRandomize",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardCostRandomize {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectGamble",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectGamble {
    pub choose_discards: bool,
    pub discards_before: Option<u8>,
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectLiquidMemoriesPick",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectLiquidMemoriesPick {
    pub target: Option<PyTarget>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCombatEscape",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCombatEscape {
    pub target: Option<PyTarget>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyEffect {
    DamagePhysical(PyEffectDamagePhysical),
    DamagePhysicalIfPoisoned(PyEffectDamagePhysicalIfPoisoned),
    HeelHookProc(PyEffectHeelHookProc),
    EscapePlanCheck(PyEffectEscapePlanCheck),
    GlassKnifeDecay(PyEffectGlassKnifeDecay),
    CardSetupPick(PyEffectCardSetupPick),
    CardNightmarePick(PyEffectCardNightmarePick),
    DistractionAdd(PyEffectDistractionAdd),
    SetCostOverride(PyEffectSetCostOverride),
    DamageFinisher(PyEffectDamageFinisher),
    DamageFlechettes(PyEffectDamageFlechettes),
    UnloadDiscard(PyEffectUnloadDiscard),
    StormOfSteelProc(PyEffectStormOfSteelProc),
    SneakyStrikeProc(PyEffectSneakyStrikeProc),
    BlockGain(PyEffectBlockGain),
    ModifierGain(PyEffectModifierGain),
    ModifierMultiply(PyEffectModifierMultiply),
    ModifierRemove(PyEffectModifierRemove),
    EnergyDelta(PyEffectEnergyDelta),
    CardAdd(PyEffectCardAdd),
    CardDraw(PyEffectCardDraw),
    CardDrawUpTo(PyEffectCardDrawUpTo),
    CardDiscard(PyEffectCardDiscard),
    CardRetain(PyEffectCardRetain),
    DamageMindBlast(PyEffectDamageMindBlast),
    ShuffleDiscardPileIntoDrawPile(PyEffectShuffleDiscardPileIntoDrawPile),
    MaxHealthDelta(PyEffectMaxHealthDelta),
    HealthDelta(PyEffectHealthDelta),
    PotionAddRandom(PyEffectPotionAddRandom),
    PotionDiscard(PyEffectPotionDiscard),
    RewardRollPotions(PyEffectRewardRollPotions),
    CardDiscoverRoll(PyEffectCardDiscoverRoll),
    GoldDelta(PyEffectGoldDelta),
    RelicGrantRandom(PyEffectRelicGrantRandom),
    WheelSpin(PyEffectWheelSpin),
    BonfireOffer(PyEffectBonfireOffer),
    FaceTrade(PyEffectFaceTrade),
    MonsterSpawn(PyEffectMonsterSpawn),
    CombatStart(PyEffectCombatStart),
    AdventurerSearch(PyEffectAdventurerSearch),
    RelicGrantSpecific(PyEffectRelicGrantSpecific),
    EventAdvanceState(PyEffectEventAdvanceState),
    ScrapOozeReach(PyEffectScrapOozeReach),
    EventConsume(PyEffectEventConsume),
    CardDiscoverPick(PyEffectCardDiscoverPick),
    CardPurge(PyEffectCardPurge),
    CardUpgrade(PyEffectCardUpgrade),
    CardDuplicate(PyEffectCardDuplicate),
    CardTransform(PyEffectCardTransform),
    CardAddRandom(PyEffectCardAddRandom),
    CardDrawIfNoAttacks(PyEffectCardDrawIfNoAttacks),
    HandOfGreedProc(PyEffectHandOfGreedProc),
    CardExhaust(PyEffectCardExhaust),
    CardMove(PyEffectCardMove),
    CardPlayFromDrawTop(PyEffectCardPlayFromDrawTop),
    CardCostRandomize(PyEffectCardCostRandomize),
    Gamble(PyEffectGamble),
    LiquidMemoriesPick(PyEffectLiquidMemoriesPick),
    CombatEscape(PyEffectCombatEscape),
}

variant_union!(PyEffect {
    DamagePhysical => PyEffectDamagePhysical,
    DamagePhysicalIfPoisoned => PyEffectDamagePhysicalIfPoisoned,
    HeelHookProc => PyEffectHeelHookProc,
    EscapePlanCheck => PyEffectEscapePlanCheck,
    GlassKnifeDecay => PyEffectGlassKnifeDecay,
    CardSetupPick => PyEffectCardSetupPick,
    CardNightmarePick => PyEffectCardNightmarePick,
    DistractionAdd => PyEffectDistractionAdd,
    SetCostOverride => PyEffectSetCostOverride,
    DamageFinisher => PyEffectDamageFinisher,
    DamageFlechettes => PyEffectDamageFlechettes,
    UnloadDiscard => PyEffectUnloadDiscard,
    StormOfSteelProc => PyEffectStormOfSteelProc,
    SneakyStrikeProc => PyEffectSneakyStrikeProc,
    BlockGain => PyEffectBlockGain,
    ModifierGain => PyEffectModifierGain,
    ModifierMultiply => PyEffectModifierMultiply,
    ModifierRemove => PyEffectModifierRemove,
    EnergyDelta => PyEffectEnergyDelta,
    CardAdd => PyEffectCardAdd,
    CardDraw => PyEffectCardDraw,
    CardDrawUpTo => PyEffectCardDrawUpTo,
    CardDiscard => PyEffectCardDiscard,
    CardRetain => PyEffectCardRetain,
    DamageMindBlast => PyEffectDamageMindBlast,
    ShuffleDiscardPileIntoDrawPile => PyEffectShuffleDiscardPileIntoDrawPile,
    MaxHealthDelta => PyEffectMaxHealthDelta,
    HealthDelta => PyEffectHealthDelta,
    PotionAddRandom => PyEffectPotionAddRandom,
    PotionDiscard => PyEffectPotionDiscard,
    RewardRollPotions => PyEffectRewardRollPotions,
    CardDiscoverRoll => PyEffectCardDiscoverRoll,
    GoldDelta => PyEffectGoldDelta,
    RelicGrantRandom => PyEffectRelicGrantRandom,
    WheelSpin => PyEffectWheelSpin,
    BonfireOffer => PyEffectBonfireOffer,
    FaceTrade => PyEffectFaceTrade,
    MonsterSpawn => PyEffectMonsterSpawn,
    CombatStart => PyEffectCombatStart,
    AdventurerSearch => PyEffectAdventurerSearch,
    RelicGrantSpecific => PyEffectRelicGrantSpecific,
    EventAdvanceState => PyEffectEventAdvanceState,
    ScrapOozeReach => PyEffectScrapOozeReach,
    EventConsume => PyEffectEventConsume,
    CardDiscoverPick => PyEffectCardDiscoverPick,
    CardPurge => PyEffectCardPurge,
    CardUpgrade => PyEffectCardUpgrade,
    CardDuplicate => PyEffectCardDuplicate,
    CardTransform => PyEffectCardTransform,
    CardAddRandom => PyEffectCardAddRandom,
    CardDrawIfNoAttacks => PyEffectCardDrawIfNoAttacks,
    HandOfGreedProc => PyEffectHandOfGreedProc,
    CardExhaust => PyEffectCardExhaust,
    CardMove => PyEffectCardMove,
    CardPlayFromDrawTop => PyEffectCardPlayFromDrawTop,
    CardCostRandomize => PyEffectCardCostRandomize,
    Gamble => PyEffectGamble,
    LiquidMemoriesPick => PyEffectLiquidMemoriesPick,
    CombatEscape => PyEffectCombatEscape,
});

pub(crate) fn snapshot_effect(effect: &Effect) -> PyEffect {
    let target = match effect.target {
        Target::Resolve {
            candidate_pool,
            selection_kind,
        } => Some(PyTarget {
            candidate_pool: candidate_pool.into(),
            selection_kind: selection_kind.into(),
        }),
        Target::Direct(None) => None,
        Target::Direct(Some(_)) => panic!(
            "snapshot_effect: unexpected Direct(Some) on static card effect: {:?}",
            effect,
        ),
    };
    match effect.kind {
        EffectKind::DamagePhysical { amount } => {
            PyEffect::DamagePhysical(PyEffectDamagePhysical { amount, target })
        }
        EffectKind::DamagePhysicalIfPoisoned { amount } => {
            PyEffect::DamagePhysicalIfPoisoned(PyEffectDamagePhysicalIfPoisoned { amount, target })
        }
        EffectKind::HeelHookProc => PyEffect::HeelHookProc(PyEffectHeelHookProc { target }),
        EffectKind::EscapePlanCheck { block } => {
            PyEffect::EscapePlanCheck(PyEffectEscapePlanCheck { block, target })
        }
        EffectKind::GlassKnifeDecay { delta } => {
            PyEffect::GlassKnifeDecay(PyEffectGlassKnifeDecay { delta, target })
        }
        EffectKind::CardSetupPick { free, bottom } => {
            PyEffect::CardSetupPick(PyEffectCardSetupPick {
                free,
                bottom,
                target,
            })
        }
        EffectKind::CardNightmarePick => {
            PyEffect::CardNightmarePick(PyEffectCardNightmarePick { target })
        }
        EffectKind::DistractionAdd => PyEffect::DistractionAdd(PyEffectDistractionAdd { target }),
        EffectKind::SetCostOverride {
            amount,
            only_reduce,
            scope,
        } => PyEffect::SetCostOverride(PyEffectSetCostOverride {
            amount,
            only_reduce,
            scope: scope.into(),
            target,
        }),
        EffectKind::DamageFinisher { damage } => {
            PyEffect::DamageFinisher(PyEffectDamageFinisher { damage, target })
        }
        EffectKind::DamageFlechettes { damage } => {
            PyEffect::DamageFlechettes(PyEffectDamageFlechettes { damage, target })
        }
        EffectKind::UnloadDiscard => PyEffect::UnloadDiscard(PyEffectUnloadDiscard { target }),
        EffectKind::StormOfSteelProc { upgraded } => {
            PyEffect::StormOfSteelProc(PyEffectStormOfSteelProc { upgraded, target })
        }
        EffectKind::SneakyStrikeProc { energy } => {
            PyEffect::SneakyStrikeProc(PyEffectSneakyStrikeProc { energy, target })
        }
        EffectKind::BlockGain { amount } => {
            PyEffect::BlockGain(PyEffectBlockGain { amount, target })
        }
        EffectKind::ModifierGain { kind, stacks } => PyEffect::ModifierGain(PyEffectModifierGain {
            kind: kind.into(),
            stacks,
            target,
        }),
        EffectKind::ModifierMultiply { kind, factor } => {
            PyEffect::ModifierMultiply(PyEffectModifierMultiply {
                kind: kind.into(),
                factor,
                target,
            })
        }
        EffectKind::ModifierRemove { kind } => PyEffect::ModifierRemove(PyEffectModifierRemove {
            kind: kind.into(),
            target,
        }),
        EffectKind::EnergyDelta { sign, amount } => PyEffect::EnergyDelta(PyEffectEnergyDelta {
            sign: sign.into(),
            amount,
            target,
        }),
        EffectKind::CardAdd {
            card_name,
            pile,
            count,
            upgraded,
        } => PyEffect::CardAdd(PyEffectCardAdd {
            card_name: card_name.as_str().to_string(),
            pile: pile.into(),
            count,
            upgraded,
            target,
        }),
        EffectKind::CardDraw { count } => PyEffect::CardDraw(PyEffectCardDraw { count, target }),
        EffectKind::CardDrawUpTo { amount } => {
            PyEffect::CardDrawUpTo(PyEffectCardDrawUpTo { amount, target })
        }
        EffectKind::CardDiscard { source: _ } => {
            PyEffect::CardDiscard(PyEffectCardDiscard { target })
        }
        EffectKind::CardRetain => PyEffect::CardRetain(PyEffectCardRetain { target }),
        EffectKind::DamageMindBlast => {
            PyEffect::DamageMindBlast(PyEffectDamageMindBlast { target })
        }
        EffectKind::ShuffleDiscardPileIntoDrawPile => {
            PyEffect::ShuffleDiscardPileIntoDrawPile(PyEffectShuffleDiscardPileIntoDrawPile {
                target,
            })
        }
        EffectKind::GoldDelta { sign, amount } => PyEffect::GoldDelta(PyEffectGoldDelta {
            sign: sign.into(),
            amount: amount.into(),
            target,
        }),
        EffectKind::HealthDelta { sign, amount } => PyEffect::HealthDelta(PyEffectHealthDelta {
            sign: sign.into(),
            amount: amount.into(),
            target,
        }),
        EffectKind::MaxHealthDelta { sign, amount } => {
            PyEffect::MaxHealthDelta(PyEffectMaxHealthDelta {
                sign: sign.into(),
                amount: amount.into(),
                target,
            })
        }
        EffectKind::CardPurge => PyEffect::CardPurge(PyEffectCardPurge { target }),
        EffectKind::CardDuplicate => PyEffect::CardDuplicate(PyEffectCardDuplicate { target }),
        EffectKind::CardTransform => PyEffect::CardTransform(PyEffectCardTransform { target }),
        EffectKind::RelicGrantRandom => {
            PyEffect::RelicGrantRandom(PyEffectRelicGrantRandom { target })
        }
        EffectKind::WheelSpin => PyEffect::WheelSpin(PyEffectWheelSpin { target }),
        EffectKind::BonfireOffer => PyEffect::BonfireOffer(PyEffectBonfireOffer { target }),
        EffectKind::FaceTrade => PyEffect::FaceTrade(PyEffectFaceTrade { target }),
        EffectKind::MonsterSpawn { name } => PyEffect::MonsterSpawn(PyEffectMonsterSpawn {
            name: name.into(),
            target,
        }),
        EffectKind::CombatStart {
            event_gold,
            event_relic,
            event_relic_roll,
        } => PyEffect::CombatStart(PyEffectCombatStart {
            event_gold: event_gold.map(Into::into),
            event_relic: event_relic.map(Into::into),
            event_relic_roll,
            target,
        }),
        EffectKind::AdventurerSearch => {
            PyEffect::AdventurerSearch(PyEffectAdventurerSearch { target })
        }
        EffectKind::RelicGrantSpecific {
            name,
            fallback_circlet,
        } => PyEffect::RelicGrantSpecific(PyEffectRelicGrantSpecific {
            name: name.into(),
            fallback_circlet,
            target,
        }),
        EffectKind::EventAdvanceState { delta } => {
            PyEffect::EventAdvanceState(PyEffectEventAdvanceState { delta, target })
        }
        EffectKind::ScrapOozeReach {
            dmg,
            chance,
            advance_on_miss,
        } => PyEffect::ScrapOozeReach(PyEffectScrapOozeReach {
            dmg,
            chance,
            advance_on_miss,
            target,
        }),
        EffectKind::EventConsume => PyEffect::EventConsume(PyEffectEventConsume { target }),
        EffectKind::PotionAddRandom { limited } => {
            PyEffect::PotionAddRandom(PyEffectPotionAddRandom { limited, target })
        }
        EffectKind::PotionDiscard => PyEffect::PotionDiscard(PyEffectPotionDiscard { target }),
        EffectKind::RewardRollPotions { count } => {
            PyEffect::RewardRollPotions(PyEffectRewardRollPotions { count, target })
        }
        EffectKind::CardDiscoverRoll { kind, color, count } => {
            PyEffect::CardDiscoverRoll(PyEffectCardDiscoverRoll {
                kind: kind.map(|k| k.into()),
                color: color.into(),
                count,
                target,
            })
        }
        EffectKind::CardUpgrade => PyEffect::CardUpgrade(PyEffectCardUpgrade { target }),
        EffectKind::CardDiscoverPick => {
            PyEffect::CardDiscoverPick(PyEffectCardDiscoverPick { target })
        }
        EffectKind::CardAddRandom {
            color,
            kind,
            pile,
            count,
            cost_zero,
            upgraded,
        } => PyEffect::CardAddRandom(PyEffectCardAddRandom {
            color: color.into(),
            kind: kind.map(|k| k.into()),
            pile: pile.into(),
            count,
            cost_zero: cost_zero.map(|c| c.into()),
            upgraded,
            target,
        }),
        EffectKind::CardDrawIfNoAttacks { count } => {
            PyEffect::CardDrawIfNoAttacks(PyEffectCardDrawIfNoAttacks { count, target })
        }
        EffectKind::HandOfGreedProc { gold } => {
            PyEffect::HandOfGreedProc(PyEffectHandOfGreedProc { gold, target })
        }
        EffectKind::CardExhaust => PyEffect::CardExhaust(PyEffectCardExhaust { target }),
        EffectKind::CardMove { pile } => PyEffect::CardMove(PyEffectCardMove {
            pile: pile.into(),
            target,
        }),
        EffectKind::CardPlayFromDrawTop => {
            PyEffect::CardPlayFromDrawTop(PyEffectCardPlayFromDrawTop { target })
        }
        EffectKind::CardCostRandomize => {
            PyEffect::CardCostRandomize(PyEffectCardCostRandomize { target })
        }
        EffectKind::Gamble {
            choose_discards,
            discards_before,
        } => PyEffect::Gamble(PyEffectGamble {
            choose_discards,
            discards_before,
            target,
        }),
        EffectKind::LiquidMemoriesPick => {
            PyEffect::LiquidMemoriesPick(PyEffectLiquidMemoriesPick { target })
        }
        EffectKind::CombatEscape => PyEffect::CombatEscape(PyEffectCombatEscape { target }),
        other => unreachable!(
            "snapshot_effect: unexpected EffectKind on static card effect: {:?}",
            other
        ),
    }
}
