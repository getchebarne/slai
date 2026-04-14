// Core type enums shared across the engine.

// Vitals: physical combat state. Shared by character and monsters.
#[derive(Debug, Clone, Copy)]
pub struct Vitals {
    pub health: u16,
    pub health_max: u16,
    pub block: u16,
}

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
    BladeDance,
    Blur,
    Burst,
    CalculatedGamble,
    CloakAndDagger,
    DaggerThrow,
    Dash,
    Defend,
    Deflect,
    DieDieDie,
    DodgeAndRoll,
    FlyingKnee,
    Footwork,
    InfiniteBlades,
    LegSweep,
    Neutralize,
    Outmaneuver,
    PhantasmalKiller,
    Shiv,
    Strike,
    Survivor,
    Terror,
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
            Self::BladeDance => "Blade Dance",
            Self::Blur => "Blur",
            Self::Burst => "Burst",
            Self::CalculatedGamble => "Calculated Gamble",
            Self::CloakAndDagger => "Cloak And Dagger",
            Self::DaggerThrow => "Dagger Throw",
            Self::Dash => "Dash",
            Self::Defend => "Defend",
            Self::Deflect => "Deflect",
            Self::DieDieDie => "Die Die Die",
            Self::DodgeAndRoll => "Dodge And Roll",
            Self::FlyingKnee => "Flying Knee",
            Self::Footwork => "Footwork",
            Self::InfiniteBlades => "Infinite Blades",
            Self::LegSweep => "Leg Sweep",
            Self::Neutralize => "Neutralize",
            Self::Outmaneuver => "Outmaneuver",
            Self::PhantasmalKiller => "Phantasmal Killer",
            Self::Shiv => "Shiv",
            Self::Strike => "Strike",
            Self::Survivor => "Survivor",
            Self::Terror => "Terror",
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
pub enum RoomType {
    CombatBoss,
    CombatMonster,
    RestSite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    CombatReward,
    CombatDefault,
    CombatAwaitDiscard { num: u8 },
    GameOver,
    Map,
    RestSite,
}

