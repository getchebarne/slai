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
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::CostScope;
use crate::types::Mode;
use crate::utils::mode_top;
use crate::utils::scale_attack_damage;
use crate::utils::scale_block_gain;
use crate::utils::vuln_factor;
use crate::utils::weak_factor;

use super::effect::PyEffect;
use super::effect::PyEffectBlockGain;
use super::effect::PyEffectDamagePhysical;
use super::effect::snapshot_effect;
use super::macros::flat_variants;
use super::macros::mirror_enum;

mirror_enum!(PyCardKind from CardKind, "CardKind", skip_from_py_object, {
    Attack, Skill, Power, Curse, Status,
});

mirror_enum!(PyCardColor from CardColor, "CardColor", skip_from_py_object, {
    Green, Colorless, Curse,
});

mirror_enum!(PyCardRarity from CardRarity, "CardRarity", skip_from_py_object, {
    Basic, Common, Uncommon, Rare, Special, Curse,
});

flat_variants!(PyCardCostKind {
    Fixed => PyCardCostKindFixed as "CardCostKindFixed",
    MinusDiscardsThisTurn => PyCardCostKindMinusDiscardsThisTurn as "CardCostKindMinusDiscardsThisTurn",
    GrowsOnDamageInstanceTaken => PyCardCostKindGrowsOnDamageInstanceTaken as "CardCostKindGrowsOnDamageInstanceTaken",
    XCost => PyCardCostKindXCost as "CardCostKindXCost" { offset: i8 },
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

mirror_enum!(PyPlayRestriction from PlayRestriction, "PlayRestriction", skip_from_py_object, {
    Always, Never, DrawPileEmpty,
});

mirror_enum!(PyCardPile from CardPile, "CardPile", skip_from_py_object, {
    Hand, Draw, Discard, Deck,
});

mirror_enum!(PyCostScope from CostScope, "CostScope", skip_from_py_object, {
    Turn, Combat, UntilPlayed,
});

mirror_enum!(PyCardName from CardName, "CardName", skip_from_py_object, {
    AThousandCuts, Accuracy, Acrobatics, Adrenaline, AfterImage, Alchemize, AllOutAttack,
    Backflip, Backstab, BandageUp, Bane, BladeDance, Blind, Blur, BouncingFlask, BulletTime,
    Burn, Burst, CalculatedGamble, Caltrops, Catalyst, Choke, CloakAndDagger, Concentrate,
    CorpseExplosion, CripplingPoison, DaggerSpray, DaggerThrow, Dash, Dazed, DeadlyPoison,
    DeepBreath, Defend, Deflect, DieDieDie, Distraction, DodgeAndRoll, Doppelganger,
    EndlessAgony, Envenom, EscapePlan, Eviscerate, Expertise, Finesse, Finisher, FlashOfSteel,
    Flechettes, FlyingKnee, Footwork, GlassKnife, GoodInstincts, GrandFinale, HeelHook,
    InfiniteBlades, LegSweep, Malaise, MasterOfStrategy, MasterfulStab, MindBlast, Neutralize,
    Nightmare, NoxiousFumes, Outmaneuver, PhantasmalKiller, PiercingWail, PoisonedStab,
    Predator, Prepared, QuickSlash, Reflex, RiddleWithHoles, Setup, Shiv, Skewer, Slice,
    Slimed, SneakyStrike, StormOfSteel, Strike, SuckerPunch, Survivor, SwiftStrike, Tactician,
    Terror, ToolsOfTheTrade, Unload, WellLaidPlans, WraithForm, AscendersBane, Regret, Pain,
    Doubt, Decay, Injury, Shame, Writhe, Parasite, Normality, Apparition, Bite, DarkShackles,
    DramaticEntrance, Jax, Panacea, Trip, Apotheosis, Chrysalis, Discovery, Enlightenment,
    HandOfGreed, Impatience, JackOfAllTrades, Madness, Magnetism, Metamorphosis, Panache,
    PanicButton, SadisticNature, ThinkingAhead, Transmutation, Forethought, Mayhem, Purity,
    SecretTechnique, SecretWeapon, TheBomb, Violence, CurseOfTheBell, Wound,
});

// Exposed structs
#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "Card",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCard {
    pub name: PyCardName,
    pub display_name: String,

    // Cost-related fields
    pub cost: u8,
    pub cost_base: u8,
    pub cost_override: Option<u8>,
    pub cost_override_scope: Option<PyCostScope>,
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
    pub bottled: bool,
    pub requires_target: bool,
    pub retain: bool,
    // `playable` does NOT factor in energy cost; clients must also check `cost <= energy.energy_current`
    pub playable: bool,

    // Effects. Snapshot copy: DamagePhysical / BlockGain amounts carry the current player-modifier
    // adjustment (Str/Vigor/Weak/DoubleDamage, Dex/Frail), target-agnostic, so clients read finished
    // combat values. This makes hash(card) (which hashes effects) vary with combat modifiers.
    pub effects: Vec<PyEffect>,
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
            Self::CurseOfTheBell => "Curse of the Bell",
            Self::Wound => "Wound",
        }
    }
}

// Snapshot a Card's effects with the current player modifiers folded into the DamagePhysical /
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
            PyEffect::DamagePhysical(PyEffectDamagePhysical {
                amount,
                lifesteal,
                target,
            }) => {
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
                PyEffect::DamagePhysical(PyEffectDamagePhysical {
                    amount: d,
                    lifesteal,
                    target,
                })
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
    // Combat-only; outside combat defaults are permissive (Cards not played)
    let (restriction_ok, this_turn_discards, this_combat_damage, energy_current) =
        if let Mode::Combat {
            id_pile_draw,
            energy,
            this_turn_discards,
            this_combat_damage_instances_taken,
            ..
        } = mode_top(&state.mode_stack)
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
    let py_card = PyCard {
        name: card.card_name.into(),
        display_name,
        cost: get_card_effective_cost(card, this_turn_discards, this_combat_damage, energy_current),
        cost_base: card.card_cost,
        cost_override: card.card_cost_override.map(|o| o.amount),
        cost_override_scope: card.card_cost_override.map(|o| o.scope.into()),
        cost_kind: card.card_cost_kind.into(),
        kind: card.card_kind.into(),
        color: card.card_color.into(),
        rarity: card.card_rarity.into(),
        play_restriction: card.card_play_restriction.into(),
        upgraded: card.card_upgraded,
        exhaust: card.card_exhaust,
        ethereal: card.card_ethereal,
        innate: card.card_innate,
        bottled: card.card_bottled,
        requires_target: card.requires_target,
        retain: card.card_retain,
        playable: restriction_ok && !entangled_blocks,
        effects: snapshot_adjusted_effects(card, &state.entities[state.id_character].modifiers),
    };
    py_card
}
