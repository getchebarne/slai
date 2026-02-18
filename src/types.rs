// Core type enums shared across the engine.

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
pub enum CardType {
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
    Dummy,
    FungiBeast,
    JawWorm,
    LouseGreen,
    TheGuardian,
}

impl MonsterName {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cultist => "Cultist",
            Self::Dummy => "Dummy",
            Self::FungiBeast => "Fungi Beast",
            Self::JawWorm => "Jaw Worm",
            Self::LouseGreen => "Louse (green)",
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
#[repr(u8)]
pub enum Fsm {
    CardReward,
    CombatDefault,
    CombatAwaitTarget,
    CombatAwaitDiscard,
    GameOver,
    Map,
    RestSite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorId {
    Character,
    Monster(u8),
}
