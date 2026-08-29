use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::type_hint_union;
use pyo3::type_object::PyTypeInfo;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;

use super::amount::PyAmount;
use super::amount::PyAmountScalar;
use super::amount::PyDeltaSign;
use super::card::PyCardColor;
use super::card::PyCardKind;
use super::card::PyCardName;
use super::card::PyCardPile;
use super::card::PyCardRarity;
use super::card::PyCostScope;
use super::macros::flat_variants;
use super::modifier::PyModifierKind;
use super::monster::PyMonsterName;
use super::relic::PyRelicName;
use super::relic::PyRelicTier;
use super::target::PyTarget;

// Mirrors only EffectKind variants reachable from static Card/Monster defs; snapshot_effect panics on runtime-only variants
flat_variants!(PyEffect {
    DamagePhysical => PyEffectDamagePhysical as "EffectDamagePhysical" { amount: u16, lifesteal: bool, target: PyTarget },
    DamagePhysicalIfPoisoned => PyEffectDamagePhysicalIfPoisoned as "EffectDamagePhysicalIfPoisoned" { amount: u16, target: PyTarget },
    HeelHookProc => PyEffectHeelHookProc as "EffectHeelHookProc" { target: PyTarget },
    EscapePlanCheck => PyEffectEscapePlanCheck as "EffectEscapePlanCheck" { block: u16, target: PyTarget },
    GlassKnifeDecay => PyEffectGlassKnifeDecay as "EffectGlassKnifeDecay" { delta: i16, target: PyTarget },
    CardSetupPick => PyEffectCardSetupPick as "EffectCardSetupPick" { free: bool, bottom: bool, target: PyTarget },
    CardNightmarePick => PyEffectCardNightmarePick as "EffectCardNightmarePick" { target: PyTarget },
    DistractionAdd => PyEffectDistractionAdd as "EffectDistractionAdd",
    SetCostOverride => PyEffectSetCostOverride as "EffectSetCostOverride" { amount: u8, only_reduce: bool, random: bool, scope: PyCostScope, target: PyTarget },
    DamageFinisher => PyEffectDamageFinisher as "EffectDamageFinisher" { damage: u16, target: PyTarget },
    DamageFlechettes => PyEffectDamageFlechettes as "EffectDamageFlechettes" { damage: u16, target: PyTarget },
    UnloadDiscard => PyEffectUnloadDiscard as "EffectUnloadDiscard",
    StormOfSteelProc => PyEffectStormOfSteelProc as "EffectStormOfSteelProc" { upgraded: bool },
    SneakyStrikeProc => PyEffectSneakyStrikeProc as "EffectSneakyStrikeProc" { energy: u8 },
    BlockGain => PyEffectBlockGain as "EffectBlockGain" { amount: u16, target: PyTarget },
    ModifierGain => PyEffectModifierGain as "EffectModifierGain" { kind: PyModifierKind, stacks: i16, target: PyTarget },
    ModifierMultiply => PyEffectModifierMultiply as "EffectModifierMultiply" { kind: PyModifierKind, factor: u8, target: PyTarget },
    EnergyDelta => PyEffectEnergyDelta as "EffectEnergyDelta" { sign: PyDeltaSign, amount: u16 },
    CardAdd => PyEffectCardAdd as "EffectCardAdd" { card_name: PyCardName, pile: PyCardPile, count: u16, upgraded: bool },
    CardDraw => PyEffectCardDraw as "EffectCardDraw" { count: u16 },
    CardDrawUpTo => PyEffectCardDrawUpTo as "EffectCardDrawUpTo" { amount: u8 },
    CardDiscard => PyEffectCardDiscard as "EffectCardDiscard" { target: PyTarget },
    CardRetain => PyEffectCardRetain as "EffectCardRetain" { target: PyTarget },
    DamageMindBlast => PyEffectDamageMindBlast as "EffectDamageMindBlast" { target: PyTarget },
    ShuffleDiscardPileIntoDrawPile => PyEffectShuffleDiscardPileIntoDrawPile as "EffectShuffleDiscardPileIntoDrawPile",
    MaxHealthDelta => PyEffectMaxHealthDelta as "EffectMaxHealthDelta" { sign: PyDeltaSign, amount: PyAmountScalar, target: PyTarget },
    HealthDelta => PyEffectHealthDelta as "EffectHealthDelta" { sign: PyDeltaSign, amount: PyAmountScalar, target: PyTarget },
    PotionAddRandom => PyEffectPotionAddRandom as "EffectPotionAddRandom" { limited: bool },
    PotionDiscard => PyEffectPotionDiscard as "EffectPotionDiscard" { target: PyTarget },
    RewardRollPotions => PyEffectRewardRollPotions as "EffectRewardRollPotions" { count: u8, uniform: bool },
    CardDiscoverRoll => PyEffectCardDiscoverRoll as "EffectCardDiscoverRoll" { kind: Option<PyCardKind>, color: PyCardColor, exclude: Vec<PyCardName>, count: u8 },
    GoldDelta => PyEffectGoldDelta as "EffectGoldDelta" { sign: PyDeltaSign, amount: PyAmount },
    RelicGrantRandom => PyEffectRelicGrantRandom as "EffectRelicGrantRandom" { tier: Option<PyRelicTier> },
    WheelSpin => PyEffectWheelSpin as "EffectWheelSpin",
    BonfireOffer => PyEffectBonfireOffer as "EffectBonfireOffer" { target: PyTarget },
    CardBottle => PyEffectCardBottle as "EffectCardBottle" { target: PyTarget },
    MonsterSpawn => PyEffectMonsterSpawn as "EffectMonsterSpawn" { name: PyMonsterName },
    CombatStart => PyEffectCombatStart as "EffectCombatStart",
    AdventurerSearch => PyEffectAdventurerSearch as "EffectAdventurerSearch",
    RelicGrantSpecific => PyEffectRelicGrantSpecific as "EffectRelicGrantSpecific" { name: PyRelicName, fallback_circlet: bool },
    EventAdvanceState => PyEffectEventAdvanceState as "EffectEventAdvanceState" { delta: i8 },
    ScrapOozeReach => PyEffectScrapOozeReach as "EffectScrapOozeReach" { chance: u8, advance_on_miss: bool },
    EventConsume => PyEffectEventConsume as "EffectEventConsume",
    CardDiscoverPick => PyEffectCardDiscoverPick as "EffectCardDiscoverPick" { cost_zero: Option<PyCostScope>, pile: PyCardPile, target: PyTarget },
    CardPurge => PyEffectCardPurge as "EffectCardPurge" { target: PyTarget },
    CardUpgrade => PyEffectCardUpgrade as "EffectCardUpgrade" { target: PyTarget },
    CardDuplicate => PyEffectCardDuplicate as "EffectCardDuplicate" { target: PyTarget },
    CardTransform => PyEffectCardTransform as "EffectCardTransform" { upgraded: bool, target: PyTarget },
    CardAddRandom => PyEffectCardAddRandom as "EffectCardAddRandom" { color: PyCardColor, kind: Option<PyCardKind>, pile: PyCardPile, count: u8, cost_zero: Option<PyCostScope>, upgraded: bool, rarity: Option<PyCardRarity> },
    CardDrawIfNoAttacks => PyEffectCardDrawIfNoAttacks as "EffectCardDrawIfNoAttacks" { count: u16 },
    HandOfGreedProc => PyEffectHandOfGreedProc as "EffectHandOfGreedProc" { gold: u16, target: PyTarget },
    RitualDaggerProc => PyEffectRitualDaggerProc as "EffectRitualDaggerProc" { bump: u16, target: PyTarget },
    CardExhaust => PyEffectCardExhaust as "EffectCardExhaust" { target: PyTarget },
    CardMove => PyEffectCardMove as "EffectCardMove" { pile: PyCardPile, cost_zero: Option<PyCostScope>, target: PyTarget },
    CardPlayFromDrawTop => PyEffectCardPlayFromDrawTop as "EffectCardPlayFromDrawTop",
    Gamble => PyEffectGamble as "EffectGamble" { choose_discards: bool },
    CombatEnd => PyEffectCombatEnd as "EffectCombatEnd" { escaped_character: bool },
    RelicLose => PyEffectRelicLose as "EffectRelicLose" { target: PyTarget },
    RewardRollNeowCards => PyEffectRewardRollNeowCards as "EffectRewardRollNeowCards" { colorless: bool, rare_only: bool },
    StrengthLoseTemp => PyEffectStrengthLoseTemp as "EffectStrengthLoseTemp" { stacks: i16, target: PyTarget },
    MausoleumOpen => PyEffectMausoleumOpen as "EffectMausoleumOpen",
    KnowingSkullCostBump => PyEffectKnowingSkullCostBump as "EffectKnowingSkullCostBump",
    JoustBet => PyEffectJoustBet as "EffectJoustBet" { on_owner: bool },
    RewardRollLibraryCards => PyEffectRewardRollLibraryCards as "EffectRewardRollLibraryCards",
    RelicGrantPool => PyEffectRelicGrantPool as "EffectRelicGrantPool" { pool: Vec<PyRelicName> },
    DebuffsClear => PyEffectDebuffsClear as "EffectDebuffsClear" { target: PyTarget },
    GremlinSummon => PyEffectGremlinSummon as "EffectGremlinSummon",
    HexaghostBurnIncrease => PyEffectHexaghostBurnIncrease as "EffectHexaghostBurnIncrease" { count: u8 },
    ModifierRemove => PyEffectModifierRemove as "EffectModifierRemove" { kind: PyModifierKind, target: PyTarget },
    MonsterEscape => PyEffectMonsterEscape as "EffectMonsterEscape" { target: PyTarget },
    MonsterSplit => PyEffectMonsterSplit as "EffectMonsterSplit" { name: PyMonsterName, target: PyTarget },
    StasisSteal => PyEffectStasisSteal as "EffectStasisSteal",
});

