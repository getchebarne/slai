use std::convert::Infallible;

use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use strum::IntoEnumIterator;

use crate::effect::CardPlay;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::PlaySource;
use crate::effect::RelicRoll;
use crate::effect::ScaledAt;
use crate::effect::Target;
use crate::entity::Entity;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::types::Location;
use crate::types::RelicName;
use crate::utils::entity_requires_target;
use crate::utils::has_relic;
use crate::utils::scale_attack_damage;
use crate::utils::scale_block_gain;
use crate::utils::slow_factor;
use crate::utils::strike_dummy_bonus;
use crate::utils::vuln_factor;
use crate::utils::weak_factor;
use crate::utils::wrist_blade_bonus;

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
use super::macros::mirror_enum;
use super::modifier::PyModifierKind;
use super::monster::PyMonsterName;
use super::relic::PyRelicExclusion;
use super::relic::PyRelicName;
use super::relic::PyRelicTier;
use super::target::PyTarget;
use crate::effect::RewardRollTrigger;

mirror_enum!(PyRewardRollTrigger from RewardRollTrigger, "RewardRollTrigger", {
    CallingBell, Cauldron, CombatBoss, CombatElite, CombatMonster, DreamCatcher, EventFight,
    EventFightUnpaid, Lab, Library, NeowPotions, Orrery, SensoryStone, SmokeBomb, TinyHouse,
    WomanInBlue,
});

#[pyclass(
    skip_from_py_object,
    eq,
    eq_int,
    frozen,
    name = "PlaySource",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumIter)]
pub enum PyPlaySource {
    Hand,
    DrawTop,
    Replay,
    ReplayCopy,
    TurnEnd,
}

#[pymethods]
impl PyPlaySource {
    // Declaration order, which is also int() order
    #[staticmethod]
    fn members() -> Vec<PyPlaySource> {
        PyPlaySource::iter().collect()
    }

    // Hash by raw discriminant so eq_int and hash agree; the derived hash
    // breaks Python's eq/hash contract for int-keyed dicts
    fn __hash__(&self) -> isize {
        *self as isize
    }
}

impl From<PlaySource> for PyPlaySource {
    fn from(play_source: PlaySource) -> Self {
        match play_source {
            PlaySource::Hand => Self::Hand,
            PlaySource::DrawTop => Self::DrawTop,
            PlaySource::Replay => Self::Replay,
            PlaySource::ReplayCopy { .. } => Self::ReplayCopy,
            PlaySource::TurnEnd { .. } => Self::TurnEnd,
        }
    }
}

// A Card play waiting in the card play queue; a Card queued twice has two
#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "CardPlay",
    module = "slai.slai"
)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PyCardPlay {
    pub id_card: usize,

    // Shown only when the Card needs a target, the play isn't draw-top and it's still in the roster
    pub id_target: Option<usize>,
    pub play_source: PyPlaySource,
    pub energy: u16,         // Fixed when queued; X-cost Cards read it as X
    pub cost_effective: u16, // Fixed when queued; the play's cost, which a hand play pays
    pub draw_pile_size: u16, // Fixed when queued; Mind Blast hits for it
}

pub(crate) fn snapshot_card_play(state: &GameState, card_play: &CardPlay) -> PyCardPlay {
    PyCardPlay {
        id_card: card_play.id_card,
        id_target: id_target_shown(state, card_play),
        play_source: card_play.play_source.into(),
        energy: card_play.energy,
        cost_effective: card_play.cost_effective,
        draw_pile_size: card_play.draw_pile_size,
    }
}

pub(crate) fn id_target_shown(state: &GameState, card_play: &CardPlay) -> Option<usize> {
    card_play.id_target.filter(|&id_target| {
        card_play.play_source != PlaySource::DrawTop
            && entity_requires_target(&state.entities[card_play.id_card])
            && state.combat.id_monsters.contains(&Some(id_target))
    })
}

