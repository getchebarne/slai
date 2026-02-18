// Card definitions: static Card values for every card, keyed by (CardName, upgraded).

use crate::effect::{EffectTemplate, SelectionKind, TargetKind};
use crate::modifier::ModifierKind;
use crate::state::Card;
use crate::types::*;

// ---------------------------------------------------------------------------
// Static card definitions
// ---------------------------------------------------------------------------

static STRIKE: Card = Card {
    name: CardName::Strike,
    type_: CardType::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::DamagePhysical {
        base: 6,
        target: TargetKind::CardTarget,
    }],
};
static STRIKE_PLUS: Card = Card {
    name: CardName::Strike,
    type_: CardType::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::DamagePhysical {
        base: 9,
        target: TargetKind::CardTarget,
    }],
};

static DEFEND: Card = Card {
    name: CardName::Defend,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::BlockGain {
        amount: 5,
        target: TargetKind::Character,
    }],
};
static DEFEND_PLUS: Card = Card {
    name: CardName::Defend,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::BlockGain {
        amount: 8,
        target: TargetKind::Character,
    }],
};

static NEUTRALIZE: Card = Card {
    name: CardName::Neutralize,
    type_: CardType::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 0,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::DamagePhysical {
            base: 3,
            target: TargetKind::CardTarget,
        },
        EffectTemplate::ModifierGain {
            kind: ModifierKind::Weak,
            stacks: 1,
            target: TargetKind::CardTarget,
        },
    ],
};
static NEUTRALIZE_PLUS: Card = Card {
    name: CardName::Neutralize,
    type_: CardType::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 0,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::DamagePhysical {
            base: 4,
            target: TargetKind::CardTarget,
        },
        EffectTemplate::ModifierGain {
            kind: ModifierKind::Weak,
            stacks: 2,
            target: TargetKind::CardTarget,
        },
    ],
};

static SURVIVOR: Card = Card {
    name: CardName::Survivor,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 8,
            target: TargetKind::Character,
        },
        EffectTemplate::CardDiscard {
            selection: SelectionKind::Input,
        },
    ],
};
static SURVIVOR_PLUS: Card = Card {
    name: CardName::Survivor,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Basic,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 11,
            target: TargetKind::Character,
        },
        EffectTemplate::CardDiscard {
            selection: SelectionKind::Input,
        },
    ],
};

static BACKFLIP: Card = Card {
    name: CardName::Backflip,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 5,
            target: TargetKind::Character,
        },
        EffectTemplate::CardDraw { count: 2 },
    ],
};
static BACKFLIP_PLUS: Card = Card {
    name: CardName::Backflip,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 5,
            target: TargetKind::Character,
        },
        EffectTemplate::CardDraw { count: 2 },
    ],
};

static BACKSTAB: Card = Card {
    name: CardName::Backstab,
    type_: CardType::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 0,
    upgraded: false,
    exhaust: true,
    innate: true,
    effects: &[EffectTemplate::DamagePhysical {
        base: 11,
        target: TargetKind::CardTarget,
    }],
};
static BACKSTAB_PLUS: Card = Card {
    name: CardName::Backstab,
    type_: CardType::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 0,
    upgraded: true,
    exhaust: true,
    innate: true,
    effects: &[EffectTemplate::DamagePhysical {
        base: 15,
        target: TargetKind::CardTarget,
    }],
};

static BLADE_DANCE: Card = Card {
    name: CardName::BladeDance,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::AddShivs { count: 3 }],
};
static BLADE_DANCE_PLUS: Card = Card {
    name: CardName::BladeDance,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::AddShivs { count: 4 }],
};

static CLOAK_AND_DAGGER: Card = Card {
    name: CardName::CloakAndDagger,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 6,
            target: TargetKind::Character,
        },
        EffectTemplate::AddShivs { count: 1 },
    ],
};
static CLOAK_AND_DAGGER_PLUS: Card = Card {
    name: CardName::CloakAndDagger,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 6,
            target: TargetKind::Character,
        },
        EffectTemplate::AddShivs { count: 2 },
    ],
};

