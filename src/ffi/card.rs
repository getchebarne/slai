use pyo3::prelude::*;

use crate::action::Action;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::game::GameState;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::CostScope;
use crate::types::RelicName;
use crate::utils::entity_requires_target;
use crate::utils::get_card_effective_cost;
use crate::utils::has_relic;

use super::effect::PyEffect;
use super::effect::snapshot_effects;
use super::identity_hash;
use super::macros::flat_variants;
use super::macros::mirror_enum;

mirror_enum!(PyCardKind from CardKind, "CardKind", {
    Attack, Skill, Power, Curse, Status,
});

mirror_enum!(PyCardColor from CardColor, "CardColor", {
    Green, Colorless, Curse,
});

mirror_enum!(PyCardRarity from CardRarity, "CardRarity", {
    Basic, Common, Uncommon, Rare, Special, Curse,
});

flat_variants!(PyCardCostKind as "CardCostKind" {
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
    Hand, Draw, DrawTop, Discard, Deck,
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
    Terror, ToolsOfTheTrade, Unload, Void, WellLaidPlans, WraithForm, AscendersBane, Regret, Pain,
    Doubt, Decay, Injury, Shame, Writhe, Parasite, Normality, Clumsy, Apparition, Bite,
    DarkShackles, DramaticEntrance, Jax, Panacea, Trip, Apotheosis, Chrysalis, Discovery,
    Enlightenment, HandOfGreed, Impatience, JackOfAllTrades, Madness, Magnetism, Metamorphosis,
    Panache, PanicButton, SadisticNature, ThinkingAhead, Transmutation, Forethought, Mayhem,
    Purity, SecretTechnique, SecretWeapon, TheBomb, Violence, CurseOfTheBell, Wound,
    RitualDagger, Necronomicurse,
});

// Exposed structs
#[pyclass(eq, hash, frozen, get_all, name = "Card", module = "slai._slai")]
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PyCard {
    pub id: usize, // Entity ID
    pub name: PyCardName,

    // Cost-related fields
    pub cost: u16,
    pub cost_base: u16,
    pub cost_override: Option<u16>,
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
    pub playable: bool, // Playable now: a CardPlay for it is legal

    // Effects when played, with Dex / Str / Vigor / etc. applied
    pub effects_play: Vec<PyEffect>,
    pub effects_discard: Vec<PyEffect>, // When discarded from the hand
    pub effects_draw: Vec<PyEffect>,    // When drawn

    // Price while the Shop offers it
    pub price: Option<u16>,

    // The Cards above it in the draw pile, when its draw depth is known (Frozen Eye: always)
    pub draw_depth: Option<u16>,

    // Hash of every field but the id; private: no struct update elsewhere
    identity_hash: u64,
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
            Self::Void => "Void",
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

// A Card outside the fight (the master deck, a reward, the shop, an event) shows its own cost and effects
pub(crate) fn snapshot_card(state: &GameState, id_card: usize) -> PyCard {
    let card = &state.entities[id_card];
    let effects_play = snapshot_effects(
        state,
        &card.card_effects_play[..card.card_effects_play_len as usize],
        None,
    );
    build_py_card(
        state,
        id_card,
        get_card_effective_cost(card, 0),
        effects_play,
    )
}

// A Card of the fight: what playing it would cost and do, Mind Blast hitting for `draw_pile_size`
pub(crate) fn snapshot_card_combat(
    state: &GameState,
    id_card: usize,
    draw_pile_size: u16,
) -> PyCard {
    let cost =
        get_card_effective_cost(&state.entities[id_card], state.combat.energy.energy_current);
    snapshot_card_combat_at_cost(state, id_card, cost, draw_pile_size)
}

// A Card of the fight priced at `cost`; a waiting play carries its queued cost
pub(crate) fn snapshot_card_combat_at_cost(
    state: &GameState,
    id_card: usize,
    cost: u16,
    draw_pile_size: u16,
) -> PyCard {
    let card = &state.entities[id_card];
    let effects_play = snapshot_effects(
        state,
        &card.card_effects_play[..card.card_effects_play_len as usize],
        Some((card, cost, draw_pile_size)),
    );
    build_py_card(state, id_card, cost, effects_play)
}

// The Card's own fields, with the cost and play effects its place decides
fn build_py_card(
    state: &GameState,
    id_card: usize,
    cost: u16,
    effects_play: Vec<PyEffect>,
) -> PyCard {
    let card = &state.entities[id_card];

    // Counted from the draw pile's top when the player knows it; Frozen Eye shows every Card's
    let draw_depth_known =
        card.card_draw_visible || has_relic(&state.id_relics, RelicName::FrozenEye);
    let draw_depth = state
        .combat
        .id_card_pile_draw
        .iter()
        .rev()
        .position(|&id| id == id_card)
        .filter(|_| draw_depth_known)
        .map(|draw_depth| draw_depth as u16);
    let py_card = PyCard {
        id: 0,
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
        playable: state
            .legal_actions
            .iter()
            .any(|action| matches!(action, Action::CardPlay { id_card: id, .. } if *id == id_card)),
        effects_play,
        effects_discard: snapshot_effects(state, card.card_effects_discard, None),
        effects_draw: snapshot_effects(state, card.card_effects_draw, None),
        price: card.price,
        draw_depth,
        identity_hash: 0,
    };

    // The hash is taken before the id is set, so it leaves the id out
    PyCard {
        id: id_card,
        identity_hash: identity_hash(&py_card),
        ..py_card
    }
}