// Mirrors only EffectKind variants reachable from static Card/Monster defs or a halted selection; snapshot_effect panics on runtime-only variants
flat_variants!(PyEffect as "Effect" {
    DamagePhysical => PyEffectDamagePhysical as "EffectDamagePhysical" { amount: u16, instances: u16, lifesteal: bool, if_poisoned: bool, block_from_hit: bool, target: PyTarget },
    HeelHookProc => PyEffectHeelHookProc as "EffectHeelHookProc" { damage: u16, target: PyTarget },
    SkewerProc => PyEffectSkewerProc as "EffectSkewerProc" { damage: u16, target: PyTarget },
    EscapePlanCheck => PyEffectEscapePlanCheck as "EffectEscapePlanCheck" { block: u16, target: PyTarget },
    GlassKnifeDecay => PyEffectGlassKnifeDecay as "EffectGlassKnifeDecay" { delta: i16, target: PyTarget },
    CardSetupSelect => PyEffectCardSetupSelect as "EffectCardSetupSelect" { free: bool, bottom: bool, target: PyTarget },
    CardNightmareSelect => PyEffectCardNightmareSelect as "EffectCardNightmareSelect" { target: PyTarget },
    DistractionAdd => PyEffectDistractionAdd as "EffectDistractionAdd",
    SetCostOverride => PyEffectSetCostOverride as "EffectSetCostOverride" { amount: u16, only_reduce: bool, random: bool, scope: PyCostScope, target: PyTarget },
    DamageFinisher => PyEffectDamageFinisher as "EffectDamageFinisher" { damage: u16, target: PyTarget },
    DamageFlechettes => PyEffectDamageFlechettes as "EffectDamageFlechettes" { damage: u16, target: PyTarget },
    UnloadDiscard => PyEffectUnloadDiscard as "EffectUnloadDiscard",
    StormOfSteelProc => PyEffectStormOfSteelProc as "EffectStormOfSteelProc" { upgraded: bool },
    SneakyStrikeProc => PyEffectSneakyStrikeProc as "EffectSneakyStrikeProc" { energy: u8 },
    BlockGain => PyEffectBlockGain as "EffectBlockGain" { amount: u16, target: PyTarget },
    BombArm => PyEffectBombArm as "EffectBombArm" { turns: u8, damage: u16 },
    LifestealHeal => PyEffectLifestealHeal as "EffectLifestealHeal" { target: PyTarget },
    ModifierDelta => PyEffectModifierDelta as "EffectModifierDelta" { kind: PyModifierKind, stacks: i16, target: PyTarget },
    ModifierMultiply => PyEffectModifierMultiply as "EffectModifierMultiply" { kind: PyModifierKind, factor: u8, target: PyTarget },
    EnergyDelta => PyEffectEnergyDelta as "EffectEnergyDelta" { sign: PyDeltaSign, amount: u16 },
    CardAdd => PyEffectCardAdd as "EffectCardAdd" { card_name: PyCardName, card_pile: PyCardPile, count: u16, upgraded: bool },
    CardDraw => PyEffectCardDraw as "EffectCardDraw" { count: u16 },
    CardDrawUpTo => PyEffectCardDrawUpTo as "EffectCardDrawUpTo" { amount: u8 },
    CardDiscard => PyEffectCardDiscard as "EffectCardDiscard" { target: PyTarget },
    CardRetain => PyEffectCardRetain as "EffectCardRetain" { target: PyTarget },
    DamageMindBlast => PyEffectDamageMindBlast as "EffectDamageMindBlast" { damage: u16, target: PyTarget },
    ShuffleCardPileDiscardIntoCardPileDraw => PyEffectShuffleCardPileDiscardIntoCardPileDraw as "EffectShuffleCardPileDiscardIntoCardPileDraw",
    MaxHealthDelta => PyEffectMaxHealthDelta as "EffectMaxHealthDelta" { sign: PyDeltaSign, amount: PyAmountScalar, target: PyTarget },
    HealthDelta => PyEffectHealthDelta as "EffectHealthDelta" { sign: PyDeltaSign, amount: PyAmountScalar, target: PyTarget },
    PotionAddRandom => PyEffectPotionAddRandom as "EffectPotionAddRandom" { limited: bool, uniform: bool },
    PotionDiscard => PyEffectPotionDiscard as "EffectPotionDiscard" { target: PyTarget },
    RewardRollPotions => PyEffectRewardRollPotions as "EffectRewardRollPotions" { count: u8, trigger: PyRewardRollTrigger },
    RewardRollGold => PyEffectRewardRollGold as "EffectRewardRollGold" { amount: PyAmount },
    CardDiscoverRoll => PyEffectCardDiscoverRoll as "EffectCardDiscoverRoll" { kind: Option<PyCardKind>, color: PyCardColor, exclude: Vec<PyCardName>, count: u8 },
    GoldDelta => PyEffectGoldDelta as "EffectGoldDelta" { sign: PyDeltaSign, amount: PyAmount },
    RelicGrantRandom => PyEffectRelicGrantRandom as "EffectRelicGrantRandom" { tier: Option<PyRelicTier>, exclusion: PyRelicExclusion },
    WheelSpin => PyEffectWheelSpin as "EffectWheelSpin",
    BonfireOffer => PyEffectBonfireOffer as "EffectBonfireOffer" { target: PyTarget },
    CardBottle => PyEffectCardBottle as "EffectCardBottle" { target: PyTarget },
    MonsterSpawn => PyEffectMonsterSpawn as "EffectMonsterSpawn" { name: PyMonsterName },
    CombatStart => PyEffectCombatStart as "EffectCombatStart",
    AdventurerSearch => PyEffectAdventurerSearch as "EffectAdventurerSearch",
    RelicGrantSpecific => PyEffectRelicGrantSpecific as "EffectRelicGrantSpecific" { name: PyRelicName },
    EventAdvanceStage => PyEffectEventAdvanceStage as "EffectEventAdvanceStage" { delta: i8 },
    ScrapOozeReach => PyEffectScrapOozeReach as "EffectScrapOozeReach" { chance: u8, advance_on_miss: bool },
    EventConsume => PyEffectEventConsume as "EffectEventConsume",
    EventClose => PyEffectEventClose as "EffectEventClose",
    CardDiscoverSelect => PyEffectCardDiscoverSelect as "EffectCardDiscoverSelect" { cost_zero: Option<PyCostScope>, card_pile: PyCardPile, copies: u8, target: PyTarget },
    CardAdopt => PyEffectCardAdopt as "EffectCardAdopt" { target: PyTarget },
    CardPileEventRollClear => PyEffectCardPileEventRollClear as "EffectCardPileEventRollClear",
    CardPurge => PyEffectCardPurge as "EffectCardPurge" { target: PyTarget },
    CardUpgrade => PyEffectCardUpgrade as "EffectCardUpgrade" { target: PyTarget },
    RestSmith => PyEffectRestSmith as "EffectRestSmith" { target: PyTarget },
    RestToke => PyEffectRestToke as "EffectRestToke" { target: PyTarget },
    CardDuplicate => PyEffectCardDuplicate as "EffectCardDuplicate" { card_pile: PyCardPile, target: PyTarget },
    CardTransform => PyEffectCardTransform as "EffectCardTransform" { upgraded: bool, target: PyTarget },
    CardAddRandom => PyEffectCardAddRandom as "EffectCardAddRandom" { color: PyCardColor, kind: Option<PyCardKind>, card_pile: PyCardPile, count: u16, cost_zero: Option<PyCostScope>, upgraded: bool, rarity: Option<PyCardRarity> },
    CardDrawIfNoAttacks => PyEffectCardDrawIfNoAttacks as "EffectCardDrawIfNoAttacks" { count: u16 },
    HandOfGreedProc => PyEffectHandOfGreedProc as "EffectHandOfGreedProc" { gold: u16, target: PyTarget },
    RitualDaggerProc => PyEffectRitualDaggerProc as "EffectRitualDaggerProc" { bump: u16, target: PyTarget },
    CardExhaust => PyEffectCardExhaust as "EffectCardExhaust" { target: PyTarget },
    CardMove => PyEffectCardMove as "EffectCardMove" { card_pile: PyCardPile, cost_zero: Option<PyCostScope>, target: PyTarget },
    CardPlayFromCardPileDrawTop => PyEffectCardPlayFromCardPileDrawTop as "EffectCardPlayFromCardPileDrawTop",
    Gamble => PyEffectGamble as "EffectGamble" { choose_discards: bool },
    CombatEnd => PyEffectCombatEnd as "EffectCombatEnd" { escaped_character: bool },
    RelicLose => PyEffectRelicLose as "EffectRelicLose" { target: PyTarget },
    StrengthLoseTemp => PyEffectStrengthLoseTemp as "EffectStrengthLoseTemp" { stacks: i16, target: PyTarget },
    MausoleumOpen => PyEffectMausoleumOpen as "EffectMausoleumOpen",
    KnowingSkullCostBump => PyEffectKnowingSkullCostBump as "EffectKnowingSkullCostBump",
    JoustBet => PyEffectJoustBet as "EffectJoustBet" { on_owner: bool },
    RelicGrantPool => PyEffectRelicGrantPool as "EffectRelicGrantPool" { pool: Vec<PyRelicName> },
    DebuffsClear => PyEffectDebuffsClear as "EffectDebuffsClear" { target: PyTarget },
    HexaghostBurnIncrease => PyEffectHexaghostBurnIncrease as "EffectHexaghostBurnIncrease" { count: u8 },
    ModifierRemove => PyEffectModifierRemove as "EffectModifierRemove" { kind: PyModifierKind, target: PyTarget },
    ModifierStrengthNegativeRemove => PyEffectModifierStrengthNegativeRemove as "EffectModifierStrengthNegativeRemove" { target: PyTarget },
    MonsterEscape => PyEffectMonsterEscape as "EffectMonsterEscape" { target: PyTarget },
    MonsterRemove => PyEffectMonsterRemove as "EffectMonsterRemove" { target: PyTarget },
    MonsterSplit => PyEffectMonsterSplit as "EffectMonsterSplit" { name: PyMonsterName, target: PyTarget },
    MonsterSummon => PyEffectMonsterSummon as "EffectMonsterSummon" { pool: Vec<PyMonsterName> },
    CardStasisSteal => PyEffectCardStasisSteal as "EffectCardStasisSteal",
    RewardRollCards => PyEffectRewardRollCards as "EffectRewardRollCards" { bundles: u8, trigger: PyRewardRollTrigger },
    NeowCardsRoll => PyEffectNeowCardsRoll as "EffectNeowCardsRoll" { colorless: bool, rare_only: bool },
    RewardRollRelic => PyEffectRewardRollRelic as "EffectRewardRollRelic" { pool: Vec<PyRelicName> },
    DamageDeal => PyEffectDamageDeal as "EffectDamageDeal" { amount: u16, lifesteal: bool, target: PyTarget },
    MatchFlipSeen => PyEffectMatchFlipSeen as "EffectMatchFlipSeen" { target: PyTarget },
    MatchFlipUnseen => PyEffectMatchFlipUnseen as "EffectMatchFlipUnseen",
    HalfDeadRebirth => PyEffectHalfDeadRebirth as "EffectHalfDeadRebirth" { target: PyTarget },
    HealthSet => PyEffectHealthSet as "EffectHealthSet" { amount: PyAmountScalar, target: PyTarget },
    RoomEnter => PyEffectRoomEnter as "EffectRoomEnter",
    PhilosopherStoneProc => PyEffectPhilosopherStoneProc as "EffectPhilosopherStoneProc" { target: PyTarget },
});

