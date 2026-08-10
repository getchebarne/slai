use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::type_hint_union;
use pyo3::type_object::PyTypeInfo;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::KnowingSkullWish;
use crate::effect::RewardSource;
use crate::effect::Target;

use super::amount::PyAmount;
use super::amount::PyDeltaSign;
use super::card::PyCardColor;
use super::card::PyCardKind;
use super::card::PyCardName;
use super::card::PyCardPile;
use super::card::PyCardRarity;
use super::card::PyCostScope;
use super::macros::flat_variants;
use super::macros::mirror_enum;
use super::modifier::PyModifierKind;
use super::monster::PyMonsterName;
use super::relic::PyRelicName;
use super::relic::PyRelicTier;
use super::target::PyTarget;

mirror_enum!(PyKnowingSkullWish from KnowingSkullWish, "KnowingSkullWish", skip_from_py_object, {
    Potion, Gold, Card,
});

// Mirrors only EffectKind variants reachable from static Card/monster defs; snapshot_effect panics on runtime-only variants
flat_variants!(PyEffect {
    DamagePhysical => PyEffectDamagePhysical as "EffectDamagePhysical" { amount: u16, target: Option<PyTarget> },
    DamagePhysicalIfPoisoned => PyEffectDamagePhysicalIfPoisoned as "EffectDamagePhysicalIfPoisoned" { amount: u16, target: Option<PyTarget> },
    HeelHookProc => PyEffectHeelHookProc as "EffectHeelHookProc" { target: Option<PyTarget> },
    EscapePlanCheck => PyEffectEscapePlanCheck as "EffectEscapePlanCheck" { block: u16, target: Option<PyTarget> },
    GlassKnifeDecay => PyEffectGlassKnifeDecay as "EffectGlassKnifeDecay" { delta: i16, target: Option<PyTarget> },
    CardSetupPick => PyEffectCardSetupPick as "EffectCardSetupPick" { free: bool, bottom: bool, target: Option<PyTarget> },
    CardNightmarePick => PyEffectCardNightmarePick as "EffectCardNightmarePick" { target: Option<PyTarget> },
    DistractionAdd => PyEffectDistractionAdd as "EffectDistractionAdd" { target: Option<PyTarget> },
    SetCostOverride => PyEffectSetCostOverride as "EffectSetCostOverride" { amount: u8, only_reduce: bool, random: bool, scope: PyCostScope, target: Option<PyTarget> },
    DamageFinisher => PyEffectDamageFinisher as "EffectDamageFinisher" { damage: u16, target: Option<PyTarget> },
    DamageFlechettes => PyEffectDamageFlechettes as "EffectDamageFlechettes" { damage: u16, target: Option<PyTarget> },
    UnloadDiscard => PyEffectUnloadDiscard as "EffectUnloadDiscard" { target: Option<PyTarget> },
    StormOfSteelProc => PyEffectStormOfSteelProc as "EffectStormOfSteelProc" { upgraded: bool, target: Option<PyTarget> },
    SneakyStrikeProc => PyEffectSneakyStrikeProc as "EffectSneakyStrikeProc" { energy: u8, target: Option<PyTarget> },
    BlockGain => PyEffectBlockGain as "EffectBlockGain" { amount: u16, target: Option<PyTarget> },
    ModifierGain => PyEffectModifierGain as "EffectModifierGain" { kind: PyModifierKind, stacks: i16, target: Option<PyTarget> },
    ModifierMultiply => PyEffectModifierMultiply as "EffectModifierMultiply" { kind: PyModifierKind, factor: u8, target: Option<PyTarget> },
    ModifierRemove => PyEffectModifierRemove as "EffectModifierRemove" { kind: PyModifierKind, target: Option<PyTarget> },
    EnergyDelta => PyEffectEnergyDelta as "EffectEnergyDelta" { sign: PyDeltaSign, amount: u16, target: Option<PyTarget> },
    CardAdd => PyEffectCardAdd as "EffectCardAdd" { card_name: String, pile: PyCardPile, count: u16, upgraded: bool, target: Option<PyTarget> },
    CardDraw => PyEffectCardDraw as "EffectCardDraw" { count: u16, target: Option<PyTarget> },
    CardDrawUpTo => PyEffectCardDrawUpTo as "EffectCardDrawUpTo" { amount: u8, target: Option<PyTarget> },
    CardDiscard => PyEffectCardDiscard as "EffectCardDiscard" { target: Option<PyTarget> },
    CardRetain => PyEffectCardRetain as "EffectCardRetain" { target: Option<PyTarget> },
    DamageMindBlast => PyEffectDamageMindBlast as "EffectDamageMindBlast" { target: Option<PyTarget> },
    ShuffleDiscardPileIntoDrawPile => PyEffectShuffleDiscardPileIntoDrawPile as "EffectShuffleDiscardPileIntoDrawPile" { target: Option<PyTarget> },
    MaxHealthDelta => PyEffectMaxHealthDelta as "EffectMaxHealthDelta" { sign: PyDeltaSign, amount: PyAmount, target: Option<PyTarget> },
    HealthDelta => PyEffectHealthDelta as "EffectHealthDelta" { sign: PyDeltaSign, amount: PyAmount, target: Option<PyTarget> },
    PotionAddRandom => PyEffectPotionAddRandom as "EffectPotionAddRandom" { limited: bool, target: Option<PyTarget> },
    PotionDiscard => PyEffectPotionDiscard as "EffectPotionDiscard" { target: Option<PyTarget> },
    RewardRollPotions => PyEffectRewardRollPotions as "EffectRewardRollPotions" { count: u8, uniform: bool, target: Option<PyTarget> },
    CardDiscoverRoll => PyEffectCardDiscoverRoll as "EffectCardDiscoverRoll" { kind: Option<PyCardKind>, color: PyCardColor, exclude: Vec<PyCardName>, count: u8, target: Option<PyTarget> },
    GoldDelta => PyEffectGoldDelta as "EffectGoldDelta" { sign: PyDeltaSign, amount: PyAmount, target: Option<PyTarget> },
    RelicGrantRandom => PyEffectRelicGrantRandom as "EffectRelicGrantRandom" { tier: Option<PyRelicTier>, target: Option<PyTarget> },
    WheelSpin => PyEffectWheelSpin as "EffectWheelSpin" { target: Option<PyTarget> },
    BonfireOffer => PyEffectBonfireOffer as "EffectBonfireOffer" { target: Option<PyTarget> },
    CardBottle => PyEffectCardBottle as "EffectCardBottle" { target: Option<PyTarget> },
    MonsterSpawn => PyEffectMonsterSpawn as "EffectMonsterSpawn" { name: PyMonsterName, target: Option<PyTarget> },
    CombatStart => PyEffectCombatStart as "EffectCombatStart" { event_gold: Option<PyAmount>, event_relic: Option<PyRelicName>, event_relic_roll: bool, event_relic_tiers: Vec<PyRelicTier>, target: Option<PyTarget> },
    AdventurerSearch => PyEffectAdventurerSearch as "EffectAdventurerSearch" { target: Option<PyTarget> },
    RelicGrantSpecific => PyEffectRelicGrantSpecific as "EffectRelicGrantSpecific" { name: PyRelicName, fallback_circlet: bool, target: Option<PyTarget> },
    EventAdvanceState => PyEffectEventAdvanceState as "EffectEventAdvanceState" { delta: i8, target: Option<PyTarget> },
    ScrapOozeReach => PyEffectScrapOozeReach as "EffectScrapOozeReach" { dmg: u16, chance: u8, advance_on_miss: bool, target: Option<PyTarget> },
    EventConsume => PyEffectEventConsume as "EffectEventConsume" { target: Option<PyTarget> },
    CardDiscoverPick => PyEffectCardDiscoverPick as "EffectCardDiscoverPick" { cost_zero: Option<PyCostScope>, pile: PyCardPile, target: Option<PyTarget> },
    CardPurge => PyEffectCardPurge as "EffectCardPurge" { target: Option<PyTarget> },
    CardUpgrade => PyEffectCardUpgrade as "EffectCardUpgrade" { target: Option<PyTarget> },
    CardDuplicate => PyEffectCardDuplicate as "EffectCardDuplicate" { target: Option<PyTarget> },
    CardTransform => PyEffectCardTransform as "EffectCardTransform" { upgraded: bool, target: Option<PyTarget> },
    CardAddRandom => PyEffectCardAddRandom as "EffectCardAddRandom" { color: PyCardColor, kind: Option<PyCardKind>, pile: PyCardPile, count: u8, cost_zero: Option<PyCostScope>, upgraded: bool, rarity: Option<PyCardRarity>, target: Option<PyTarget> },
    CardDrawIfNoAttacks => PyEffectCardDrawIfNoAttacks as "EffectCardDrawIfNoAttacks" { count: u16, target: Option<PyTarget> },
    HandOfGreedProc => PyEffectHandOfGreedProc as "EffectHandOfGreedProc" { gold: u16, target: Option<PyTarget> },
    CardExhaust => PyEffectCardExhaust as "EffectCardExhaust" { target: Option<PyTarget> },
    CardMove => PyEffectCardMove as "EffectCardMove" { pile: PyCardPile, cost_zero: Option<PyCostScope>, target: Option<PyTarget> },
    CardPlayFromDrawTop => PyEffectCardPlayFromDrawTop as "EffectCardPlayFromDrawTop" { target: Option<PyTarget> },
    Gamble => PyEffectGamble as "EffectGamble" { choose_discards: bool, discards_before: Option<u8>, target: Option<PyTarget> },
    CombatEnd => PyEffectCombatEnd as "EffectCombatEnd" { escaped_character: bool, target: Option<PyTarget> },
    RelicLose => PyEffectRelicLose as "EffectRelicLose" { name: PyRelicName, target: Option<PyTarget> },
    RewardRollNeowCards => PyEffectRewardRollNeowCards as "EffectRewardRollNeowCards" { colorless: bool, rare_only: bool, target: Option<PyTarget> },
    StrengthLoseTemp => PyEffectStrengthLoseTemp as "EffectStrengthLoseTemp" { stacks: i16, target: Option<PyTarget> },
    DamageLifesteal => PyEffectDamageLifesteal as "EffectDamageLifesteal" { amount: u16, target: Option<PyTarget> },
    MausoleumOpen => PyEffectMausoleumOpen as "EffectMausoleumOpen" { target: Option<PyTarget> },
    KnowingSkullAsk => PyEffectKnowingSkullAsk as "EffectKnowingSkullAsk" { wish: PyKnowingSkullWish, target: Option<PyTarget> },
    JoustBet => PyEffectJoustBet as "EffectJoustBet" { on_owner: bool, target: Option<PyTarget> },
    MatchGameFlip => PyEffectMatchGameFlip as "EffectMatchGameFlip" { target: Option<PyTarget> },
    RewardRollLibraryCards => PyEffectRewardRollLibraryCards as "EffectRewardRollLibraryCards" { target: Option<PyTarget> },
    RelicGrantPool => PyEffectRelicGrantPool as "EffectRelicGrantPool" { pool: Vec<PyRelicName>, target: Option<PyTarget> },
});

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
        Target::Direct(Some(_)) => panic!(
            "snapshot_effect: unexpected Direct(Some) on static Card effect: {:?}",
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
            random,
            scope,
        } => PyEffect::SetCostOverride(PyEffectSetCostOverride {
            amount,
            only_reduce,
            random,
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
        EffectKind::CardTransform { upgraded } => {
            PyEffect::CardTransform(PyEffectCardTransform { upgraded, target })
        }
        EffectKind::RelicGrantRandom { tier } => {
            PyEffect::RelicGrantRandom(PyEffectRelicGrantRandom {
                tier: tier.map(Into::into),
                target,
            })
        }
        EffectKind::RelicLose { name } => PyEffect::RelicLose(PyEffectRelicLose {
            name: name.into(),
            target,
        }),
        EffectKind::WheelSpin => PyEffect::WheelSpin(PyEffectWheelSpin { target }),
        EffectKind::BonfireOffer => PyEffect::BonfireOffer(PyEffectBonfireOffer { target }),
        EffectKind::CardBottle => PyEffect::CardBottle(PyEffectCardBottle { target }),
        EffectKind::MonsterSpawn { name } => PyEffect::MonsterSpawn(PyEffectMonsterSpawn {
            name: name.into(),
            target,
        }),
        EffectKind::CombatStart { loot } => PyEffect::CombatStart(PyEffectCombatStart {
            event_gold: loot.gold.map(Into::into),
            event_relic: loot.relic.map(Into::into),
            event_relic_roll: loot.relic_roll,
            event_relic_tiers: loot
                .relic_tiers
                .iter()
                .flatten()
                .map(|&t| t.into())
                .collect(),
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
        EffectKind::RewardRoll {
            source: RewardSource::Potions { count, uniform },
        } => PyEffect::RewardRollPotions(PyEffectRewardRollPotions {
            count,
            uniform,
            target,
        }),
        EffectKind::RewardRoll {
            source:
                RewardSource::NeowCards {
                    colorless,
                    rare_only,
                },
        } => PyEffect::RewardRollNeowCards(PyEffectRewardRollNeowCards {
            colorless,
            rare_only,
            target,
        }),
        EffectKind::CardDiscoverRoll {
            kind,
            color,
            exclude,
            count,
        } => PyEffect::CardDiscoverRoll(PyEffectCardDiscoverRoll {
            kind: kind.map(|k| k.into()),
            color: color.into(),
            exclude: exclude.iter().map(|&n| n.into()).collect(),
            count,
            target,
        }),
        EffectKind::CardUpgrade => PyEffect::CardUpgrade(PyEffectCardUpgrade { target }),
        EffectKind::CardDiscoverPick { cost_zero, pile } => {
            PyEffect::CardDiscoverPick(PyEffectCardDiscoverPick {
                pile: pile.into(),
                cost_zero: cost_zero.map(|c| c.into()),
                target,
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
            kind: kind.map(|k| k.into()),
            pile: pile.into(),
            count,
            cost_zero: cost_zero.map(|c| c.into()),
            upgraded,
            rarity: rarity.map(Into::into),
            target,
        }),
        EffectKind::CardDrawIfNoAttacks { count } => {
            PyEffect::CardDrawIfNoAttacks(PyEffectCardDrawIfNoAttacks { count, target })
        }
        EffectKind::HandOfGreedProc { gold } => {
            PyEffect::HandOfGreedProc(PyEffectHandOfGreedProc { gold, target })
        }
        EffectKind::CardExhaust => PyEffect::CardExhaust(PyEffectCardExhaust { target }),
        EffectKind::CardMove { pile, cost_zero } => PyEffect::CardMove(PyEffectCardMove {
            pile: pile.into(),
            cost_zero: cost_zero.map(|c| c.into()),
            target,
        }),
        EffectKind::CardPlayFromDrawTop => {
            PyEffect::CardPlayFromDrawTop(PyEffectCardPlayFromDrawTop { target })
        }
        EffectKind::Gamble {
            choose_discards,
            discards_before,
        } => PyEffect::Gamble(PyEffectGamble {
            choose_discards,
            discards_before,
            target,
        }),
        EffectKind::CombatEnd { escaped_character } => PyEffect::CombatEnd(PyEffectCombatEnd {
            escaped_character,
            target,
        }),
        EffectKind::StrengthLoseTemp { stacks } => {
            PyEffect::StrengthLoseTemp(PyEffectStrengthLoseTemp { stacks, target })
        }
        EffectKind::DamageLifesteal { amount } => {
            PyEffect::DamageLifesteal(PyEffectDamageLifesteal { amount, target })
        }
        EffectKind::MausoleumOpen => PyEffect::MausoleumOpen(PyEffectMausoleumOpen { target }),
        EffectKind::KnowingSkullAsk { wish } => {
            PyEffect::KnowingSkullAsk(PyEffectKnowingSkullAsk {
                wish: wish.into(),
                target,
            })
        }
        EffectKind::JoustBet { on_owner } => {
            PyEffect::JoustBet(PyEffectJoustBet { on_owner, target })
        }
        EffectKind::RelicGrantPool { pool } => PyEffect::RelicGrantPool(PyEffectRelicGrantPool {
            pool: pool.iter().map(|&n| n.into()).collect(),
            target,
        }),
        EffectKind::MatchGameFlip => PyEffect::MatchGameFlip(PyEffectMatchGameFlip { target }),
        EffectKind::RewardRoll {
            source: RewardSource::LibraryCards,
        } => PyEffect::RewardRollLibraryCards(PyEffectRewardRollLibraryCards { target }),
        other => unreachable!(
            "snapshot_effect: unexpected EffectKind on static Card effect: {:?}",
            other
        ),
    }
}