// The kinds that can park in `state.effect_pending`
flat_variants!(@enum PyEffectPending {
    BonfireOffer => PyEffectBonfireOffer,
    CardBottle => PyEffectCardBottle,
    CardDiscard => PyEffectCardDiscard,
    CardDiscoverPick => PyEffectCardDiscoverPick,
    CardDuplicate => PyEffectCardDuplicate,
    CardExhaust => PyEffectCardExhaust,
    CardMove => PyEffectCardMove,
    CardNightmarePick => PyEffectCardNightmarePick,
    CardPurge => PyEffectCardPurge,
    CardRetain => PyEffectCardRetain,
    CardSetupPick => PyEffectCardSetupPick,
    CardTransform => PyEffectCardTransform,
    CardUpgrade => PyEffectCardUpgrade,
    RelicLose => PyEffectRelicLose
});

pub(crate) fn snapshot_effect_pending(effect: &Effect) -> PyEffectPending {
    match snapshot_effect(effect) {
        PyEffect::BonfireOffer(v) => PyEffectPending::BonfireOffer(v),
        PyEffect::CardBottle(v) => PyEffectPending::CardBottle(v),
        PyEffect::CardDiscard(v) => PyEffectPending::CardDiscard(v),
        PyEffect::CardDiscoverPick(v) => PyEffectPending::CardDiscoverPick(v),
        PyEffect::CardDuplicate(v) => PyEffectPending::CardDuplicate(v),
        PyEffect::CardExhaust(v) => PyEffectPending::CardExhaust(v),
        PyEffect::CardMove(v) => PyEffectPending::CardMove(v),
        PyEffect::CardNightmarePick(v) => PyEffectPending::CardNightmarePick(v),
        PyEffect::CardPurge(v) => PyEffectPending::CardPurge(v),
        PyEffect::CardRetain(v) => PyEffectPending::CardRetain(v),
        PyEffect::CardSetupPick(v) => PyEffectPending::CardSetupPick(v),
        PyEffect::CardTransform(v) => PyEffectPending::CardTransform(v),
        PyEffect::CardUpgrade(v) => PyEffectPending::CardUpgrade(v),
        PyEffect::RelicLose(v) => PyEffectPending::RelicLose(v),
        other => unreachable!("effect kind cannot halt: {:?}", other),
    }
}