// The pending effect as its Python object, built once per snapshot; typed as PyEffect's union
#[derive(Debug)]
pub struct PyEffectPendingObject(Py<PyAny>);

impl<'py> IntoPyObject<'py> for PyEffectPendingObject {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = Infallible;
    const OUTPUT_TYPE: PyStaticExpr = <PyEffect as IntoPyObject<'py>>::OUTPUT_TYPE;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(self.0.into_bound(py))
    }
}

impl<'a, 'py> IntoPyObject<'py> for &'a PyEffectPendingObject {
    type Target = PyAny;
    type Output = Borrowed<'a, 'py, PyAny>;
    type Error = Infallible;
    const OUTPUT_TYPE: PyStaticExpr = <PyEffect as IntoPyObject<'py>>::OUTPUT_TYPE;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(self.0.bind_borrowed(py))
    }
}

pub(crate) fn snapshot_effect_pending(
    py: Python<'_>,
    effect: &Effect,
) -> PyResult<PyEffectPendingObject> {
    Ok(PyEffectPendingObject(
        snapshot_effect(effect).into_pyobject(py)?.unbind(),
    ))
}

// A running game's Effects, Snecko Skull folded in; `card_combat` scales a Combat Card's play
pub(crate) fn snapshot_effects(
    state: &GameState,
    effects: &[Effect],
    card_combat: Option<(&Entity, u16, u16)>, // (Card, card_cost, draw_pile_size)
) -> Vec<PyEffect> {
    effects
        .iter()
        .map(|effect| {
            let mut py_effect = snapshot_effect(effect);
            match (&mut py_effect, card_combat) {
                // Snecko Skull: a positive Poison lands 1 higher
                (
                    PyEffect::ModifierDelta(PyEffectModifierDelta {
                        kind: PyModifierKind::Poison,
                        stacks,
                        ..
                    }),
                    _,
                ) if *stacks > 0 && has_relic(&state.id_relics, RelicName::SneckoSkull) => {
                    *stacks += 1
                }
                (
                    PyEffect::DamagePhysical(PyEffectDamagePhysical { amount: damage, .. })
                    | PyEffect::DamageFinisher(PyEffectDamageFinisher { damage, .. })
                    | PyEffect::DamageFlechettes(PyEffectDamageFlechettes { damage, .. })
                    | PyEffect::HeelHookProc(PyEffectHeelHookProc { damage, .. })
                    | PyEffect::SkewerProc(PyEffectSkewerProc { damage, .. }),
                    Some((card, cost, _)),
                ) => *damage = adjusted_damage(state, card, cost, *damage),
                (
                    PyEffect::DamageMindBlast(PyEffectDamageMindBlast { damage, .. }),
                    Some((card, cost, draw_pile_size)),
                ) => *damage = adjusted_damage(state, card, cost, draw_pile_size),
                (
                    PyEffect::BlockGain(PyEffectBlockGain { amount: block, .. })
                    | PyEffect::EscapePlanCheck(PyEffectEscapePlanCheck { block, .. }),
                    Some(_),
                ) => *block = adjusted_block(state, *block),
                // Dodge and Roll's next-turn block scales like its block
                (
                    PyEffect::ModifierDelta(PyEffectModifierDelta {
                        kind: PyModifierKind::NextTurnBlock,
                        stacks,
                        ..
                    }),
                    Some(_),
                ) => *stacks = adjusted_block(state, (*stacks).max(0) as u16) as i16,
                _ => {}
            }
            py_effect
        })
        .collect()
}

