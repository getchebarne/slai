use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::type_hint_union;
use pyo3::type_object::PyTypeInfo;

use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::entity::get_card_effective_cost;
use crate::entity::is_play_restriction_satisfied;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::Mode;
use crate::utils::scale_attack_damage;
use crate::utils::scale_block_gain;
use crate::utils::vuln_factor;
use crate::utils::weak_factor;

use super::effect::PyEffect;
use super::effect::PyEffectBlockGain;
use super::effect::PyEffectDamagePhysical;
use super::effect::snapshot_effect;
use super::macros::variant_union;

#[pyclass(
    skip_from_py_object,
    eq,
    eq_int,
    frozen,
    name = "CardKind",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyCardKind {
    Attack,
    Skill,
    Power,
    Curse,
    Status,
}

impl From<CardKind> for PyCardKind {
    fn from(kind: CardKind) -> Self {
        match kind {
            CardKind::Attack => Self::Attack,
            CardKind::Skill => Self::Skill,
            CardKind::Power => Self::Power,
            CardKind::Curse => Self::Curse,
            CardKind::Status => Self::Status,
        }
    }
}

#[pyclass(
    skip_from_py_object,
    eq,
    eq_int,
    frozen,
    name = "CardColor",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyCardColor {
    Green,
    Colorless,
    Curse,
}

impl From<CardColor> for PyCardColor {
    fn from(color: CardColor) -> Self {
        match color {
            CardColor::Green => Self::Green,
            CardColor::Colorless => Self::Colorless,
            CardColor::Curse => Self::Curse,
        }
    }
}

#[pyclass(
    skip_from_py_object,
    eq,
    eq_int,
    frozen,
    name = "CardRarity",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyCardRarity {
    Basic,
    Common,
    Uncommon,
    Rare,
    Special,
    Curse,
}