pub(crate) fn snapshot_effect(effect: &Effect) -> PyEffect {
    let target = match effect.target {
        Target::Resolve {
            candidate_pool,
            filter,
            selection_kind,
        } => Some(PyTarget {
            candidate_pool: candidate_pool.into(),
            filter: filter.into(),
            selection_kind: selection_kind.into(),
        }),
        Target::Direct(None) => None,
        Target::Direct(Some(_)) => unreachable!("snapshotted effects never carry arena ids"),
    };
    snapshot_effect_rows(effect, target)
}

fn require_target(target: Option<PyTarget>) -> PyTarget {
    target.expect("row requires a resolved target")
}

fn snapshot_effect_rows(effect: &Effect, target: Option<PyTarget>) -> PyEffect {
    if target.is_some() {
        assert!(
            !matches!(
                effect.kind,
                EffectKind::AdventurerSearch
                    | EffectKind::CardAdd { .. }
                    | EffectKind::CardAddRandom { .. }
                    | EffectKind::CardDiscoverRoll { .. }
                    | EffectKind::CardDraw { .. }
                    | EffectKind::CardDrawIfNoAttacks { .. }
                    | EffectKind::CardDrawUpTo { .. }
                    | EffectKind::CardPlayFromDrawTop
                    | EffectKind::CombatEnd { .. }
                    | EffectKind::CombatStart
                    | EffectKind::DistractionAdd
                    | EffectKind::EnergyDelta { .. }
                    | EffectKind::EventAdvanceState { .. }
                    | EffectKind::EventConsume
                    | EffectKind::Gamble { .. }
                    | EffectKind::GoldDelta { .. }
                    | EffectKind::GremlinSummon
                    | EffectKind::HexaghostBurnIncrease { .. }
                    | EffectKind::JoustBet { .. }
                    | EffectKind::KnowingSkullCostBump
                    | EffectKind::MausoleumOpen
                    | EffectKind::MonsterSpawn { .. }
                    | EffectKind::PotionAddRandom { .. }
                    | EffectKind::RelicGrantPool { .. }
                    | EffectKind::RelicGrantRandom { .. }
                    | EffectKind::RelicGrantSpecific { .. }
                    | EffectKind::RewardRollLibraryCards
                    | EffectKind::RewardRollNeowCards { .. }
                    | EffectKind::RewardRollPotions { .. }
                    | EffectKind::ScrapOozeReach { .. }
                    | EffectKind::ShuffleDiscardPileIntoDrawPile
                    | EffectKind::SneakyStrikeProc { .. }
                    | EffectKind::StasisSteal
                    | EffectKind::StormOfSteelProc { .. }
                    | EffectKind::UnloadDiscard
                    | EffectKind::WheelSpin
            ),
            "effect kind {:?} resolved a target but its FFI row has no target field",
            effect.kind
        );
    }
    match effect.kind {
        EffectKind::DamagePhysical { amount, lifesteal } => {
            PyEffect::DamagePhysical(PyEffectDamagePhysical {
                amount,
                lifesteal,
                target: require_target(target),
            })
        }
        EffectKind::DamagePhysicalIfPoisoned { amount } => {
            PyEffect::DamagePhysicalIfPoisoned(PyEffectDamagePhysicalIfPoisoned {
                amount,
                target: require_target(target),
            })
        }
        EffectKind::HeelHookProc => PyEffect::HeelHookProc(PyEffectHeelHookProc {
            target: require_target(target),
        }),
        EffectKind::EscapePlanCheck { block } => {
            PyEffect::EscapePlanCheck(PyEffectEscapePlanCheck {
                block,
                target: require_target(target),
            })
        }
        EffectKind::GlassKnifeDecay { delta } => {
            PyEffect::GlassKnifeDecay(PyEffectGlassKnifeDecay {
                delta,
                target: require_target(target),
            })
        }
        EffectKind::CardSetupPick { free, bottom } => {
            PyEffect::CardSetupPick(PyEffectCardSetupPick {
                free,
                bottom,
                target: require_target(target),
            })
        }
        EffectKind::CardNightmarePick => PyEffect::CardNightmarePick(PyEffectCardNightmarePick {
            target: require_target(target),
        }),
        EffectKind::DistractionAdd => PyEffect::DistractionAdd(PyEffectDistractionAdd),
        EffectKind::SetCostOverride {
            amount,
            only_reduce,
            random,
            scope,
        } => PyEffect::SetCostOverride(PyEffectSetCostOverride {
            amount,
            only_reduce,
            random,
            scope: scope.into(),
            target: require_target(target),
        }),
        EffectKind::DamageFinisher { damage } => PyEffect::DamageFinisher(PyEffectDamageFinisher {
            damage,
            target: require_target(target),
        }),
        EffectKind::DamageFlechettes { damage } => {
            PyEffect::DamageFlechettes(PyEffectDamageFlechettes {
                damage,
                target: require_target(target),
            })
        }
        EffectKind::UnloadDiscard => PyEffect::UnloadDiscard(PyEffectUnloadDiscard),
        EffectKind::StormOfSteelProc { upgraded } => {
            PyEffect::StormOfSteelProc(PyEffectStormOfSteelProc { upgraded })
        }
        EffectKind::SneakyStrikeProc { energy } => {
            PyEffect::SneakyStrikeProc(PyEffectSneakyStrikeProc { energy })
        }
        EffectKind::BlockGain { amount } => PyEffect::BlockGain(PyEffectBlockGain {
            amount,
            target: require_target(target),
        }),
        EffectKind::ModifierGain { kind, stacks } => PyEffect::ModifierGain(PyEffectModifierGain {
            kind: kind.into(),
            stacks,
            target: require_target(target),
        }),
        EffectKind::ModifierMultiply { kind, factor } => {
            PyEffect::ModifierMultiply(PyEffectModifierMultiply {
                kind: kind.into(),
                factor,
                target: require_target(target),
            })
        }
        EffectKind::EnergyDelta { sign, amount } => PyEffect::EnergyDelta(PyEffectEnergyDelta {
            sign: sign.into(),
            amount,
        }),
        EffectKind::CardAdd {
            card_name,
            pile,
            count,
            upgraded,
        } => PyEffect::CardAdd(PyEffectCardAdd {
            card_name: card_name.into(),
            pile: pile.into(),
            count,
            upgraded,
        }),
        EffectKind::CardDraw { count } => PyEffect::CardDraw(PyEffectCardDraw { count }),
        EffectKind::CardDrawUpTo { amount } => {
            PyEffect::CardDrawUpTo(PyEffectCardDrawUpTo { amount })
        }
        EffectKind::CardDiscard { source: _ } => PyEffect::CardDiscard(PyEffectCardDiscard {
            target: require_target(target),
        }),
        EffectKind::CardRetain => PyEffect::CardRetain(PyEffectCardRetain {
            target: require_target(target),
        }),
        EffectKind::DamageMindBlast => PyEffect::DamageMindBlast(PyEffectDamageMindBlast {
            target: require_target(target),
        }),
        EffectKind::ShuffleDiscardPileIntoDrawPile => {
            PyEffect::ShuffleDiscardPileIntoDrawPile(PyEffectShuffleDiscardPileIntoDrawPile)
        }
        EffectKind::GoldDelta { sign, amount } => PyEffect::GoldDelta(PyEffectGoldDelta {
            sign: sign.into(),
            amount: amount.into(),
        }),
        EffectKind::HealthDelta { sign, amount } => PyEffect::HealthDelta(PyEffectHealthDelta {
            sign: sign.into(),
            amount: amount.into(),
            target: require_target(target),
        }),
        EffectKind::MaxHealthDelta { sign, amount } => {
            PyEffect::MaxHealthDelta(PyEffectMaxHealthDelta {
                sign: sign.into(),
                amount: amount.into(),
                target: require_target(target),
            })
        }
        EffectKind::CardPurge => PyEffect::CardPurge(PyEffectCardPurge {
            target: require_target(target),
        }),
        EffectKind::CardDuplicate => PyEffect::CardDuplicate(PyEffectCardDuplicate {
            target: require_target(target),
        }),
        EffectKind::CardTransform { upgraded } => PyEffect::CardTransform(PyEffectCardTransform {
            upgraded,
            target: require_target(target),
        }),
        EffectKind::RelicGrantRandom { tier } => {
            PyEffect::RelicGrantRandom(PyEffectRelicGrantRandom {
                tier: tier.map(Into::into),
            })
        }
        EffectKind::RelicLose => PyEffect::RelicLose(PyEffectRelicLose {
            target: require_target(target),
        }),
        EffectKind::WheelSpin => PyEffect::WheelSpin(PyEffectWheelSpin),
        EffectKind::BonfireOffer => PyEffect::BonfireOffer(PyEffectBonfireOffer {
            target: require_target(target),
        }),
        EffectKind::CardBottle => PyEffect::CardBottle(PyEffectCardBottle {
            target: require_target(target),
        }),
        EffectKind::MonsterSpawn { name, .. } => {
            PyEffect::MonsterSpawn(PyEffectMonsterSpawn { name: name.into() })
        }
        EffectKind::CombatStart => PyEffect::CombatStart(PyEffectCombatStart),
        EffectKind::AdventurerSearch => PyEffect::AdventurerSearch(PyEffectAdventurerSearch),
        EffectKind::RelicGrantSpecific {
            name,
            fallback_circlet,
        } => PyEffect::RelicGrantSpecific(PyEffectRelicGrantSpecific {
            name: name.into(),
            fallback_circlet,
        }),
        EffectKind::EventAdvanceState { delta } => {
            PyEffect::EventAdvanceState(PyEffectEventAdvanceState { delta })
        }
        EffectKind::ScrapOozeReach {
            chance,
            advance_on_miss,
        } => PyEffect::ScrapOozeReach(PyEffectScrapOozeReach {
            chance,
            advance_on_miss,
        }),
        EffectKind::EventConsume => PyEffect::EventConsume(PyEffectEventConsume),
        EffectKind::PotionAddRandom { limited } => {
            PyEffect::PotionAddRandom(PyEffectPotionAddRandom { limited })
        }
        EffectKind::PotionDiscard => PyEffect::PotionDiscard(PyEffectPotionDiscard {
            target: require_target(target),
        }),
        EffectKind::RewardRollPotions { count, uniform } => {
            PyEffect::RewardRollPotions(PyEffectRewardRollPotions { count, uniform })
        }
        EffectKind::RewardRollNeowCards {
            colorless,
            rare_only,
        } => PyEffect::RewardRollNeowCards(PyEffectRewardRollNeowCards {
            colorless,
            rare_only,
        }),
        EffectKind::CardDiscoverRoll {
            kind,
            color,
            exclude,
            count,
        } => PyEffect::CardDiscoverRoll(PyEffectCardDiscoverRoll {
            kind: kind.map(|card_kind| card_kind.into()),
            color: color.into(),
            exclude: exclude.iter().map(|&card_name| card_name.into()).collect(),
            count,
        }),
        EffectKind::CardUpgrade => PyEffect::CardUpgrade(PyEffectCardUpgrade {
            target: require_target(target),
        }),
        EffectKind::CardDiscoverPick { cost_zero, pile } => {
            PyEffect::CardDiscoverPick(PyEffectCardDiscoverPick {
                pile: pile.into(),
                cost_zero: cost_zero.map(|cost_scope| cost_scope.into()),
                target: require_target(target),
            })
        }
        EffectKind::CardAddRandom {
            color,
            kind,
            pile,
            count,
            cost_zero,
            upgraded,
            rarity,
        } => PyEffect::CardAddRandom(PyEffectCardAddRandom {
            color: color.into(),
            kind: kind.map(|card_kind| card_kind.into()),
            pile: pile.into(),
            count,
            cost_zero: cost_zero.map(|cost_scope| cost_scope.into()),
            upgraded,
            rarity: rarity.map(Into::into),
        }),
        EffectKind::CardDrawIfNoAttacks { count } => {
            PyEffect::CardDrawIfNoAttacks(PyEffectCardDrawIfNoAttacks { count })
        }
        EffectKind::HandOfGreedProc { gold } => {
            PyEffect::HandOfGreedProc(PyEffectHandOfGreedProc {
                gold,
                target: require_target(target),
            })
        }
        EffectKind::RitualDaggerProc { bump } => {
            PyEffect::RitualDaggerProc(PyEffectRitualDaggerProc {
                bump,
                target: require_target(target),
            })
        }
        EffectKind::CardExhaust => PyEffect::CardExhaust(PyEffectCardExhaust {
            target: require_target(target),
        }),
        EffectKind::CardMove { pile, cost_zero } => PyEffect::CardMove(PyEffectCardMove {
            pile: pile.into(),
            cost_zero: cost_zero.map(|cost_scope| cost_scope.into()),
            target: require_target(target),
        }),
        EffectKind::CardPlayFromDrawTop => {
            PyEffect::CardPlayFromDrawTop(PyEffectCardPlayFromDrawTop)
        }
        EffectKind::Gamble {
            choose_discards, ..
        } => PyEffect::Gamble(PyEffectGamble { choose_discards }),
        EffectKind::CombatEnd { escaped_character } => {
            PyEffect::CombatEnd(PyEffectCombatEnd { escaped_character })
        }
        EffectKind::StrengthLoseTemp { stacks } => {
            PyEffect::StrengthLoseTemp(PyEffectStrengthLoseTemp {
                stacks,
                target: require_target(target),
            })
        }
        EffectKind::MausoleumOpen => PyEffect::MausoleumOpen(PyEffectMausoleumOpen),
        EffectKind::KnowingSkullCostBump => {
            PyEffect::KnowingSkullCostBump(PyEffectKnowingSkullCostBump)
        }
        EffectKind::JoustBet { on_owner } => PyEffect::JoustBet(PyEffectJoustBet { on_owner }),
        EffectKind::RelicGrantPool { pool } => PyEffect::RelicGrantPool(PyEffectRelicGrantPool {
            pool: pool.iter().map(|&relic_name| relic_name.into()).collect(),
        }),
        EffectKind::RewardRollLibraryCards => {
            PyEffect::RewardRollLibraryCards(PyEffectRewardRollLibraryCards)
        }
        EffectKind::DebuffsClear => PyEffect::DebuffsClear(PyEffectDebuffsClear {
            target: require_target(target),
        }),
        EffectKind::GremlinSummon => PyEffect::GremlinSummon(PyEffectGremlinSummon),
        EffectKind::HexaghostBurnIncrease { count } => {
            PyEffect::HexaghostBurnIncrease(PyEffectHexaghostBurnIncrease { count })
        }
        EffectKind::ModifierRemove { kind } => PyEffect::ModifierRemove(PyEffectModifierRemove {
            kind: kind.into(),
            target: require_target(target),
        }),
        EffectKind::MonsterEscape => PyEffect::MonsterEscape(PyEffectMonsterEscape {
            target: require_target(target),
        }),
        EffectKind::MonsterSplit { name } => PyEffect::MonsterSplit(PyEffectMonsterSplit {
            name: name.into(),
            target: require_target(target),
        }),
        EffectKind::StasisSteal => PyEffect::StasisSteal(PyEffectStasisSteal),
        other => unreachable!(
            "snapshot_effect: unexpected EffectKind on static Card effect: {:?}",
            other
        ),
    }
}