static DAGGER_THROW: Card = Card {
    name: CardName::DaggerThrow,
    type_: CardType::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::DamagePhysical {
            base: 9,
            target: TargetKind::CardTarget,
        },
        EffectTemplate::CardDraw { count: 1 },
        EffectTemplate::CardDiscard {
            selection: SelectionKind::Input,
        },
    ],
};
static DAGGER_THROW_PLUS: Card = Card {
    name: CardName::DaggerThrow,
    type_: CardType::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::DamagePhysical {
            base: 12,
            target: TargetKind::CardTarget,
        },
        EffectTemplate::CardDraw { count: 1 },
        EffectTemplate::CardDiscard {
            selection: SelectionKind::Input,
        },
    ],
};

static DASH: Card = Card {
    name: CardName::Dash,
    type_: CardType::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 2,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 10,
            target: TargetKind::Character,
        },
        EffectTemplate::DamagePhysical {
            base: 10,
            target: TargetKind::CardTarget,
        },
    ],
};
static DASH_PLUS: Card = Card {
    name: CardName::Dash,
    type_: CardType::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 2,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 13,
            target: TargetKind::Character,
        },
        EffectTemplate::DamagePhysical {
            base: 13,
            target: TargetKind::CardTarget,
        },
    ],
};

static DEFLECT: Card = Card {
    name: CardName::Deflect,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 0,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::BlockGain {
        amount: 4,
        target: TargetKind::Character,
    }],
};
static DEFLECT_PLUS: Card = Card {
    name: CardName::Deflect,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 0,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::BlockGain {
        amount: 7,
        target: TargetKind::Character,
    }],
};

static DODGE_AND_ROLL: Card = Card {
    name: CardName::DodgeAndRoll,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 4,
            target: TargetKind::Character,
        },
        EffectTemplate::ModifierGain {
            kind: ModifierKind::NextTurnBlock,
            stacks: 4,
            target: TargetKind::Character,
        },
    ],
};
static DODGE_AND_ROLL_PLUS: Card = Card {
    name: CardName::DodgeAndRoll,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 6,
            target: TargetKind::Character,
        },
        EffectTemplate::ModifierGain {
            kind: ModifierKind::NextTurnBlock,
            stacks: 6,
            target: TargetKind::Character,
        },
    ],
};

static FLYING_KNEE: Card = Card {
    name: CardName::FlyingKnee,
    type_: CardType::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::DamagePhysical {
            base: 8,
            target: TargetKind::CardTarget,
        },
        EffectTemplate::ModifierGain {
            kind: ModifierKind::NextTurnEnergy,
            stacks: 1,
            target: TargetKind::Character,
        },
    ],
};
static FLYING_KNEE_PLUS: Card = Card {
    name: CardName::FlyingKnee,
    type_: CardType::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::DamagePhysical {
            base: 11,
            target: TargetKind::CardTarget,
        },
        EffectTemplate::ModifierGain {
            kind: ModifierKind::NextTurnEnergy,
            stacks: 1,
            target: TargetKind::Character,
        },
    ],
};

static OUTMANEUVER: Card = Card {
    name: CardName::Outmaneuver,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::NextTurnEnergy,
        stacks: 2,
        target: TargetKind::Character,
    }],
};
static OUTMANEUVER_PLUS: Card = Card {
    name: CardName::Outmaneuver,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::NextTurnEnergy,
        stacks: 3,
        target: TargetKind::Character,
    }],
};

static ACROBATICS: Card = Card {
    name: CardName::Acrobatics,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::CardDraw { count: 3 },
        EffectTemplate::CardDiscard {
            selection: SelectionKind::Input,
        },
    ],
};
static ACROBATICS_PLUS: Card = Card {
    name: CardName::Acrobatics,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Common,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::CardDraw { count: 3 },
        EffectTemplate::CardDiscard {
            selection: SelectionKind::Input,
        },
    ],
};

// --- Uncommon ---

static FOOTWORK: Card = Card {
    name: CardName::Footwork,
    type_: CardType::Power,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Dexterity,
        stacks: 2,
        target: TargetKind::Character,
    }],
};
static FOOTWORK_PLUS: Card = Card {
    name: CardName::Footwork,
    type_: CardType::Power,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Dexterity,
        stacks: 3,
        target: TargetKind::Character,
    }],
};

