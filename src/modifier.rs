// Modifier system: fixed-size arrays + bitmask, all free functions.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ModifierKind {
    Accuracy = 0,
    AfterImage,
    Blur,
    Burst,
    Dexterity,
    DoubleDamage,
    InfiniteBlades,
    ModeShift,
    NextTurnBlock,
    NextTurnEnergy,
    Phantasmal,
    Ritual,
    SharpHide,
    SporeCloud,
    Strength,
    ThousandCuts,
    Vulnerable,
    Weak,
    Count, // sentinel for array sizing
}

pub const MODIFIER_COUNT: usize = ModifierKind::Count as usize;

impl ModifierKind {
    pub fn from_u8(v: u8) -> Self {
        assert!((v as usize) < MODIFIER_COUNT, "invalid ModifierKind: {v}");
        // SAFETY: repr(u8) and we validated the range
        unsafe { std::mem::transmute(v) }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Modifier {
    pub kind: ModifierKind,
    pub is_buff: bool,
    pub stacks_duration: bool,
    pub min_stacks: i16,
    pub max_stacks: i16,
}

static MODIFIERS: [Modifier; MODIFIER_COUNT] = [
    Modifier {
        kind: ModifierKind::Accuracy,
        is_buff: true,
        stacks_duration: false,
        min_stacks: 1,
        max_stacks: 999,
    },
    Modifier {
        kind: ModifierKind::AfterImage,
        is_buff: true,
        stacks_duration: false,
        min_stacks: 1,
        max_stacks: 999,
    },
    Modifier {
        kind: ModifierKind::Blur,
        is_buff: true,
        stacks_duration: true,
        min_stacks: 1,
        max_stacks: 999,
    },
    Modifier {
        kind: ModifierKind::Burst,
        is_buff: true,
        stacks_duration: false,
        min_stacks: 1,
        max_stacks: 999,
    },
    Modifier {
        kind: ModifierKind::Dexterity,
        is_buff: true,
        stacks_duration: false,
        min_stacks: -999,
        max_stacks: 999,
    },
    Modifier {
        kind: ModifierKind::DoubleDamage,
        is_buff: true,
        stacks_duration: true,
        min_stacks: 1,
        max_stacks: 999,
    },
    Modifier {
        kind: ModifierKind::InfiniteBlades,
        is_buff: true,
        stacks_duration: false,
        min_stacks: 1,
        max_stacks: 999,
    },
    Modifier {
        kind: ModifierKind::ModeShift,
        is_buff: true,
        stacks_duration: false,
        min_stacks: 1,
        max_stacks: 999,
    },
    Modifier {
        kind: ModifierKind::NextTurnBlock,
        is_buff: true,
        stacks_duration: false,
        min_stacks: 1,
        max_stacks: 999,
    },
    Modifier {
        kind: ModifierKind::NextTurnEnergy,
        is_buff: true,
        stacks_duration: false,
        min_stacks: 1,
        max_stacks: 999,
    },
    Modifier {
        kind: ModifierKind::Phantasmal,
        is_buff: true,
        stacks_duration: true,
        min_stacks: 1,
        max_stacks: 999,
    },
    Modifier {
        kind: ModifierKind::Ritual,
        is_buff: true,
        stacks_duration: false,
        min_stacks: 1,
        max_stacks: 999,
    },
    Modifier {
        kind: ModifierKind::SharpHide,
        is_buff: true,
        stacks_duration: false,
        min_stacks: 1,
        max_stacks: 999,
    },
    Modifier {
        kind: ModifierKind::SporeCloud,
        is_buff: true,
        stacks_duration: false,
        min_stacks: 1,
        max_stacks: 999,
    },
    Modifier {
        kind: ModifierKind::Strength,
        is_buff: true,
        stacks_duration: false,
        min_stacks: -999,
        max_stacks: 999,
    },
    Modifier {
        kind: ModifierKind::ThousandCuts,
        is_buff: true,
        stacks_duration: false,
        min_stacks: 1,
        max_stacks: 999,
    },
    Modifier {
        kind: ModifierKind::Vulnerable,
        is_buff: false,
        stacks_duration: true,
        min_stacks: 0,
        max_stacks: 999,
    },
    Modifier {
        kind: ModifierKind::Weak,
        is_buff: false,
        stacks_duration: true,
        min_stacks: 0,
        max_stacks: 999,
    },
];

pub fn modifier_def(kind: ModifierKind) -> &'static Modifier {
    &MODIFIERS[kind as usize]
}

#[derive(Debug, Clone)]
pub struct Modifiers {
    pub stacks: [i16; MODIFIER_COUNT],
    pub is_new: [bool; MODIFIER_COUNT],
    pub active: u32,
}

pub fn modifiers_new() -> Modifiers {
    Modifiers {
        stacks: [0; MODIFIER_COUNT],
        is_new: [false; MODIFIER_COUNT],
        active: 0,
    }
}

pub fn modifier_stacks(mods: &Modifiers, kind: ModifierKind) -> i16 {
    mods.stacks[kind as usize]
}

pub fn modifier_has(mods: &Modifiers, kind: ModifierKind) -> bool {
    mods.active & (1 << kind as u32) != 0
}

pub fn modifier_apply(mods: &mut Modifiers, kind: ModifierKind, stacks: i16) {
    let cfg = modifier_def(kind);
    let idx = kind as usize;
    if modifier_has(mods, kind) {
        mods.stacks[idx] = (mods.stacks[idx] + stacks).clamp(cfg.min_stacks, cfg.max_stacks);
    } else {
        mods.stacks[idx] = stacks.clamp(cfg.min_stacks, cfg.max_stacks);
        mods.is_new[idx] = true;
        mods.active |= 1 << kind as u32;
    }
}

pub fn modifier_remove(mods: &mut Modifiers, kind: ModifierKind) {
    let idx = kind as usize;
    mods.stacks[idx] = 0;
    mods.is_new[idx] = false;
    mods.active &= !(1 << kind as u32);
}

pub fn modifier_tick(mods: &mut Modifiers) {
    let mut bits = mods.active;
    while bits != 0 {
        let idx = bits.trailing_zeros() as usize;
        bits &= bits - 1;
        let kind = ModifierKind::from_u8(idx as u8);
        let cfg = modifier_def(kind);
        if cfg.stacks_duration && !mods.is_new[idx] {
            mods.stacks[idx] -= 1;
            if mods.stacks[idx] < cfg.min_stacks {
                modifier_remove(mods, kind);
            }
        }
    }
}

pub fn modifier_set_not_new(mods: &mut Modifiers) {
    mods.is_new = [false; MODIFIER_COUNT];
}

pub fn modifier_clear(mods: &mut Modifiers) {
    mods.stacks = [0; MODIFIER_COUNT];
    mods.is_new = [false; MODIFIER_COUNT];
    mods.active = 0;
}
