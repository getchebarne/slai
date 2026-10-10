use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::type_hint_union;
use pyo3::type_object::PyTypeInfo;

use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::CostScope;
use crate::utils::entity_requires_target;
use crate::utils::get_card_effective_cost;
use crate::utils::is_play_restriction_satisfied;
use crate::utils::play_cap_reached;
use crate::utils::scale_attack_damage;
use crate::utils::scale_block_gain;
use crate::utils::strike_dummy_bonus;
use crate::utils::vuln_factor;
use crate::utils::weak_factor;
use crate::utils::wrist_blade_bonus;

use super::effect::PyEffect;
use super::effect::PyEffectBlockGain;
use super::effect::PyEffectDamageFinisher;
use super::effect::PyEffectDamageFlechettes;
use super::effect::PyEffectDamageMindBlast;
use super::effect::PyEffectDamagePhysical;
use super::effect::PyEffectDamagePhysicalIfPoisoned;
use super::effect::PyEffectEscapePlanCheck;
use super::effect::PyEffectModifierDelta;
use super::effect::snapshot_effect;
use super::macros::flat_variants;
use super::macros::mirror_enum;
use super::modifier::PyModifierKind;

mirror_enum!(PyCardKind from CardKind, "CardKind", {
    Attack, Skill, Power, Curse, Status,
});

mirror_enum!(PyCardColor from CardColor, "CardColor", {
    Green, Colorless, Curse,
});