impl From<CardRarity> for PyCardRarity {
    fn from(rarity: CardRarity) -> Self {
        match rarity {
            CardRarity::Basic => Self::Basic,
            CardRarity::Common => Self::Common,
            CardRarity::Uncommon => Self::Uncommon,
            CardRarity::Rare => Self::Rare,
            CardRarity::Special => Self::Special,
            CardRarity::Curse => Self::Curse,
        }
    }
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    name = "CardCostKindFixed",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCardCostKindFixed;

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    name = "CardCostKindMinusDiscardsThisTurn",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCardCostKindMinusDiscardsThisTurn;

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    name = "CardCostKindGrowsOnDamageInstanceTaken",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCardCostKindGrowsOnDamageInstanceTaken;

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "CardCostKindXCost",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCardCostKindXCost {
    pub offset: i8,
}

// NB: variant order and field order must stay byte-identical to the internal
// enum — card_identity_hash feeds this through derived Hash
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyCardCostKind {
    Fixed(PyCardCostKindFixed),
    MinusDiscardsThisTurn(PyCardCostKindMinusDiscardsThisTurn),
    GrowsOnDamageInstanceTaken(PyCardCostKindGrowsOnDamageInstanceTaken),
    XCost(PyCardCostKindXCost),
}

variant_union!(PyCardCostKind {
    Fixed => PyCardCostKindFixed,
    MinusDiscardsThisTurn => PyCardCostKindMinusDiscardsThisTurn,
    GrowsOnDamageInstanceTaken => PyCardCostKindGrowsOnDamageInstanceTaken,
    XCost => PyCardCostKindXCost,
});

impl From<CardCostKind> for PyCardCostKind {
    fn from(kind: CardCostKind) -> Self {
        match kind {
            CardCostKind::Fixed => Self::Fixed(PyCardCostKindFixed),
            CardCostKind::MinusDiscardsThisTurn => {
                Self::MinusDiscardsThisTurn(PyCardCostKindMinusDiscardsThisTurn)
            }
            CardCostKind::GrowsOnDamageInstanceTaken => {
                Self::GrowsOnDamageInstanceTaken(PyCardCostKindGrowsOnDamageInstanceTaken)
            }
            CardCostKind::XCost { offset } => Self::XCost(PyCardCostKindXCost { offset }),
        }
    }
}

#[pyclass(
    skip_from_py_object,
    eq,
    eq_int,
    frozen,
    name = "PlayRestriction",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyPlayRestriction {
    Always,
    Never,
    DrawPileEmpty,
}

impl From<PlayRestriction> for PyPlayRestriction {
    fn from(restriction: PlayRestriction) -> Self {
        match restriction {
            PlayRestriction::Always => Self::Always,
            PlayRestriction::Never => Self::Never,
            PlayRestriction::DrawPileEmpty => Self::DrawPileEmpty,
        }
    }
}

#[pyclass(
    skip_from_py_object,
    eq,
    eq_int,
    frozen,
    name = "CardName",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyCardName {
    AThousandCuts,
    Accuracy,
    Acrobatics,
    Adrenaline,
    AfterImage,
    Alchemize,
    AllOutAttack,
    Backflip,
    Backstab,
    BandageUp,
    Bane,
    BladeDance,
    Blind,
    Blur,
    BouncingFlask,
    BulletTime,
    Burn,
    Burst,
    CalculatedGamble,
    Caltrops,
    Catalyst,
    Choke,
    CloakAndDagger,
    Concentrate,
    CorpseExplosion,
    CripplingPoison,
    DaggerSpray,
    DaggerThrow,
    Dash,
    Dazed,
    DeadlyPoison,
    DeepBreath,
    Defend,
    Deflect,
    DieDieDie,
    Distraction,
    DodgeAndRoll,
    Doppelganger,
    EndlessAgony,
    Envenom,
    EscapePlan,
    Eviscerate,
    Expertise,
    Finesse,
    Finisher,
    FlashOfSteel,
    Flechettes,
    FlyingKnee,
    Footwork,
    GlassKnife,
    GoodInstincts,
    GrandFinale,
    HeelHook,
    InfiniteBlades,
    LegSweep,
    Malaise,
    MasterOfStrategy,
    MasterfulStab,
    MindBlast,
    Neutralize,
    Nightmare,
    NoxiousFumes,
    Outmaneuver,
    PhantasmalKiller,
    PiercingWail,
    PoisonedStab,
    Predator,
    Prepared,
    QuickSlash,
    Reflex,
    RiddleWithHoles,
    Setup,
    Shiv,
    Skewer,
    Slice,
    Slimed,
    SneakyStrike,
    StormOfSteel,
    Strike,
    SuckerPunch,
    Survivor,
    SwiftStrike,
    Tactician,
    Terror,
    ToolsOfTheTrade,
    Unload,
    WellLaidPlans,
    WraithForm,
    AscendersBane,
    Regret,
    Pain,
    Doubt,
    Decay,
    Injury,
    Shame,
    Writhe,
    Parasite,
    Normality,
    Apparition,
    Bite,
    DarkShackles,
    DramaticEntrance,
    Jax,
    Panacea,
    Trip,
    Apotheosis,
    Chrysalis,
    Discovery,
    Enlightenment,
    HandOfGreed,
    Impatience,
    JackOfAllTrades,
    Madness,
    Magnetism,
    Metamorphosis,
    Panache,
    PanicButton,
    SadisticNature,
    ThinkingAhead,
    Transmutation,
    Forethought,
    Mayhem,
    Purity,
    SecretTechnique,
    SecretWeapon,
    TheBomb,
    Violence,
}

impl From<CardName> for PyCardName {
    // 1:1 by name; explicit match (not transmute) catches drift if either enum changes
    fn from(name: CardName) -> Self {
        match name {
            CardName::AThousandCuts => Self::AThousandCuts,
            CardName::Accuracy => Self::Accuracy,
            CardName::Acrobatics => Self::Acrobatics,
            CardName::Adrenaline => Self::Adrenaline,
            CardName::AfterImage => Self::AfterImage,
            CardName::Alchemize => Self::Alchemize,
            CardName::AllOutAttack => Self::AllOutAttack,
            CardName::Backflip => Self::Backflip,
            CardName::Backstab => Self::Backstab,
            CardName::BandageUp => Self::BandageUp,
            CardName::Bane => Self::Bane,
            CardName::BladeDance => Self::BladeDance,
            CardName::Blind => Self::Blind,
            CardName::Blur => Self::Blur,
            CardName::BouncingFlask => Self::BouncingFlask,
            CardName::BulletTime => Self::BulletTime,
            CardName::Burn => Self::Burn,
            CardName::Burst => Self::Burst,
            CardName::CalculatedGamble => Self::CalculatedGamble,
            CardName::Caltrops => Self::Caltrops,
            CardName::Catalyst => Self::Catalyst,
            CardName::Choke => Self::Choke,
            CardName::CloakAndDagger => Self::CloakAndDagger,
            CardName::Concentrate => Self::Concentrate,
            CardName::CorpseExplosion => Self::CorpseExplosion,
            CardName::CripplingPoison => Self::CripplingPoison,
            CardName::DaggerSpray => Self::DaggerSpray,
            CardName::DaggerThrow => Self::DaggerThrow,
            CardName::Dash => Self::Dash,
            CardName::Dazed => Self::Dazed,
            CardName::DeadlyPoison => Self::DeadlyPoison,
            CardName::DeepBreath => Self::DeepBreath,
            CardName::Defend => Self::Defend,
            CardName::Deflect => Self::Deflect,
            CardName::DieDieDie => Self::DieDieDie,
            CardName::Distraction => Self::Distraction,
            CardName::DodgeAndRoll => Self::DodgeAndRoll,
            CardName::Doppelganger => Self::Doppelganger,
            CardName::EndlessAgony => Self::EndlessAgony,
            CardName::Envenom => Self::Envenom,
            CardName::EscapePlan => Self::EscapePlan,
            CardName::Eviscerate => Self::Eviscerate,
            CardName::Expertise => Self::Expertise,
            CardName::Finesse => Self::Finesse,
            CardName::Finisher => Self::Finisher,
            CardName::FlashOfSteel => Self::FlashOfSteel,
            CardName::Flechettes => Self::Flechettes,
            CardName::FlyingKnee => Self::FlyingKnee,
            CardName::Footwork => Self::Footwork,
            CardName::GlassKnife => Self::GlassKnife,
            CardName::GoodInstincts => Self::GoodInstincts,
            CardName::GrandFinale => Self::GrandFinale,
            CardName::HeelHook => Self::HeelHook,
            CardName::InfiniteBlades => Self::InfiniteBlades,
            CardName::LegSweep => Self::LegSweep,
            CardName::Malaise => Self::Malaise,
            CardName::MasterOfStrategy => Self::MasterOfStrategy,
            CardName::MasterfulStab => Self::MasterfulStab,
            CardName::MindBlast => Self::MindBlast,
            CardName::Neutralize => Self::Neutralize,
            CardName::Nightmare => Self::Nightmare,
            CardName::NoxiousFumes => Self::NoxiousFumes,
            CardName::Outmaneuver => Self::Outmaneuver,
            CardName::PhantasmalKiller => Self::PhantasmalKiller,
            CardName::PiercingWail => Self::PiercingWail,
            CardName::PoisonedStab => Self::PoisonedStab,
            CardName::Predator => Self::Predator,
            CardName::Prepared => Self::Prepared,
            CardName::QuickSlash => Self::QuickSlash,
            CardName::Reflex => Self::Reflex,
            CardName::RiddleWithHoles => Self::RiddleWithHoles,
            CardName::Setup => Self::Setup,
            CardName::Shiv => Self::Shiv,
            CardName::Skewer => Self::Skewer,
            CardName::Slice => Self::Slice,
            CardName::Slimed => Self::Slimed,
            CardName::SneakyStrike => Self::SneakyStrike,
            CardName::StormOfSteel => Self::StormOfSteel,
            CardName::Strike => Self::Strike,
            CardName::SuckerPunch => Self::SuckerPunch,
            CardName::Survivor => Self::Survivor,
            CardName::SwiftStrike => Self::SwiftStrike,
            CardName::Tactician => Self::Tactician,
            CardName::Terror => Self::Terror,
            CardName::ToolsOfTheTrade => Self::ToolsOfTheTrade,
            CardName::Unload => Self::Unload,
            CardName::WellLaidPlans => Self::WellLaidPlans,
            CardName::WraithForm => Self::WraithForm,
            CardName::AscendersBane => Self::AscendersBane,
            CardName::Regret => Self::Regret,
            CardName::Pain => Self::Pain,
            CardName::Doubt => Self::Doubt,
            CardName::Decay => Self::Decay,
            CardName::Injury => Self::Injury,
            CardName::Shame => Self::Shame,
            CardName::Writhe => Self::Writhe,
            CardName::Parasite => Self::Parasite,
            CardName::Normality => Self::Normality,
            CardName::Apparition => Self::Apparition,
            CardName::Bite => Self::Bite,
            CardName::DarkShackles => Self::DarkShackles,
            CardName::DramaticEntrance => Self::DramaticEntrance,
            CardName::Jax => Self::Jax,
            CardName::Panacea => Self::Panacea,
            CardName::Trip => Self::Trip,
            CardName::Apotheosis => Self::Apotheosis,
            CardName::Chrysalis => Self::Chrysalis,
            CardName::Discovery => Self::Discovery,
            CardName::Enlightenment => Self::Enlightenment,
            CardName::HandOfGreed => Self::HandOfGreed,
            CardName::Impatience => Self::Impatience,
            CardName::JackOfAllTrades => Self::JackOfAllTrades,
            CardName::Madness => Self::Madness,
            CardName::Magnetism => Self::Magnetism,
            CardName::Metamorphosis => Self::Metamorphosis,
            CardName::Panache => Self::Panache,
            CardName::PanicButton => Self::PanicButton,
            CardName::SadisticNature => Self::SadisticNature,
            CardName::ThinkingAhead => Self::ThinkingAhead,
            CardName::Transmutation => Self::Transmutation,
            CardName::Forethought => Self::Forethought,
            CardName::Mayhem => Self::Mayhem,
            CardName::Purity => Self::Purity,
            CardName::SecretTechnique => Self::SecretTechnique,
            CardName::SecretWeapon => Self::SecretWeapon,
            CardName::TheBomb => Self::TheBomb,
            CardName::Violence => Self::Violence,
        }
    }
}

// Exposed structs
#[pyclass(
    skip_from_py_object,
    frozen,
    get_all,
    name = "Card",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyCard {
    pub name: PyCardName,
    pub display_name: String,

    // Cost-related fields
    pub cost: u8,
    pub cost_base: u8,
    pub cost_zero_once: bool,
    pub cost_override: Option<u8>,
    pub cost_kind: PyCardCostKind,

    // Categorical fields
    pub kind: PyCardKind,
    pub color: PyCardColor,
    pub rarity: PyCardRarity,
    pub play_restriction: PyPlayRestriction,

    // Other boolean fields
    pub upgraded: bool,
    pub exhaust: bool,
    pub ethereal: bool,
    pub innate: bool,
    pub requires_target: bool,
    pub retain: bool,
    // `playable` does NOT factor in energy cost; clients must also check `cost <= energy.energy_current`
    pub playable: bool,

    // Effects. Snapshot copy: DamagePhysical / BlockGain amounts carry the current player-modifier
    // adjustment (Str/Vigor/Weak/DoubleDamage, Dex/Frail), target-agnostic, so clients read finished
    // combat values. This makes identity_hash (which hashes effects) vary with combat modifiers.
    pub effects: Vec<PyEffect>,

    // Fingerprint over every snapshot field above except display_name (derived from
    // name+upgraded): one u64 getter replaces a per-field FFI walk for clients that
    // key caches/dedup on card identity. Deterministic across processes.
    pub identity_hash: u64,
}

// Display-name lookups
impl CardName {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AThousandCuts => "A Thousand Cuts",
            Self::Accuracy => "Accuracy",
            Self::Acrobatics => "Acrobatics",
            Self::Adrenaline => "Adrenaline",
            Self::AfterImage => "After Image",
            Self::Alchemize => "Alchemize",
            Self::AllOutAttack => "All Out Attack",
            Self::Backflip => "Backflip",
            Self::Backstab => "Backstab",
            Self::BandageUp => "Bandage Up",
            Self::Bane => "Bane",
            Self::BladeDance => "Blade Dance",
            Self::Blind => "Blind",
            Self::Blur => "Blur",
            Self::BouncingFlask => "Bouncing Flask",
            Self::BulletTime => "Bullet Time",
            Self::Burn => "Burn",
            Self::Burst => "Burst",
            Self::CalculatedGamble => "Calculated Gamble",
            Self::Caltrops => "Caltrops",
            Self::Catalyst => "Catalyst",
            Self::Choke => "Choke",
            Self::CloakAndDagger => "Cloak And Dagger",
            Self::Concentrate => "Concentrate",
            Self::CorpseExplosion => "Corpse Explosion",
            Self::CripplingPoison => "Crippling Poison",
            Self::DaggerSpray => "Dagger Spray",
            Self::DaggerThrow => "Dagger Throw",
            Self::Dash => "Dash",
            Self::Dazed => "Dazed",
            Self::DeadlyPoison => "Deadly Poison",
            Self::DeepBreath => "Deep Breath",
            Self::Defend => "Defend",
            Self::Deflect => "Deflect",
            Self::DieDieDie => "Die Die Die",
            Self::Distraction => "Distraction",
            Self::DodgeAndRoll => "Dodge And Roll",
            Self::Doppelganger => "Doppelganger",
            Self::EndlessAgony => "Endless Agony",
            Self::Envenom => "Envenom",
            Self::EscapePlan => "Escape Plan",
            Self::Eviscerate => "Eviscerate",
            Self::Expertise => "Expertise",
            Self::Finesse => "Finesse",
            Self::Finisher => "Finisher",
            Self::FlashOfSteel => "Flash Of Steel",
            Self::Flechettes => "Flechettes",
            Self::FlyingKnee => "Flying Knee",
            Self::Footwork => "Footwork",
            Self::GlassKnife => "Glass Knife",
            Self::GoodInstincts => "Good Instincts",
            Self::GrandFinale => "Grand Finale",
            Self::HeelHook => "Heel Hook",
            Self::InfiniteBlades => "Infinite Blades",
            Self::LegSweep => "Leg Sweep",
            Self::Malaise => "Malaise",
            Self::MasterOfStrategy => "Master Of Strategy",
            Self::MasterfulStab => "Masterful Stab",
            Self::MindBlast => "Mind Blast",
            Self::Neutralize => "Neutralize",
            Self::Nightmare => "Nightmare",
            Self::NoxiousFumes => "Noxious Fumes",
            Self::Outmaneuver => "Outmaneuver",
            Self::PhantasmalKiller => "Phantasmal Killer",
            Self::PiercingWail => "Piercing Wail",
            Self::PoisonedStab => "Poisoned Stab",
            Self::Predator => "Predator",
            Self::Prepared => "Prepared",
            Self::QuickSlash => "Quick Slash",
            Self::Reflex => "Reflex",
            Self::RiddleWithHoles => "Riddle With Holes",
            Self::Setup => "Setup",
            Self::Shiv => "Shiv",
            Self::Skewer => "Skewer",
            Self::Slice => "Slice",
            Self::Slimed => "Slimed",
            Self::SneakyStrike => "Sneaky Strike",
            Self::StormOfSteel => "Storm Of Steel",
            Self::Strike => "Strike",
            Self::SuckerPunch => "Sucker Punch",
            Self::Survivor => "Survivor",
            Self::SwiftStrike => "Swift Strike",
            Self::Tactician => "Tactician",
            Self::Terror => "Terror",
            Self::ToolsOfTheTrade => "Tools Of The Trade",
            Self::Unload => "Unload",
            Self::WellLaidPlans => "Well Laid Plans",
            Self::WraithForm => "Wraith Form",
            Self::AscendersBane => "Ascender's Bane",
            Self::Regret => "Regret",
            Self::Pain => "Pain",
            Self::Doubt => "Doubt",
            Self::Decay => "Decay",
            Self::Injury => "Injury",
            Self::Shame => "Shame",
            Self::Writhe => "Writhe",
            Self::Parasite => "Parasite",
            Self::Normality => "Normality",
            Self::Apparition => "Apparition",
            Self::Bite => "Bite",
            Self::DarkShackles => "Dark Shackles",
            Self::DramaticEntrance => "Dramatic Entrance",
            Self::Jax => "J.A.X.",
            Self::Panacea => "Panacea",
            Self::Trip => "Trip",
            Self::Apotheosis => "Apotheosis",
            Self::Chrysalis => "Chrysalis",
            Self::Discovery => "Discovery",
            Self::Enlightenment => "Enlightenment",
            Self::HandOfGreed => "Hand of Greed",
            Self::Impatience => "Impatience",
            Self::JackOfAllTrades => "Jack of All Trades",
            Self::Madness => "Madness",
            Self::Magnetism => "Magnetism",
            Self::Metamorphosis => "Metamorphosis",
            Self::Panache => "Panache",
            Self::PanicButton => "Panic Button",
            Self::SadisticNature => "Sadistic Nature",
            Self::ThinkingAhead => "Thinking Ahead",
            Self::Transmutation => "Transmutation",
            Self::Forethought => "Forethought",
            Self::Mayhem => "Mayhem",
            Self::Purity => "Purity",
            Self::SecretTechnique => "Secret Technique",
            Self::SecretWeapon => "Secret Weapon",
            Self::TheBomb => "The Bomb",
            Self::Violence => "Violence",
        }
    }
}