static LEG_SWEEP: Card = Card {
    name: CardName::LegSweep,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 2,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 11,
            target: TargetKind::Character,
        },
        EffectTemplate::ModifierGain {
            kind: ModifierKind::Weak,
            stacks: 2,
            target: TargetKind::CardTarget,
        },
    ],
};
static LEG_SWEEP_PLUS: Card = Card {
    name: CardName::LegSweep,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 2,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 14,
            target: TargetKind::Character,
        },
        EffectTemplate::ModifierGain {
            kind: ModifierKind::Weak,
            stacks: 3,
            target: TargetKind::CardTarget,
        },
    ],
};

static TERROR: Card = Card {
    name: CardName::Terror,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: false,
    exhaust: true,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Vulnerable,
        stacks: 99,
        target: TargetKind::CardTarget,
    }],
};
static TERROR_PLUS: Card = Card {
    name: CardName::Terror,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 0,
    upgraded: true,
    exhaust: true,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Vulnerable,
        stacks: 99,
        target: TargetKind::CardTarget,
    }],
};

static ALL_OUT_ATTACK: Card = Card {
    name: CardName::AllOutAttack,
    type_: CardType::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::DamagePhysical {
            base: 10,
            target: TargetKind::AllMonsters,
        },
        EffectTemplate::CardDiscard {
            selection: SelectionKind::Random,
        },
    ],
};
static ALL_OUT_ATTACK_PLUS: Card = Card {
    name: CardName::AllOutAttack,
    type_: CardType::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::DamagePhysical {
            base: 14,
            target: TargetKind::AllMonsters,
        },
        EffectTemplate::CardDiscard {
            selection: SelectionKind::Random,
        },
    ],
};

static CALCULATED_GAMBLE: Card = Card {
    name: CardName::CalculatedGamble,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 0,
    upgraded: false,
    exhaust: true,
    innate: false,
    effects: &[EffectTemplate::CalculatedGamble],
};
static CALCULATED_GAMBLE_PLUS: Card = Card {
    name: CardName::CalculatedGamble,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 0,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::CalculatedGamble],
};

static ACCURACY: Card = Card {
    name: CardName::Accuracy,
    type_: CardType::Power,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Accuracy,
        stacks: 4,
        target: TargetKind::Character,
    }],
};
static ACCURACY_PLUS: Card = Card {
    name: CardName::Accuracy,
    type_: CardType::Power,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Accuracy,
        stacks: 6,
        target: TargetKind::Character,
    }],
};

static BLUR: Card = Card {
    name: CardName::Blur,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 5,
            target: TargetKind::Character,
        },
        EffectTemplate::ModifierGain {
            kind: ModifierKind::Blur,
            stacks: 1,
            target: TargetKind::Character,
        },
    ],
};
static BLUR_PLUS: Card = Card {
    name: CardName::Blur,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[
        EffectTemplate::BlockGain {
            amount: 8,
            target: TargetKind::Character,
        },
        EffectTemplate::ModifierGain {
            kind: ModifierKind::Blur,
            stacks: 1,
            target: TargetKind::Character,
        },
    ],
};

static INFINITE_BLADES: Card = Card {
    name: CardName::InfiniteBlades,
    type_: CardType::Power,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::InfiniteBlades,
        stacks: 1,
        target: TargetKind::Character,
    }],
};
static INFINITE_BLADES_PLUS: Card = Card {
    name: CardName::InfiniteBlades,
    type_: CardType::Power,
    color: CardColor::Green,
    rarity: CardRarity::Uncommon,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: true,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::InfiniteBlades,
        stacks: 1,
        target: TargetKind::Character,
    }],
};

// --- Rare ---

static ADRENALINE: Card = Card {
    name: CardName::Adrenaline,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 0,
    upgraded: false,
    exhaust: true,
    innate: false,
    effects: &[
        EffectTemplate::EnergyGain { amount: 1 },
        EffectTemplate::CardDraw { count: 2 },
    ],
};
static ADRENALINE_PLUS: Card = Card {
    name: CardName::Adrenaline,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 0,
    upgraded: true,
    exhaust: true,
    innate: false,
    effects: &[
        EffectTemplate::EnergyGain { amount: 2 },
        EffectTemplate::CardDraw { count: 2 },
    ],
};

static DIE_DIE_DIE: Card = Card {
    name: CardName::DieDieDie,
    type_: CardType::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: false,
    exhaust: true,
    innate: false,
    effects: &[EffectTemplate::DamagePhysical {
        base: 13,
        target: TargetKind::AllMonsters,
    }],
};
static DIE_DIE_DIE_PLUS: Card = Card {
    name: CardName::DieDieDie,
    type_: CardType::Attack,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: true,
    exhaust: true,
    innate: false,
    effects: &[EffectTemplate::DamagePhysical {
        base: 17,
        target: TargetKind::AllMonsters,
    }],
};