// A Combat Card's scaled hit, the same for all its hits; Paper Krane never applies to it
fn adjusted_damage(state: &GameState, card: &Entity, cost: u16, base: u16) -> u16 {
    let mods_char = &state.entities[state.id_character].modifiers;

    // Strike Dummy and Wrist Blade join the base damage before scaling
    let bonus = strike_dummy_bonus(card.card_name, &state.id_relics)
        + wrist_blade_bonus(card, cost, &state.id_relics);
    let vigor = if has_modifier(mods_char, ModifierKind::Vigor) {
        modifier_stacks(mods_char, ModifierKind::Vigor).max(0) as u16
    } else {
        0
    };
    let str_stacks = if has_modifier(mods_char, ModifierKind::Strength) {
        modifier_stacks(mods_char, ModifierKind::Strength)
    } else {
        0
    };
    scale_attack_damage(
        base.saturating_add(bonus).saturating_add(vigor),
        str_stacks,
        has_modifier(mods_char, ModifierKind::DoubleDamage),
        has_modifier(mods_char, ModifierKind::PenNib),
        weak_factor(has_modifier(mods_char, ModifierKind::Weak), false),
        slow_factor(0),
        vuln_factor(false, false),
        false,
    ) as u16
}

fn adjusted_block(state: &GameState, base: u16) -> u16 {
    let mods_char = &state.entities[state.id_character].modifiers;
    if has_modifier(mods_char, ModifierKind::NoBlock) {
        return 0;
    }
    let dex = if has_modifier(mods_char, ModifierKind::Dexterity) {
        modifier_stacks(mods_char, ModifierKind::Dexterity)
    } else {
        0
    };
    scale_block_gain(base, dex, has_modifier(mods_char, ModifierKind::Frail))
}