// Snapshot a card's effects with the current player modifiers folded into the DamagePhysical /
// BlockGain amounts (target-agnostic — Vulnerable/Intangible depend on the L3 target chosen later),
// via the same scaling utils as the live pipeline. Other effect kinds pass through unchanged.
pub(crate) fn snapshot_adjusted_effects(card: &Entity, char_mods: &Modifiers) -> Vec<PyEffect> {
    let vigor = if has_modifier(char_mods, ModifierKind::Vigor) {
        modifier_stacks(char_mods, ModifierKind::Vigor).max(0) as u16
    } else {
        0
    };
    let str_stacks = if has_modifier(char_mods, ModifierKind::Strength) {
        modifier_stacks(char_mods, ModifierKind::Strength)
    } else {
        0
    };
    let weak = has_modifier(char_mods, ModifierKind::Weak);
    let double = has_modifier(char_mods, ModifierKind::DoubleDamage);
    let dex = if has_modifier(char_mods, ModifierKind::Dexterity) {
        modifier_stacks(char_mods, ModifierKind::Dexterity)
    } else {
        0
    };
    let frail = has_modifier(char_mods, ModifierKind::Frail);

    card.card_effects[..card.card_effects_len as usize]
        .iter()
        .map(snapshot_effect)
        .map(|effect| match effect {
            PyEffect::DamagePhysical(PyEffectDamagePhysical { amount, target }) => {
                // Player attacker: Paper Krane never applies
                let mut d = scale_attack_damage(
                    amount.saturating_add(vigor),
                    str_stacks,
                    weak_factor(weak, false),
                    vuln_factor(false, false),
                );
                if double {
                    d = d.saturating_mul(2);
                }
                PyEffect::DamagePhysical(PyEffectDamagePhysical { amount: d, target })
            }
            PyEffect::BlockGain(PyEffectBlockGain { amount, target }) => {
                PyEffect::BlockGain(PyEffectBlockGain {
                    amount: scale_block_gain(amount, dex, frail),
                    target,
                })
            }
            other => other,
        })
        .collect()
}