mirror_enum!(PyCardRarity from CardRarity, "CardRarity", {
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

mirror_enum!(PyPlayRestriction from PlayRestriction, "PlayRestriction", {
    Always, Never, CardPileDrawEmpty, CardPileDrawHasAttack, CardPileDrawHasSkill,
});

mirror_enum!(PyCardPile from CardPile, "CardPile", {
    Hand, Draw, Discard, Deck,
});

mirror_enum!(PyCostScope from CostScope, "CostScope", {
    Turn, Combat,
});

mirror_enum!(PyCardName from CardName, "CardName", {
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
    Doubt, Decay, Injury, Shame, Writhe, Parasite, Normality, Clumsy, Apparition, Bite,
    DarkShackles, DramaticEntrance, Jax, Panacea, Trip, Apotheosis, Chrysalis, Discovery,
    Enlightenment, HandOfGreed, Impatience, JackOfAllTrades, Madness, Magnetism, Metamorphosis,
    Panache, PanicButton, SadisticNature, ThinkingAhead, Transmutation, Forethought, Mayhem,
    Purity, SecretTechnique, SecretWeapon, TheBomb, Violence, CurseOfTheBell, Wound,
    RitualDagger, Necronomicurse,
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
    pub id: usize, // Entity ID
    pub name: PyCardName,

    // Cost-related fields
    pub cost: u16,
    pub cost_base: u8,
    pub cost_override: Option<u8>,
    pub cost_override_scope: Option<PyCostScope>,
    pub free_to_play_once: bool,
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
    pub playable: bool,

    // Effects when played, with Dex / Str / Vigor / etc. applied
    pub effects_play: Vec<PyEffect>,
    pub effects_discard: Vec<PyEffect>, // When discarded from the hand
    pub effects_draw: Vec<PyEffect>,    // When drawn
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
            Self::Clumsy => "Clumsy",
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
            Self::RitualDagger => "Ritual Dagger",
            Self::Necronomicurse => "Necronomicurse",
        }
    }
}

// A combat Card's effects with the Character's Modifiers and Relic bonuses folded into the damage and block amounts; Mind Blast's base is `draw_pile_size`
pub(crate) fn snapshot_adjusted_effects(
    state: &GameState,
    card: &Entity,
    draw_pile_size: u16,
) -> Vec<PyEffect> {
    let char_mods = &state.entities[state.id_character].modifiers;

    // Strike Dummy and Wrist Blade join the base damage before scaling, as in the engine
    let cost = get_card_effective_cost(card, state.combat.energy.energy_current);
    let bonus = strike_dummy_bonus(card.card_name, &state.id_relics)
        + wrist_blade_bonus(card, cost, &state.id_relics);
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
    let pen_nib = has_modifier(char_mods, ModifierKind::PenNib);
    let dex = if has_modifier(char_mods, ModifierKind::Dexterity) {
        modifier_stacks(char_mods, ModifierKind::Dexterity)
    } else {
        0
    };
    let frail = has_modifier(char_mods, ModifierKind::Frail);
    let no_block = has_modifier(char_mods, ModifierKind::NoBlock);

    // Every hit of the Card shows the same scaled damage; Player attacker: Paper Krane never applies
    let adjusted_damage = |base: u16| {
        scale_attack_damage(
            base.saturating_add(bonus).saturating_add(vigor),
            str_stacks,
            double,
            pen_nib,
            weak_factor(weak, false),
            vuln_factor(false, false),
            false,
        )
    };

    // No Block zeroes every block the Card would grant
    let adjusted_block = |base: u16| {
        if no_block {
            0
        } else {
            scale_block_gain(base, dex, frail)
        }
    };

    card.card_effects_play[..card.card_effects_play_len as usize]
        .iter()
        .map(snapshot_effect)
        .map(|effect| match effect {
            PyEffect::DamagePhysical(PyEffectDamagePhysical {
                amount,
                instances,
                lifesteal,
                target,
            }) => PyEffect::DamagePhysical(PyEffectDamagePhysical {
                amount: adjusted_damage(amount),
                instances,
                lifesteal,
                target,
            }),
            PyEffect::DamagePhysicalIfPoisoned(PyEffectDamagePhysicalIfPoisoned {
                amount,
                target,
            }) => PyEffect::DamagePhysicalIfPoisoned(PyEffectDamagePhysicalIfPoisoned {
                amount: adjusted_damage(amount),
                target,
            }),
            PyEffect::DamageFinisher(PyEffectDamageFinisher { damage, target }) => {
                PyEffect::DamageFinisher(PyEffectDamageFinisher {
                    damage: adjusted_damage(damage),
                    target,
                })
            }
            PyEffect::DamageFlechettes(PyEffectDamageFlechettes { damage, target }) => {
                PyEffect::DamageFlechettes(PyEffectDamageFlechettes {
                    damage: adjusted_damage(damage),
                    target,
                })
            }
            PyEffect::DamageMindBlast(PyEffectDamageMindBlast { target, .. }) => {
                PyEffect::DamageMindBlast(PyEffectDamageMindBlast {
                    damage: adjusted_damage(draw_pile_size),
                    target,
                })
            }
            PyEffect::BlockGain(PyEffectBlockGain { amount, target }) => {
                PyEffect::BlockGain(PyEffectBlockGain {
                    amount: adjusted_block(amount),
                    target,
                })
            }
            // Dodge and Roll's next-turn block scales like its block
            PyEffect::ModifierDelta(PyEffectModifierDelta {
                kind: PyModifierKind::NextTurnBlock,
                stacks,
                target,
            }) => PyEffect::ModifierDelta(PyEffectModifierDelta {
                kind: PyModifierKind::NextTurnBlock,
                stacks: adjusted_block(stacks.max(0) as u16) as i16,
                target,
            }),
            PyEffect::EscapePlanCheck(PyEffectEscapePlanCheck { block, target }) => {
                PyEffect::EscapePlanCheck(PyEffectEscapePlanCheck {
                    block: adjusted_block(block),
                    target,
                })
            }
            other => other,
        })
        .collect()
}

// A Card outside the fight (the master deck, a reward, the shop, an event) shows its own cost and effects, and counts as playable
pub(crate) fn snapshot_card(state: &GameState, id_card: usize) -> PyCard {
    let card = &state.entities[id_card];
    let effects_play = card.card_effects_play[..card.card_effects_play_len as usize]
        .iter()
        .map(snapshot_effect)
        .collect();
    build_py_card(
        id_card,
        card,
        get_card_effective_cost(card, 0),
        true,
        effects_play,
    )
}

// A Card of the fight: what playing it would cost and do, Mind Blast hitting for `draw_pile_size`, and whether it can be played now
pub(crate) fn snapshot_card_combat(
    state: &GameState,
    id_card: usize,
    draw_pile_size: u16,
) -> PyCard {
    let card = &state.entities[id_card];
    let energy_current = state.combat.energy.energy_current;
    let entangled = has_modifier(
        &state.entities[state.id_character].modifiers,
        ModifierKind::Entangled,
    );
    let cost = get_card_effective_cost(card, energy_current);
    let playable = is_play_restriction_satisfied(
        card.card_play_restriction,
        card.card_kind,
        &state.combat.id_card_pile_draw,
        &state.entities,
        &state.id_relics,
    ) && !(entangled && card.card_kind == CardKind::Attack)
        && !play_cap_reached(
            &state.combat.id_card_pile_hand,
            &state.entities,
            &state.id_relics,
            state.combat.this_turn_cards_played,
        )
        && cost <= energy_current;
    build_py_card(
        id_card,
        card,
        cost,
        playable,
        snapshot_adjusted_effects(state, card, draw_pile_size),
    )
}

// The Card's own fields, with the cost, playability and play effects its place decides
fn build_py_card(
    id_card: usize,
    card: &Entity,
    cost: u16,
    playable: bool,
    effects_play: Vec<PyEffect>,
) -> PyCard {
    PyCard {
        id: id_card,
        name: card.card_name.into(),
        cost,
        cost_base: card.card_cost,
        cost_override: card
            .card_cost_override
            .map(|cost_override| cost_override.amount),
        cost_override_scope: card
            .card_cost_override
            .map(|cost_override| cost_override.scope.into()),
        free_to_play_once: card.card_free_to_play_once,
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
        requires_target: entity_requires_target(card),
        retain: card.card_retain,
        playable,
        effects_play,
        effects_discard: card
            .card_effects_discard
            .iter()
            .map(snapshot_effect)
            .collect(),
        effects_draw: card.card_effects_draw.iter().map(snapshot_effect).collect(),
    }
}
