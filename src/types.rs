// Core type enums shared across the engine.

// Vitals: physical combat state. Shared by character and monsters.
#[derive(Debug, Clone, Copy)]
pub struct Vitals {
    pub health: u16,
    pub health_max: u16,
    pub block: u16,
}

pub const ZERO_VITALS: Vitals = Vitals {
    health: 0,
    health_max: 0,
    block: 0,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CardName {
    AThousandCuts,
    Accuracy,
    Acrobatics,
    Adrenaline,
    AfterImage,
    AllOutAttack,
    Backflip,
    Backstab,
    Bane,
    BladeDance,
    Blur,
    BouncingFlask,
    Burst,
    CalculatedGamble,
    Caltrops,
    Catalyst,
    CloakAndDagger,
    Concentrate,
    CorpseExplosion,
    CripplingPoison,
    DaggerSpray,
    DaggerThrow,
    Dash,
    DeadlyPoison,
    Defend,
    Deflect,
    DieDieDie,
    DodgeAndRoll,
    Envenom,
    FlyingKnee,
    Footwork,
    InfiniteBlades,
    LegSweep,
    Neutralize,
    NoxiousFumes,
    Outmaneuver,
    PhantasmalKiller,
    PiercingWail,
    PoisonedStab,
    Predator,
    Prepared,
    QuickSlash,
    RiddleWithHoles,
    Shiv,
    Slice,
    Strike,
    SuckerPunch,
    Survivor,
    Terror,
    ToolsOfTheTrade,
    WellLaidPlans,
}

impl CardName {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AThousandCuts => "A Thousand Cuts",
            Self::Accuracy => "Accuracy",
            Self::Acrobatics => "Acrobatics",
            Self::Adrenaline => "Adrenaline",
            Self::AfterImage => "After Image",
            Self::AllOutAttack => "All Out Attack",
            Self::Backflip => "Backflip",
            Self::Backstab => "Backstab",
            Self::Bane => "Bane",
            Self::BladeDance => "Blade Dance",
            Self::Blur => "Blur",
            Self::BouncingFlask => "Bouncing Flask",
            Self::Burst => "Burst",
            Self::CalculatedGamble => "Calculated Gamble",
            Self::Caltrops => "Caltrops",
            Self::Catalyst => "Catalyst",
            Self::CloakAndDagger => "Cloak And Dagger",
            Self::Concentrate => "Concentrate",
            Self::CorpseExplosion => "Corpse Explosion",
            Self::CripplingPoison => "Crippling Poison",
            Self::DaggerSpray => "Dagger Spray",
            Self::DaggerThrow => "Dagger Throw",
            Self::Dash => "Dash",
            Self::DeadlyPoison => "Deadly Poison",
            Self::Defend => "Defend",
            Self::Deflect => "Deflect",
            Self::DieDieDie => "Die Die Die",
            Self::DodgeAndRoll => "Dodge And Roll",
            Self::Envenom => "Envenom",
            Self::FlyingKnee => "Flying Knee",
            Self::Footwork => "Footwork",
            Self::InfiniteBlades => "Infinite Blades",
            Self::LegSweep => "Leg Sweep",
            Self::Neutralize => "Neutralize",
            Self::NoxiousFumes => "Noxious Fumes",
            Self::Outmaneuver => "Outmaneuver",
            Self::PhantasmalKiller => "Phantasmal Killer",
            Self::PiercingWail => "Piercing Wail",
            Self::PoisonedStab => "Poisoned Stab",
            Self::Predator => "Predator",
            Self::Prepared => "Prepared",
            Self::QuickSlash => "Quick Slash",
            Self::RiddleWithHoles => "Riddle With Holes",
            Self::Shiv => "Shiv",
            Self::Slice => "Slice",
            Self::Strike => "Strike",
            Self::SuckerPunch => "Sucker Punch",
            Self::Survivor => "Survivor",
            Self::Terror => "Terror",
            Self::ToolsOfTheTrade => "Tools Of The Trade",
            Self::WellLaidPlans => "Well Laid Plans",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardKind {
    Attack,
    Curse,
    Power,
    Skill,
    Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardColor {
    Colorless,
    Curse,
    Green,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardRarity {
    Basic,
    Common,
    Uncommon,
    Rare,
    Special,
    Curse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MonsterName {
    Cultist,
    FungiBeast,
    JawWorm,
    TheGuardian,
}

impl MonsterName {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cultist => "Cultist",
            Self::FungiBeast => "Fungi Beast",
            Self::JawWorm => "Jaw Worm",
            Self::TheGuardian => "The Guardian",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MonsterKind {
    Normal,
    Elite,
    Boss,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoomKind {
    CombatBoss,
    CombatMonster,
    RestSite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    CombatReward,
    CombatDefault,
    CombatAwaitDiscard { num: u8 },
    CombatAwaitRetain { num: u8 },
    GameOver,
    Map,
    RestSite,
}
