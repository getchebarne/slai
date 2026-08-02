// Modifier system

use strum::EnumCount;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount)]
#[repr(u8)]
pub enum ModifierKind {
    Accuracy = 0,
    AfterImage,
    Angry,
    Artifact,
    Asleep,
    Blur,
    Burst,
    Choke,
    CorpseExplosion,
    CurlUp,
    Dexterity,
    DoubleDamage,
    DrawCardNextTurn,
    Enrage,
    Entangled,
    Envenom,
    Frail,
    InfiniteBlades,
    Intangible,
    Metallicize,
    ModeShift,
    NextTurnBlock,
    NextTurnEnergy,
    NoDraw,
    NoxiousFumes,
    Phantasmal,
    PlatedArmor,
    Poison,
    Retain,
    Ritual,
    Shackled,
    SharpHide,
    Splittable,
    SporeCloud,
    Strength,
    Thievery,
    Thorns,
    ThousandCuts,
    ToolsOfTheTrade,
    Vigor,
    Vulnerable,
    Weak,
    WraithForm,
    Buffer,
    PenNib,
    Magnetism,
    NoBlock,
    Panache,
    SadisticNature,
    Mayhem,
    TheBomb,
    Regeneration,
    LoseStrength,
    LoseDexterity,
    DuplicateNextCardPlay,
}

pub const MODIFIER_COUNT: usize = ModifierKind::COUNT;

pub fn modifier_kind_from_u8(v: u8) -> ModifierKind {
    assert!((v as usize) < MODIFIER_COUNT, "Invalid ModifierKind: {v}");
    // SAFETY: repr(u8) and we validated the range
    unsafe { std::mem::transmute(v) }
}

pub fn stacks_max_for(kind: ModifierKind) -> i16 {
    MODIFIER_DEFS[kind as usize].stacks_max
}

pub fn modifier_is_buff(kind: ModifierKind) -> bool {
    MODIFIER_DEFS[kind as usize].is_buff
}

#[derive(Debug, Clone, Copy)]
pub struct ModifierDef {
    pub kind: ModifierKind,
    pub is_buff: bool,
    pub stacks_duration: bool,
    pub stacks_min: i16,
    pub stacks_max: i16,
}

static MODIFIER_DEFS: [ModifierDef; MODIFIER_COUNT] = [
    ModifierDef {
        kind: ModifierKind::Accuracy,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::AfterImage,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Angry,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Artifact,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Asleep,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 1,
    },
    ModifierDef {
        kind: ModifierKind::Blur,
        is_buff: true,
        stacks_duration: true,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Burst,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Choke,
        is_buff: false,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::CorpseExplosion,
        is_buff: false,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::CurlUp,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Dexterity,
        is_buff: true,
        stacks_duration: false,
        stacks_min: -999,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::DoubleDamage,
        is_buff: true,
        stacks_duration: true,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::DrawCardNextTurn,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Enrage,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Entangled,
        is_buff: false,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 1,
    },
    ModifierDef {
        kind: ModifierKind::Envenom,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Frail,
        is_buff: false,
        stacks_duration: true,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::InfiniteBlades,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Intangible,
        is_buff: true,
        stacks_duration: true,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Metallicize,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::ModeShift,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::NextTurnBlock,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::NextTurnEnergy,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::NoDraw,
        is_buff: false,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 1,
    },
    ModifierDef {
        kind: ModifierKind::NoxiousFumes,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Phantasmal,
        is_buff: true,
        stacks_duration: true,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::PlatedArmor,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Poison,
        is_buff: false,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Retain,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Ritual,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Shackled,
        is_buff: false,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::SharpHide,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Splittable,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 1,
    },
    ModifierDef {
        kind: ModifierKind::SporeCloud,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Strength,
        is_buff: true,
        stacks_duration: false,
        stacks_min: -999,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Thievery,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Thorns,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::ThousandCuts,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::ToolsOfTheTrade,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Vigor,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Vulnerable,
        is_buff: false,
        stacks_duration: true,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Weak,
        is_buff: false,
        stacks_duration: true,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::WraithForm,
        is_buff: false,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Buffer,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::PenNib,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 1,
    },
    ModifierDef {
        kind: ModifierKind::Magnetism,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::NoBlock,
        is_buff: false,
        stacks_duration: true,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Panache,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::SadisticNature,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Mayhem,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::TheBomb,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Regeneration,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::LoseStrength,
        is_buff: false,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::LoseDexterity,
        is_buff: false,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::DuplicateNextCardPlay,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
];

#[derive(Debug, Clone, Copy)]
pub struct Modifiers {
    pub stacks: [i16; MODIFIER_COUNT],
    pub is_new: [bool; MODIFIER_COUNT],
    pub active: u64, // bitmask
}

pub fn modifier_def(kind: ModifierKind) -> &'static ModifierDef {
    &MODIFIER_DEFS[kind as usize]
}

pub const ZERO_MODIFIERS: Modifiers = Modifiers {
    stacks: [0; MODIFIER_COUNT],
    is_new: [false; MODIFIER_COUNT],
    active: 0,
};

pub fn modifiers_new() -> Modifiers {
    ZERO_MODIFIERS
}

pub fn modifier_stacks(mods: &Modifiers, kind: ModifierKind) -> i16 {
    mods.stacks[kind as usize]
}

pub fn has_modifier(mods: &Modifiers, kind: ModifierKind) -> bool {
    mods.active & (1 << kind as u32) != 0
}

// Iterate the ModifierKinds set in an `active` bitmask. Takes the mask by value (a
// snapshot), so the source Modifiers may be mutated while iterating
pub fn active_modifier_kinds(active: u64) -> impl Iterator<Item = ModifierKind> {
    let mut bits = active;
    std::iter::from_fn(move || {
        if bits == 0 {
            return None;
        }
        let kind = modifier_kind_from_u8(bits.trailing_zeros() as u8);
        bits &= bits - 1;
        Some(kind)
    })
}

// Sum onto the existing stacks (0 if absent); below stacks_min removes, above stacks_max saturates
pub fn modifier_apply(mods: &mut Modifiers, kind: ModifierKind, stacks: i16) {
    let mod_def = modifier_def(kind);
    let idx = kind as usize;

    // Calculate new amount of stacks
    let stacks_new = if has_modifier(mods, kind) {
        mods.stacks[idx] + stacks
    } else {
        stacks
    };

    // Remove if below minimum stacks
    if stacks_new < mod_def.stacks_min {
        return modifier_remove(mods, kind);
    }

    // If not previously owned, create it with `is_new = True`
    if !has_modifier(mods, kind) {
        mods.is_new[idx] = true;
        mods.active |= 1 << kind as u32;
    }

    // Else, set new value
    mods.stacks[idx] = stacks_new.min(mod_def.stacks_max);
}

pub fn modifier_remove(mods: &mut Modifiers, kind: ModifierKind) {
    let idx = kind as usize;
    mods.stacks[idx] = 0;
    mods.is_new[idx] = false;
    mods.active &= !(1 << kind as u32);
}

pub fn modifier_tick(mods: &mut Modifiers) {
    for kind in active_modifier_kinds(mods.active) {
        let idx = kind as usize;
        let mod_def = modifier_def(kind);
        if mod_def.stacks_duration && !mods.is_new[idx] {
            mods.stacks[idx] -= 1;
            if mods.stacks[idx] < mod_def.stacks_min {
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

// Check that modifier definitons are in the correct order
const _: () = {
    let mut i = 0;
    while i < MODIFIER_COUNT {
        assert!(MODIFIER_DEFS[i].kind as usize == i);
        i += 1;
    }
};