pub(crate) fn snapshot_effect(effect: &Effect) -> PyEffect {
    let target = match effect.target {
        Target::Resolve {
            candidate_pool,
            filters,
            selection_kind,
        } => Some(PyTarget {
            candidate_pool: candidate_pool.into(),
            filters: filters.iter().map(|&filter| filter.into()).collect(),
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
                    | EffectKind::BombArm { .. }
                    | EffectKind::CardAdd { .. }
                    | EffectKind::CardAddRandom { .. }
                    | EffectKind::CardDiscoverRoll { .. }
                    | EffectKind::CardDraw { .. }
                    | EffectKind::CardDrawIfNoAttacks { .. }
                    | EffectKind::CardDrawUpTo { .. }
                    | EffectKind::CardPileEventRollClear
                    | EffectKind::CardPlayFromCardPileDrawTop
                    | EffectKind::CardStasisSteal
                    | EffectKind::CombatEnd { .. }
                    | EffectKind::CombatStart { .. }
                    | EffectKind::DistractionAdd
                    | EffectKind::EnergyDelta { .. }
                    | EffectKind::EventAdvanceStage { .. }
                    | EffectKind::EventClose
                    | EffectKind::EventConsume
                    | EffectKind::Gamble { .. }
                    | EffectKind::GoldDelta { .. }
                    | EffectKind::HexaghostBurnIncrease { .. }
                    | EffectKind::JoustBet { .. }
                    | EffectKind::KnowingSkullCostBump
                    | EffectKind::MausoleumOpen
                    | EffectKind::MonsterSpawn { .. }
                    | EffectKind::MonsterSummon { .. }
                    | EffectKind::NeowCardsRoll { .. }
                    | EffectKind::PotionAddRandom { .. }
                    | EffectKind::RelicGrantPool { .. }
                    | EffectKind::RelicGrantRandom { .. }
                    | EffectKind::RelicGrantSpecific { .. }
                    | EffectKind::RewardRollCards { .. }
                    | EffectKind::RewardRollGold { .. }
                    | EffectKind::RewardRollPotions { .. }
                    | EffectKind::RewardRollRelic { .. }
                    | EffectKind::RoomEnter { .. }
                    | EffectKind::ScrapOozeReach { .. }
                    | EffectKind::ShuffleCardPileDiscardIntoCardPileDraw
                    | EffectKind::SneakyStrikeProc { .. }
                    | EffectKind::StormOfSteelProc { .. }
                    | EffectKind::UnloadDiscard
                    | EffectKind::WheelSpin
            ),
            "effect kind {:?} resolved a target but its FFI row has no target field",
            effect.kind
        );
    }
    match effect.kind {
        // A hit scaled as its play started is pushed at runtime only
        EffectKind::DamagePhysical {
            amount,
            instances,
            lifesteal,
            if_poisoned,
            block_from_hit,
            scaled_at: ScaledAt::Now,
        } => PyEffect::DamagePhysical(PyEffectDamagePhysical {
            amount,
            instances,
            lifesteal,
            if_poisoned,
            block_from_hit,
            target: require_target(target),
        }),
        EffectKind::HeelHookProc { damage } => PyEffect::HeelHookProc(PyEffectHeelHookProc {
            damage,
            target: require_target(target),
        }),
        EffectKind::SkewerProc { damage } => PyEffect::SkewerProc(PyEffectSkewerProc {
            damage,
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
        EffectKind::CardSetupSelect { free, bottom } => {
            PyEffect::CardSetupSelect(PyEffectCardSetupSelect {
                free,
                bottom,
                target: require_target(target),
            })
        }
        EffectKind::CardNightmareSelect => {
            PyEffect::CardNightmareSelect(PyEffectCardNightmareSelect {
                target: require_target(target),
            })
        }
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
        EffectKind::BombArm { turns, damage } => {
            PyEffect::BombArm(PyEffectBombArm { turns, damage })
        }
        EffectKind::LifestealHeal => PyEffect::LifestealHeal(PyEffectLifestealHeal {
            target: require_target(target),
        }),
        EffectKind::BlockGain { amount } => PyEffect::BlockGain(PyEffectBlockGain {
            amount,
            target: require_target(target),
        }),
        EffectKind::ModifierDelta { kind, stacks } => {
            PyEffect::ModifierDelta(PyEffectModifierDelta {
                kind: kind.into(),
                stacks,
                target: require_target(target),
            })
        }
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
            card_pile,
            count,
            upgraded,
        } => PyEffect::CardAdd(PyEffectCardAdd {
            card_name: card_name.into(),
            card_pile: card_pile.into(),
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
        // The draw pile sets the damage in combat; the Card's own base is 0
        EffectKind::DamageMindBlast => PyEffect::DamageMindBlast(PyEffectDamageMindBlast {
            damage: 0,
            target: require_target(target),
        }),
        EffectKind::ShuffleCardPileDiscardIntoCardPileDraw => {
            PyEffect::ShuffleCardPileDiscardIntoCardPileDraw(
                PyEffectShuffleCardPileDiscardIntoCardPileDraw,
            )
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
        EffectKind::CardDuplicate { card_pile } => PyEffect::CardDuplicate(PyEffectCardDuplicate {
            card_pile: card_pile.into(),
            target: require_target(target),
        }),
        EffectKind::CardTransform { upgraded } => PyEffect::CardTransform(PyEffectCardTransform {
            upgraded,
            target: require_target(target),
        }),
        EffectKind::RelicGrantRandom { tier, exclusion } => {
            PyEffect::RelicGrantRandom(PyEffectRelicGrantRandom {
                tier: tier.map(Into::into),
                exclusion: exclusion.into(),
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
        EffectKind::CombatStart { .. } => PyEffect::CombatStart(PyEffectCombatStart),
        EffectKind::AdventurerSearch => PyEffect::AdventurerSearch(PyEffectAdventurerSearch),
        EffectKind::RelicGrantSpecific { name } => {
            PyEffect::RelicGrantSpecific(PyEffectRelicGrantSpecific { name: name.into() })
        }
        EffectKind::EventAdvanceStage { delta } => {
            PyEffect::EventAdvanceStage(PyEffectEventAdvanceStage { delta })
        }
        EffectKind::ScrapOozeReach {
            chance,
            advance_on_miss,
        } => PyEffect::ScrapOozeReach(PyEffectScrapOozeReach {
            chance,
            advance_on_miss,
        }),
        EffectKind::EventConsume => PyEffect::EventConsume(PyEffectEventConsume),
        EffectKind::EventClose => PyEffect::EventClose(PyEffectEventClose),
        EffectKind::PotionAddRandom { limited, uniform } => {
            PyEffect::PotionAddRandom(PyEffectPotionAddRandom { limited, uniform })
        }
        EffectKind::PotionDiscard => PyEffect::PotionDiscard(PyEffectPotionDiscard {
            target: require_target(target),
        }),
        EffectKind::RewardRollPotions { count, trigger } => {
            PyEffect::RewardRollPotions(PyEffectRewardRollPotions {
                count,
                trigger: trigger.into(),
            })
        }
        EffectKind::RewardRollGold { amount } => PyEffect::RewardRollGold(PyEffectRewardRollGold {
            amount: amount.into(),
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
        EffectKind::RestSmith => PyEffect::RestSmith(PyEffectRestSmith {
            target: require_target(target),
        }),
        EffectKind::RestToke => PyEffect::RestToke(PyEffectRestToke {
            target: require_target(target),
        }),
        EffectKind::CardDiscoverSelect {
            cost_zero,
            card_pile,
            copies,
        } => PyEffect::CardDiscoverSelect(PyEffectCardDiscoverSelect {
            card_pile: card_pile.into(),
            copies,
            cost_zero: cost_zero.map(|cost_scope| cost_scope.into()),
            target: require_target(target),
        }),
        // Rows and halted selections hold only the first pass; the landing pass is queued at runtime
        EffectKind::CardAdopt { landing: false } => PyEffect::CardAdopt(PyEffectCardAdopt {
            target: require_target(target),
        }),
        EffectKind::CardPileEventRollClear => {
            PyEffect::CardPileEventRollClear(PyEffectCardPileEventRollClear)
        }
        EffectKind::CardAddRandom {
            color,
            kind,
            card_pile,
            count,
            cost_zero,
            upgraded,
            rarity,
        } => PyEffect::CardAddRandom(PyEffectCardAddRandom {
            color: color.into(),
            kind: kind.map(|card_kind| card_kind.into()),
            card_pile: card_pile.into(),
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
        EffectKind::CardMove {
            card_pile,
            cost_zero,
        } => PyEffect::CardMove(PyEffectCardMove {
            card_pile: card_pile.into(),
            cost_zero: cost_zero.map(|cost_scope| cost_scope.into()),
            target: require_target(target),
        }),
        EffectKind::CardPlayFromCardPileDrawTop => {
            PyEffect::CardPlayFromCardPileDrawTop(PyEffectCardPlayFromCardPileDrawTop)
        }
        EffectKind::Gamble { choose_discards } => {
            PyEffect::Gamble(PyEffectGamble { choose_discards })
        }
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
        EffectKind::DebuffsClear => PyEffect::DebuffsClear(PyEffectDebuffsClear {
            target: require_target(target),
        }),
        EffectKind::HexaghostBurnIncrease { count } => {
            PyEffect::HexaghostBurnIncrease(PyEffectHexaghostBurnIncrease { count })
        }
        EffectKind::ModifierRemove { kind } => PyEffect::ModifierRemove(PyEffectModifierRemove {
            kind: kind.into(),
            target: require_target(target),
        }),
        EffectKind::ModifierStrengthNegativeRemove => {
            PyEffect::ModifierStrengthNegativeRemove(PyEffectModifierStrengthNegativeRemove {
                target: require_target(target),
            })
        }
        EffectKind::MonsterEscape => PyEffect::MonsterEscape(PyEffectMonsterEscape {
            target: require_target(target),
        }),
        EffectKind::MonsterRemove => PyEffect::MonsterRemove(PyEffectMonsterRemove {
            target: require_target(target),
        }),
        EffectKind::MonsterSplit { name, .. } => PyEffect::MonsterSplit(PyEffectMonsterSplit {
            name: name.into(),
            target: require_target(target),
        }),
        EffectKind::MonsterSummon { summon } => PyEffect::MonsterSummon(PyEffectMonsterSummon {
            pool: summon
                .pool
                .iter()
                .map(|&monster_name| monster_name.into())
                .collect(),
        }),
        EffectKind::CardStasisSteal => PyEffect::CardStasisSteal(PyEffectCardStasisSteal),
        EffectKind::RewardRollCards { bundles, trigger } => {
            PyEffect::RewardRollCards(PyEffectRewardRollCards {
                bundles,
                trigger: trigger.into(),
            })
        }
        EffectKind::NeowCardsRoll {
            colorless,
            rare_only,
        } => PyEffect::NeowCardsRoll(PyEffectNeowCardsRoll {
            colorless,
            rare_only,
        }),
        // Static defs stage only pool rolls (Cursed Tome's book)
        EffectKind::RewardRollRelic {
            roll: RelicRoll::Pool(pool),
            ..
        } => PyEffect::RewardRollRelic(PyEffectRewardRollRelic {
            pool: pool.iter().map(|&relic_name| relic_name.into()).collect(),
        }),
        EffectKind::DamageDeal { amount, lifesteal } => PyEffect::DamageDeal(PyEffectDamageDeal {
            amount,
            lifesteal,
            target: require_target(target),
        }),
        EffectKind::MatchFlipSeen => PyEffect::MatchFlipSeen(PyEffectMatchFlipSeen {
            target: require_target(target),
        }),
        EffectKind::MatchFlipUnseen => PyEffect::MatchFlipUnseen(PyEffectMatchFlipUnseen),
        EffectKind::HalfDeadRebirth => PyEffect::HalfDeadRebirth(PyEffectHalfDeadRebirth {
            target: require_target(target),
        }),
        EffectKind::HealthSet { amount } => PyEffect::HealthSet(PyEffectHealthSet {
            amount: amount.into(),
            target: require_target(target),
        }),
        EffectKind::RoomEnter {
            location: Location::BossRoom,
            landing: false,
        } => PyEffect::RoomEnter(PyEffectRoomEnter),
        EffectKind::PhilosopherStoneProc => {
            PyEffect::PhilosopherStoneProc(PyEffectPhilosopherStoneProc {
                target: require_target(target),
            })
        }
        other => unreachable!(
            "snapshot_effect: unexpected EffectKind on static Card effect: {:?}",
            other
        ),
    }
}