pub(crate) fn snapshot_card(state: &GameState, id_card: usize) -> PyCard {
    let card = &state.entities[id_card];
    let entangled = has_modifier(
        &state.entities[state.id_character].modifiers,
        ModifierKind::Entangled,
    );
    // Combat-only; outside combat defaults are permissive (cards not played)
    let (restriction_ok, this_turn_discards, this_combat_damage, energy_current) =
        if let Mode::Combat {
            id_pile_draw,
            energy,
            this_turn_discards,
            this_combat_damage_instances_taken,
            ..
        } = &state.mode
        {
            (
                is_play_restriction_satisfied(
                    card.card_play_restriction,
                    card.card_kind,
                    &id_pile_draw,
                    &state.id_relics,
                ),
                *this_turn_discards,
                *this_combat_damage_instances_taken,
                energy.energy_current,
            )
        } else {
            (true, 0, 0, 0)
        };
    let entangled_blocks = entangled && card.card_kind == CardKind::Attack;
    let base = card.card_name.as_str();
    let display_name = if card.card_upgraded {
        format!("{base}+")
    } else {
        base.to_string()
    };
    let mut py_card = PyCard {
        name: card.card_name.into(),
        display_name,
        cost: get_card_effective_cost(card, this_turn_discards, this_combat_damage, energy_current),
        cost_base: card.card_cost,
        cost_zero_once: card.card_free_to_play_once,
        cost_override: card.card_cost_override,
        cost_kind: card.card_cost_kind.into(),
        kind: card.card_kind.into(),
        color: card.card_color.into(),
        rarity: card.card_rarity.into(),
        play_restriction: card.card_play_restriction.into(),
        upgraded: card.card_upgraded,
        exhaust: card.card_exhaust,
        ethereal: card.card_ethereal,
        innate: card.card_innate,
        requires_target: card.requires_target,
        retain: card.card_retain,
        playable: restriction_ok && !entangled_blocks,
        effects: snapshot_adjusted_effects(card, &state.entities[state.id_character].modifiers),
        identity_hash: 0,
    };
    py_card.identity_hash = card_identity_hash(&py_card);
    py_card
}

// Fingerprint over the snapshot fields clients key identity on. DefaultHasher::new()
// uses fixed keys, so the value is deterministic across processes.
fn card_identity_hash(card: &PyCard) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;
    use std::hash::Hasher;
    let mut h = DefaultHasher::new();
    card.name.hash(&mut h);
    card.kind.hash(&mut h);
    card.color.hash(&mut h);
    card.rarity.hash(&mut h);
    card.cost_kind.hash(&mut h);
    card.cost.hash(&mut h);
    card.cost_base.hash(&mut h);
    card.cost_zero_once.hash(&mut h);
    card.cost_override.hash(&mut h);
    card.upgraded.hash(&mut h);
    card.exhaust.hash(&mut h);
    card.innate.hash(&mut h);
    card.ethereal.hash(&mut h);
    card.retain.hash(&mut h);
    card.requires_target.hash(&mut h);
    card.playable.hash(&mut h);
    card.effects.hash(&mut h);
    h.finish()
}