static PHANTASMAL_KILLER: Card = Card {
    name: CardName::PhantasmalKiller,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Phantasmal,
        stacks: 1,
        target: TargetKind::Character,
    }],
};
static PHANTASMAL_KILLER_PLUS: Card = Card {
    name: CardName::PhantasmalKiller,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 0,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Phantasmal,
        stacks: 1,
        target: TargetKind::Character,
    }],
};

static BURST: Card = Card {
    name: CardName::Burst,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Burst,
        stacks: 1,
        target: TargetKind::Character,
    }],
};
static BURST_PLUS: Card = Card {
    name: CardName::Burst,
    type_: CardType::Skill,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::Burst,
        stacks: 2,
        target: TargetKind::Character,
    }],
};

static AFTER_IMAGE: Card = Card {
    name: CardName::AfterImage,
    type_: CardType::Power,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::AfterImage,
        stacks: 1,
        target: TargetKind::Character,
    }],
};
static AFTER_IMAGE_PLUS: Card = Card {
    name: CardName::AfterImage,
    type_: CardType::Power,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 1,
    upgraded: true,
    exhaust: false,
    innate: true,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::AfterImage,
        stacks: 1,
        target: TargetKind::Character,
    }],
};

static A_THOUSAND_CUTS: Card = Card {
    name: CardName::AThousandCuts,
    type_: CardType::Power,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 2,
    upgraded: false,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::ThousandCuts,
        stacks: 1,
        target: TargetKind::Character,
    }],
};
static A_THOUSAND_CUTS_PLUS: Card = Card {
    name: CardName::AThousandCuts,
    type_: CardType::Power,
    color: CardColor::Green,
    rarity: CardRarity::Rare,
    cost: 2,
    upgraded: true,
    exhaust: false,
    innate: false,
    effects: &[EffectTemplate::ModifierGain {
        kind: ModifierKind::ThousandCuts,
        stacks: 2,
        target: TargetKind::Character,
    }],
};

// --- Special ---

static SHIV: Card = Card {
    name: CardName::Shiv,
    type_: CardType::Attack,
    color: CardColor::Colorless,
    rarity: CardRarity::Special,
    cost: 0,
    upgraded: false,
    exhaust: true,
    innate: false,
    effects: &[EffectTemplate::DamagePhysical {
        base: 4,
        target: TargetKind::CardTarget,
    }],
};
static SHIV_PLUS: Card = Card {
    name: CardName::Shiv,
    type_: CardType::Attack,
    color: CardColor::Colorless,
    rarity: CardRarity::Special,
    cost: 0,
    upgraded: true,
    exhaust: true,
    innate: false,
    effects: &[EffectTemplate::DamagePhysical {
        base: 6,
        target: TargetKind::CardTarget,
    }],
};

// ---------------------------------------------------------------------------
// Lookup: (CardName, upgraded) -> Card
// ---------------------------------------------------------------------------

