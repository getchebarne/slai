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
    Regeneration,
    LoseStrength,
    LoseDexterity,
    DuplicateNextCardPlay,
    Flight,
    Malleable,
    Barricade,
    Hex,
    Confusion,
    PainfulStabs,
    Minion,
}

const MODIFIER_COUNT: usize = ModifierKind::COUNT;

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

// The hook priority of a Modifier, Bomb or pending Nightmare that sets none
pub const PRIORITY_DEFAULT: u8 = 5;

#[derive(Debug, Clone, Copy)]
pub struct ModifierDef {
    pub kind: ModifierKind,
    pub priority: u8, // Turn-start and turn-end hooks fire by priority, lowest first, then by stamp
    pub is_buff: bool,
    pub stacks_duration: bool,
    pub stacks_min: i16,
    pub stacks_max: i16,
}

static MODIFIER_DEFS: [ModifierDef; MODIFIER_COUNT] = [
    ModifierDef {
        kind: ModifierKind::Accuracy,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::AfterImage,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Angry,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Artifact,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Asleep,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 1,
    },
    ModifierDef {
        kind: ModifierKind::Blur,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: true,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Burst,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Choke,
        priority: PRIORITY_DEFAULT,
        is_buff: false,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::CorpseExplosion,
        priority: PRIORITY_DEFAULT,
        is_buff: false,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::CurlUp,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Dexterity,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: -999, // Sums clamp here; Dexterity is removed at exactly 0 instead
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::DoubleDamage,
        priority: 6,
        is_buff: true,
        stacks_duration: true,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::DrawCardNextTurn,
        priority: 20,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Enrage,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Entangled,
        priority: PRIORITY_DEFAULT,
        is_buff: false,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Envenom,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Frail,
        priority: 10,
        is_buff: false,
        stacks_duration: true,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::InfiniteBlades,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Intangible,
        priority: 75,
        is_buff: true,
        stacks_duration: true,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Metallicize,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::ModeShift,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::NextTurnBlock,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 0, // A 0-block Dodge and Roll still lists it until the next turn start
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::NextTurnEnergy,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::NoDraw,
        priority: PRIORITY_DEFAULT,
        is_buff: false,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 1,
    },
    ModifierDef {
        kind: ModifierKind::NoxiousFumes,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Phantasmal,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: true,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::PlatedArmor,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Poison,
        priority: PRIORITY_DEFAULT,
        is_buff: false,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Retain,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Ritual,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Shackled,
        priority: PRIORITY_DEFAULT,
        is_buff: false,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::SharpHide,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Splittable,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 1,
    },
    ModifierDef {
        kind: ModifierKind::SporeCloud,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Strength,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: -999, // Sums clamp here; Strength is removed at exactly 0 instead
        stacks_max: 999,
    },
    ModifierDef {
        kind: ModifierKind::Thievery,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Thorns,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::ThousandCuts,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::ToolsOfTheTrade,
        priority: 25,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Vigor,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Vulnerable,
        priority: PRIORITY_DEFAULT,
        is_buff: false,
        stacks_duration: true,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Weak,
        priority: 99,
        is_buff: false,
        stacks_duration: true,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::WraithForm,
        priority: PRIORITY_DEFAULT,
        is_buff: false,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Buffer,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::PenNib,
        priority: 6,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 1,
    },
    ModifierDef {
        kind: ModifierKind::Magnetism,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::NoBlock,
        priority: PRIORITY_DEFAULT,
        is_buff: false,
        stacks_duration: true,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Panache,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::SadisticNature,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Mayhem,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Regeneration,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::LoseStrength,
        priority: PRIORITY_DEFAULT,
        is_buff: false,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::LoseDexterity,
        priority: PRIORITY_DEFAULT,
        is_buff: false,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::DuplicateNextCardPlay,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 0, // A spent stack lingers at 0 while its Card resolves
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Flight,
        priority: 50,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Malleable,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Barricade,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 1,
    },
    ModifierDef {
        kind: ModifierKind::Hex,
        priority: PRIORITY_DEFAULT,
        is_buff: false,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: i16::MAX,
    },
    ModifierDef {
        kind: ModifierKind::Confusion,
        priority: 0,
        is_buff: false,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 1,
    },
    ModifierDef {
        kind: ModifierKind::PainfulStabs,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 1,
    },
    ModifierDef {
        kind: ModifierKind::Minion,
        priority: PRIORITY_DEFAULT,
        is_buff: true,
        stacks_duration: false,
        stacks_min: 1,
        stacks_max: 1,
    },
];