pub fn get_card(name: CardName, upgraded: bool) -> Card {
    match (name, upgraded) {
        (CardName::Strike, false) => STRIKE,
        (CardName::Strike, true) => STRIKE_PLUS,
        (CardName::Defend, false) => DEFEND,
        (CardName::Defend, true) => DEFEND_PLUS,
        (CardName::Neutralize, false) => NEUTRALIZE,
        (CardName::Neutralize, true) => NEUTRALIZE_PLUS,
        (CardName::Survivor, false) => SURVIVOR,
        (CardName::Survivor, true) => SURVIVOR_PLUS,
        (CardName::Backflip, false) => BACKFLIP,
        (CardName::Backflip, true) => BACKFLIP_PLUS,
        (CardName::Backstab, false) => BACKSTAB,
        (CardName::Backstab, true) => BACKSTAB_PLUS,
        (CardName::BladeDance, false) => BLADE_DANCE,
        (CardName::BladeDance, true) => BLADE_DANCE_PLUS,
        (CardName::CloakAndDagger, false) => CLOAK_AND_DAGGER,
        (CardName::CloakAndDagger, true) => CLOAK_AND_DAGGER_PLUS,
        (CardName::DaggerThrow, false) => DAGGER_THROW,
        (CardName::DaggerThrow, true) => DAGGER_THROW_PLUS,
        (CardName::Dash, false) => DASH,
        (CardName::Dash, true) => DASH_PLUS,
        (CardName::Deflect, false) => DEFLECT,
        (CardName::Deflect, true) => DEFLECT_PLUS,
        (CardName::DodgeAndRoll, false) => DODGE_AND_ROLL,
        (CardName::DodgeAndRoll, true) => DODGE_AND_ROLL_PLUS,
        (CardName::FlyingKnee, false) => FLYING_KNEE,
        (CardName::FlyingKnee, true) => FLYING_KNEE_PLUS,
        (CardName::Outmaneuver, false) => OUTMANEUVER,
        (CardName::Outmaneuver, true) => OUTMANEUVER_PLUS,
        (CardName::Acrobatics, false) => ACROBATICS,
        (CardName::Acrobatics, true) => ACROBATICS_PLUS,
        (CardName::Footwork, false) => FOOTWORK,
        (CardName::Footwork, true) => FOOTWORK_PLUS,
        (CardName::LegSweep, false) => LEG_SWEEP,
        (CardName::LegSweep, true) => LEG_SWEEP_PLUS,
        (CardName::Terror, false) => TERROR,
        (CardName::Terror, true) => TERROR_PLUS,
        (CardName::AllOutAttack, false) => ALL_OUT_ATTACK,
        (CardName::AllOutAttack, true) => ALL_OUT_ATTACK_PLUS,
        (CardName::CalculatedGamble, false) => CALCULATED_GAMBLE,
        (CardName::CalculatedGamble, true) => CALCULATED_GAMBLE_PLUS,
        (CardName::Accuracy, false) => ACCURACY,
        (CardName::Accuracy, true) => ACCURACY_PLUS,
        (CardName::Blur, false) => BLUR,
        (CardName::Blur, true) => BLUR_PLUS,
        (CardName::InfiniteBlades, false) => INFINITE_BLADES,
        (CardName::InfiniteBlades, true) => INFINITE_BLADES_PLUS,
        (CardName::Adrenaline, false) => ADRENALINE,
        (CardName::Adrenaline, true) => ADRENALINE_PLUS,
        (CardName::DieDieDie, false) => DIE_DIE_DIE,
        (CardName::DieDieDie, true) => DIE_DIE_DIE_PLUS,
        (CardName::PhantasmalKiller, false) => PHANTASMAL_KILLER,
        (CardName::PhantasmalKiller, true) => PHANTASMAL_KILLER_PLUS,
        (CardName::Burst, false) => BURST,
        (CardName::Burst, true) => BURST_PLUS,
        (CardName::AfterImage, false) => AFTER_IMAGE,
        (CardName::AfterImage, true) => AFTER_IMAGE_PLUS,
        (CardName::AThousandCuts, false) => A_THOUSAND_CUTS,
        (CardName::AThousandCuts, true) => A_THOUSAND_CUTS_PLUS,
        (CardName::Shiv, false) => SHIV,
        (CardName::Shiv, true) => SHIV_PLUS,
    }
}

// All card names eligible for card rewards (excludes Basic and Special)
pub const REWARD_POOL_COMMON: &[CardName] = &[
    CardName::Acrobatics,
    CardName::Backflip,
    CardName::BladeDance,
    CardName::CloakAndDagger,
    CardName::DaggerThrow,
    CardName::Deflect,
    CardName::DodgeAndRoll,
    CardName::FlyingKnee,
    CardName::Outmaneuver,
];

pub const REWARD_POOL_UNCOMMON: &[CardName] = &[
    CardName::Accuracy,
    CardName::AllOutAttack,
    CardName::Backstab,
    CardName::Blur,
    CardName::CalculatedGamble,
    CardName::Dash,
    CardName::Footwork,
    CardName::InfiniteBlades,
    CardName::LegSweep,
    CardName::Terror,
];

pub const REWARD_POOL_RARE: &[CardName] = &[
    CardName::AThousandCuts,
    CardName::Adrenaline,
    CardName::AfterImage,
    CardName::Burst,
    CardName::DieDieDie,
    CardName::PhantasmalKiller,
];