#[derive(Debug, Clone, Copy)]
pub struct Modifiers {
    pub stacks: [i16; MODIFIER_COUNT],
    pub is_new: [bool; MODIFIER_COUNT],
    pub active: u128, // bitmask

    // Stamp from `Combat.modifier_seq_next`, taken when ModifierDelta makes the Modifier appear; stacking keeps it
    pub seq: [u32; MODIFIER_COUNT],
}

pub fn modifier_def(kind: ModifierKind) -> &'static ModifierDef {
    &MODIFIER_DEFS[kind as usize]
}

pub const MODIFIERS_ZERO: Modifiers = Modifiers {
    stacks: [0; MODIFIER_COUNT],
    is_new: [false; MODIFIER_COUNT],
    active: 0,
    seq: [0; MODIFIER_COUNT],
};

pub fn modifier_stacks(mods: &Modifiers, kind: ModifierKind) -> i16 {
    mods.stacks[kind as usize]
}

pub fn has_modifier(mods: &Modifiers, kind: ModifierKind) -> bool {
    mods.active & (1 << kind as u32) != 0
}

// Iterate the ModifierKinds set in an `active` bitmask. Takes the mask by value (a
// snapshot), so the source Modifiers may be mutated while iterating
pub fn active_modifier_kinds(active: u128) -> impl Iterator<Item = ModifierKind> {
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

// Sum onto the existing stacks (0 if absent); Strength/Dexterity are removed at 0, every other kind below stacks_min; a surviving sum clamps to [stacks_min, stacks_max]
pub fn modifier_apply(mods: &mut Modifiers, kind: ModifierKind, stacks: i16) {
    let mod_def = modifier_def(kind);
    let idx = kind as usize;

    // Calculate new amount of stacks
    let stacks_new = if has_modifier(mods, kind) {
        mods.stacks[idx].saturating_add(stacks)
    } else {
        stacks
    };

    let remove = match kind {
        ModifierKind::Strength | ModifierKind::Dexterity => stacks_new == 0,
        _ => stacks_new < mod_def.stacks_min,
    };
    if remove {
        return modifier_remove(mods, kind);
    }

    // If not previously owned, create it with `is_new = True`
    if !has_modifier(mods, kind) {
        mods.is_new[idx] = true;
        mods.active |= 1 << kind as u32;
    }

    // Else, set new value
    mods.stacks[idx] = stacks_new.clamp(mod_def.stacks_min, mod_def.stacks_max);
}

pub fn modifier_remove(mods: &mut Modifiers, kind: ModifierKind) {
    let idx = kind as usize;
    mods.stacks[idx] = 0;
    mods.is_new[idx] = false;
    mods.active &= !(1 << kind as u32);
    mods.seq[idx] = 0;
}

pub fn modifier_tick(mods: &mut Modifiers) {
    for mod_kind in active_modifier_kinds(mods.active) {
        let mod_idx = mod_kind as usize;
        let mod_def = modifier_def(mod_kind);
        if mod_def.stacks_duration && !mods.is_new[mod_idx] {
            mods.stacks[mod_idx] -= 1;
            if mods.stacks[mod_idx] < mod_def.stacks_min {
                modifier_remove(mods, mod_kind);
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
    mods.seq = [0; MODIFIER_COUNT];
}

// Check that modifier definitons are in the correct order
const _: () = {
    let mut idx = 0;
    while idx < MODIFIER_COUNT {
        assert!(MODIFIER_DEFS[idx].kind as usize == idx);
        idx += 1;
    }
};
