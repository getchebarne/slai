// Monster definitions: move tables, spawning, and move selection.

use rand::Rng;

use crate::effect::{EffectTemplate, TargetKind};
use crate::modifier::{ModifierKind, modifier_apply, modifier_has};
use crate::state::{Intent, Monster, Move, vitals_new};
use crate::types::*;

// ---------------------------------------------------------------------------
// Move tables (static data)
// ---------------------------------------------------------------------------

// Jaw Worm
static JW_CHOMP_11: [EffectTemplate; 1] = [EffectTemplate::DamagePhysical { base: 11, target: TargetKind::Character }];
static JW_CHOMP_12: [EffectTemplate; 1] = [EffectTemplate::DamagePhysical { base: 12, target: TargetKind::Character }];
static JW_THRASH: [EffectTemplate; 2] = [
    EffectTemplate::DamagePhysical { base: 7, target: TargetKind::Character },
    EffectTemplate::BlockGain { amount: 5, target: TargetKind::Source },
];
static JW_BELLOW_3_6: [EffectTemplate; 2] = [
    EffectTemplate::ModifierGain { kind: ModifierKind::Strength, stacks: 3, target: TargetKind::Source },
    EffectTemplate::BlockGain { amount: 6, target: TargetKind::Source },
];
static JW_BELLOW_4_6: [EffectTemplate; 2] = [
    EffectTemplate::ModifierGain { kind: ModifierKind::Strength, stacks: 4, target: TargetKind::Source },
    EffectTemplate::BlockGain { amount: 6, target: TargetKind::Source },
];
static JW_BELLOW_5_9: [EffectTemplate; 2] = [
    EffectTemplate::ModifierGain { kind: ModifierKind::Strength, stacks: 5, target: TargetKind::Source },
    EffectTemplate::BlockGain { amount: 9, target: TargetKind::Source },
];

// Cultist
static CULT_DARK_STRIKE: [EffectTemplate; 1] = [EffectTemplate::DamagePhysical { base: 6, target: TargetKind::Character }];
static CULT_INCANTATION_3: [EffectTemplate; 1] = [EffectTemplate::ModifierGain { kind: ModifierKind::Ritual, stacks: 3, target: TargetKind::Source }];
static CULT_INCANTATION_4: [EffectTemplate; 1] = [EffectTemplate::ModifierGain { kind: ModifierKind::Ritual, stacks: 4, target: TargetKind::Source }];
static CULT_INCANTATION_5: [EffectTemplate; 1] = [EffectTemplate::ModifierGain { kind: ModifierKind::Ritual, stacks: 5, target: TargetKind::Source }];

// Fungi Beast
static FB_BITE: [EffectTemplate; 1] = [EffectTemplate::DamagePhysical { base: 6, target: TargetKind::Character }];
static FB_GROW_3: [EffectTemplate; 1] = [EffectTemplate::ModifierGain { kind: ModifierKind::Strength, stacks: 3, target: TargetKind::Source }];
static FB_GROW_4: [EffectTemplate; 1] = [EffectTemplate::ModifierGain { kind: ModifierKind::Strength, stacks: 4, target: TargetKind::Source }];
static FB_GROW_5: [EffectTemplate; 1] = [EffectTemplate::ModifierGain { kind: ModifierKind::Strength, stacks: 5, target: TargetKind::Source }];

// Louse Green
static LG_GROW_3: [EffectTemplate; 1] = [EffectTemplate::ModifierGain { kind: ModifierKind::Strength, stacks: 3, target: TargetKind::Source }];
static LG_GROW_4: [EffectTemplate; 1] = [EffectTemplate::ModifierGain { kind: ModifierKind::Strength, stacks: 4, target: TargetKind::Source }];

// The Guardian
static TG_CHARGING_UP: [EffectTemplate; 1] = [EffectTemplate::BlockGain { amount: 9, target: TargetKind::Source }];
static TG_FIERCE_BASH_32: [EffectTemplate; 1] = [EffectTemplate::DamagePhysical { base: 32, target: TargetKind::Character }];
static TG_FIERCE_BASH_36: [EffectTemplate; 1] = [EffectTemplate::DamagePhysical { base: 36, target: TargetKind::Character }];
static TG_VENT_STEAM: [EffectTemplate; 2] = [
    EffectTemplate::ModifierGain { kind: ModifierKind::Weak, stacks: 2, target: TargetKind::Character },
    EffectTemplate::ModifierGain { kind: ModifierKind::Vulnerable, stacks: 2, target: TargetKind::Character },
];
static TG_WHIRLWIND: [EffectTemplate; 4] = [
    EffectTemplate::DamagePhysical { base: 5, target: TargetKind::Character },
    EffectTemplate::DamagePhysical { base: 5, target: TargetKind::Character },
    EffectTemplate::DamagePhysical { base: 5, target: TargetKind::Character },
    EffectTemplate::DamagePhysical { base: 5, target: TargetKind::Character },
];
static TG_DEFENSIVE_MODE_3: [EffectTemplate; 1] = [EffectTemplate::ModifierGain { kind: ModifierKind::SharpHide, stacks: 3, target: TargetKind::Source }];
static TG_DEFENSIVE_MODE_4: [EffectTemplate; 1] = [EffectTemplate::ModifierGain { kind: ModifierKind::SharpHide, stacks: 4, target: TargetKind::Source }];
static TG_ROLL_ATTACK_9: [EffectTemplate; 1] = [EffectTemplate::DamagePhysical { base: 9, target: TargetKind::Character }];
static TG_ROLL_ATTACK_10: [EffectTemplate; 1] = [EffectTemplate::DamagePhysical { base: 10, target: TargetKind::Character }];
static TG_TWIN_SLAM: [EffectTemplate; 4] = [
    EffectTemplate::DamagePhysical { base: 8, target: TargetKind::Character },
    EffectTemplate::DamagePhysical { base: 8, target: TargetKind::Character },
    EffectTemplate::ModifierGain { kind: ModifierKind::ModeShift, stacks: 0, target: TargetKind::Source },
    EffectTemplate::ModifierRemove { kind: ModifierKind::SharpHide, target: TargetKind::Source },
];

// ---------------------------------------------------------------------------
// Move indices (constants for readability)
// ---------------------------------------------------------------------------

// Jaw Worm: 0 = Chomp, 1 = Bellow, 2 = Thrash
const JW_CHOMP_IDX: usize = 0;
const JW_BELLOW_IDX: usize = 1;
const JW_THRASH_IDX: usize = 2;

// Cultist: 0 = Incantation, 1 = Dark Strike
const CULT_INCANT: usize = 0;
const CULT_DARK: usize = 1;

// Fungi Beast: 0 = Bite, 1 = Grow
const FB_BITE_IDX: usize = 0;
const FB_GROW_IDX: usize = 1;

// Louse Green: 0 = Bite, 1 = Grow
const LG_BITE_IDX: usize = 0;
const LG_GROW_IDX: usize = 1;

// The Guardian: 0=ChargingUp, 1=FierceBash, 2=VentSteam, 3=Whirlwind, 4=DefensiveMode, 5=RollAttack, 6=TwinSlam
const TG_CHARGING: usize = 0;
const TG_FIERCE: usize = 1;
const TG_VENT: usize = 2;
const TG_WHIRL: usize = 3;
const TG_DEFENSIVE: usize = 4;
const TG_ROLL: usize = 5;
const TG_TWIN: usize = 6;

// ---------------------------------------------------------------------------
// Spawning
// ---------------------------------------------------------------------------

pub fn spawn_monster(name: MonsterName, ascension: u8, rng: &mut impl Rng) -> Monster {
    match name {
        MonsterName::JawWorm => spawn_jaw_worm(ascension, rng),
        MonsterName::Cultist => spawn_cultist(ascension, rng),
        MonsterName::FungiBeast => spawn_fungi_beast(ascension, rng),
        MonsterName::LouseGreen => spawn_louse_green(ascension, rng),
        MonsterName::TheGuardian => spawn_the_guardian(ascension),
        MonsterName::Dummy => spawn_dummy(),
    }
}

fn spawn_jaw_worm(asc: u8, rng: &mut impl Rng) -> Monster {
    let (hmin, hmax) = if asc < 7 { (40, 44) } else { (42, 46) };
    let hp = rng.random_range(hmin..=hmax);

    let chomp_effects: &'static [EffectTemplate] = if asc < 2 { &JW_CHOMP_11 } else { &JW_CHOMP_12 };
    let chomp_damage = if asc < 2 { 11 } else { 12 };

    let bellow_effects: &'static [EffectTemplate] = if asc < 2 {
        &JW_BELLOW_3_6
    } else if asc < 17 {
        &JW_BELLOW_4_6
    } else {
        &JW_BELLOW_5_9
    };

    Monster {
        name: MonsterName::JawWorm,
        kind: MonsterKind::Normal,
        vitals: vitals_new(hp, hp),
        moves: vec![
            Move { name: "Chomp", effects: chomp_effects, intent: Intent::attack(chomp_damage, 1) },
            Move { name: "Bellow", effects: bellow_effects, intent: Intent::buff_block() },
            Move { name: "Thrash", effects: &JW_THRASH, intent: Intent::attack_block(7, 1) },
        ],
        move_current: None,
        move_history: Vec::new(),
    }
}

fn spawn_cultist(asc: u8, rng: &mut impl Rng) -> Monster {
    let (hmin, hmax) = if asc < 7 { (48, 54) } else { (50, 56) };
    let hp = rng.random_range(hmin..=hmax);

    let incant_effects: &'static [EffectTemplate] = if asc < 2 {
        &CULT_INCANTATION_3
    } else if asc < 17 {
        &CULT_INCANTATION_4
    } else {
        &CULT_INCANTATION_5
    };

    Monster {
        name: MonsterName::Cultist,
        kind: MonsterKind::Normal,
        vitals: vitals_new(hp, hp),
        moves: vec![
            Move { name: "Incantation", effects: incant_effects, intent: Intent::buff() },
            Move { name: "Dark Strike", effects: &CULT_DARK_STRIKE, intent: Intent::attack(6, 1) },
        ],
        move_current: None,
        move_history: Vec::new(),
    }
}

fn spawn_fungi_beast(asc: u8, rng: &mut impl Rng) -> Monster {
    let (hmin, hmax) = if asc < 7 { (22, 28) } else { (24, 28) };
    let hp = rng.random_range(hmin..=hmax);

    let grow_effects: &'static [EffectTemplate] = if asc < 2 {
        &FB_GROW_3
    } else if asc < 17 {
        &FB_GROW_4
    } else {
        &FB_GROW_5
    };

    let mut vitals = vitals_new(hp, hp);
    modifier_apply(&mut vitals.modifiers, ModifierKind::SporeCloud, 2);
    // Spore cloud starts as not new
    vitals.modifiers.is_new[ModifierKind::SporeCloud as usize] = false;

    Monster {
        name: MonsterName::FungiBeast,
        kind: MonsterKind::Normal,
        vitals,
        moves: vec![
            Move { name: "Bite", effects: &FB_BITE, intent: Intent::attack(6, 1) },
            Move { name: "Grow", effects: grow_effects, intent: Intent::buff() },
        ],
        move_current: None,
        move_history: Vec::new(),
    }
}

fn spawn_louse_green(asc: u8, rng: &mut impl Rng) -> Monster {
    let (hmin, hmax) = if asc < 7 { (10, 15) } else { (11, 16) };
    let hp = rng.random_range(hmin..=hmax);

    let bite_damage: u16 = {
        let base = rng.random_range(5u16..=7u16);
        if asc >= 2 { base + 1 } else { base }
    };

    let grow_effects: &'static [EffectTemplate] = if asc < 17 { &LG_GROW_3 } else { &LG_GROW_4 };

    // Curl up modifier (stored as a special initial modifier)
    let curl_stacks: i16 = if asc < 7 {
        rng.random_range(3..=7)
    } else if asc < 17 {
        rng.random_range(4..=8)
    } else {
        rng.random_range(9..=12)
    };

    let vitals = vitals_new(hp, hp);
    // Note: CurlUp is not in our ModifierKind enum. For now we skip it since
    // it's commented out in the Python code too. The Python uses it directly
    // on the modifier_map but we don't have a CurlUp kind yet.
    // TODO: add CurlUp modifier kind if needed
    let _ = curl_stacks;

    // Louse bite damage is rolled at spawn, so we need a dynamic move
    // We'll create a fresh Move with the rolled damage as a static leak
    // This is the one case where we need a non-static effects slice.
    // For now, store the bite damage on the Intent and use a single-element vec.
    let bite_effects: Vec<EffectTemplate> = vec![
        EffectTemplate::DamagePhysical { base: bite_damage, target: TargetKind::Character },
    ];
    // Leak to get 'static lifetime (these are created once per monster spawn)
    let bite_effects_static: &'static [EffectTemplate] = Box::leak(bite_effects.into_boxed_slice());

    Monster {
        name: MonsterName::LouseGreen,
        kind: MonsterKind::Normal,
        vitals,
        moves: vec![
            Move { name: "Bite", effects: bite_effects_static, intent: Intent::attack(bite_damage, 1) },
            Move { name: "Grow", effects: grow_effects, intent: Intent::buff() },
        ],
        move_current: None,
        move_history: Vec::new(),
    }
}

fn spawn_the_guardian(asc: u8) -> Monster {
    let hp: u16 = if asc < 9 { 240 } else { 250 };

    let fierce_effects: &'static [EffectTemplate] = if asc < 4 { &TG_FIERCE_BASH_32 } else { &TG_FIERCE_BASH_36 };
    let fierce_damage = if asc < 4 { 32 } else { 36 };

    let defensive_effects: &'static [EffectTemplate] = if asc < 19 { &TG_DEFENSIVE_MODE_3 } else { &TG_DEFENSIVE_MODE_4 };
    let roll_effects: &'static [EffectTemplate] = if asc < 4 { &TG_ROLL_ATTACK_9 } else { &TG_ROLL_ATTACK_10 };
    let roll_damage = if asc < 4 { 9 } else { 10 };

    let mode_shift_stacks: i16 = if asc < 9 { 30 } else if asc < 19 { 35 } else { 40 };

    let mut vitals = vitals_new(hp, hp);
    modifier_apply(&mut vitals.modifiers, ModifierKind::ModeShift, mode_shift_stacks);
    vitals.modifiers.is_new[ModifierKind::ModeShift as usize] = false;

    Monster {
        name: MonsterName::TheGuardian,
        kind: MonsterKind::Boss,
        vitals,
        moves: vec![
            Move { name: "Charging Up", effects: &TG_CHARGING_UP, intent: Intent::block_only() },
            Move { name: "Fierce Bash", effects: fierce_effects, intent: Intent::attack(fierce_damage, 1) },
            Move { name: "Vent Steam", effects: &TG_VENT_STEAM, intent: Intent::debuff() },
            Move { name: "Whirlwind", effects: &TG_WHIRLWIND, intent: Intent::attack(5, 4) },
            Move { name: "Defensive Mode", effects: defensive_effects, intent: Intent::buff() },
            Move { name: "Roll Attack", effects: roll_effects, intent: Intent::attack(roll_damage, 1) },
            Move { name: "Twin Slam", effects: &TG_TWIN_SLAM, intent: Intent::attack(8, 2) },
        ],
        move_current: None,
        move_history: Vec::new(),
    }
}

fn spawn_dummy() -> Monster {
    Monster {
        name: MonsterName::Dummy,
        kind: MonsterKind::Normal,
        vitals: vitals_new(50, 50),
        moves: vec![
            Move { name: "Attack", effects: &[EffectTemplate::DamagePhysical { base: 12, target: TargetKind::Character }], intent: Intent::attack(12, 1) },
            Move { name: "Defend", effects: &[EffectTemplate::BlockGain { amount: 12, target: TargetKind::Source }], intent: Intent::block_only() },
        ],
        move_current: None,
        move_history: Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Move selection
// ---------------------------------------------------------------------------

pub fn select_next_move(monster: &Monster, _ascension: u8, rng: &mut impl Rng) -> usize {
    match monster.name {
        MonsterName::JawWorm => jaw_worm_next_move(monster, rng),
        MonsterName::Cultist => cultist_next_move(monster),
        MonsterName::FungiBeast => fungi_beast_next_move(monster, rng),
        MonsterName::LouseGreen => louse_green_next_move(monster, rng),
        MonsterName::TheGuardian => guardian_next_move(monster),
        MonsterName::Dummy => dummy_next_move(monster, rng),
    }
}

fn last_move(monster: &Monster) -> Option<usize> {
    monster.move_history.last().copied()
}

fn last_n_moves(monster: &Monster, n: usize) -> Option<&[usize]> {
    let h = &monster.move_history;
    if h.len() >= n {
        Some(&h[h.len() - n..])
    } else {
        None
    }
}

fn jaw_worm_next_move(monster: &Monster, rng: &mut impl Rng) -> usize {
    if monster.move_current.is_none() {
        return JW_CHOMP_IDX;
    }

    let num: u32 = rng.random_range(0..99);
    if num < 25 {
        if last_move(monster) == Some(JW_CHOMP_IDX) {
            return if rng.random_bool(0.5625) { JW_BELLOW_IDX } else { JW_THRASH_IDX };
        }
        return JW_CHOMP_IDX;
    } else if num < 55 {
        if last_n_moves(monster, 2) == Some(&[JW_THRASH_IDX, JW_THRASH_IDX]) {
            return if rng.random_bool(0.357) { JW_CHOMP_IDX } else { JW_BELLOW_IDX };
        }
        return JW_THRASH_IDX;
    } else {
        if last_move(monster) == Some(JW_BELLOW_IDX) {
            return if rng.random_bool(0.416) { JW_CHOMP_IDX } else { JW_THRASH_IDX };
        }
        return JW_BELLOW_IDX;
    }
}

fn cultist_next_move(monster: &Monster) -> usize {
    if monster.move_current.is_none() {
        CULT_INCANT
    } else {
        CULT_DARK
    }
}

fn fungi_beast_next_move(monster: &Monster, rng: &mut impl Rng) -> usize {
    let num: u32 = rng.random_range(0..99);
    if num < 60 {
        if last_n_moves(monster, 2) == Some(&[FB_BITE_IDX, FB_BITE_IDX]) {
            return FB_GROW_IDX;
        }
        return FB_BITE_IDX;
    }
    if last_move(monster) == Some(FB_GROW_IDX) {
        return FB_BITE_IDX;
    }
    FB_GROW_IDX
}

fn louse_green_next_move(monster: &Monster, rng: &mut impl Rng) -> usize {
    let num: u32 = rng.random_range(0..99);
    if num < 60 {
        if last_n_moves(monster, 2) == Some(&[LG_BITE_IDX, LG_BITE_IDX]) {
            return LG_GROW_IDX;
        }
        return LG_BITE_IDX;
    }
    if last_move(monster) == Some(LG_GROW_IDX) {
        return LG_BITE_IDX;
    }
    LG_GROW_IDX
}

fn guardian_next_move(monster: &Monster) -> usize {
    if monster.move_current.is_none() {
        return TG_CHARGING;
    }

    let has_mode_shift = modifier_has(&monster.vitals.modifiers, ModifierKind::ModeShift);

    if has_mode_shift {
        // Offensive mode
        match last_move(monster).unwrap() {
            TG_CHARGING => TG_FIERCE,
            TG_FIERCE => TG_VENT,
            TG_VENT => TG_WHIRL,
            TG_WHIRL => TG_CHARGING,
            TG_TWIN => TG_WHIRL,
            _ => unreachable!("invalid guardian move in offensive mode"),
        }
    } else {
        // Defensive mode
        let has_sharp_hide = modifier_has(&monster.vitals.modifiers, ModifierKind::SharpHide);
        if has_sharp_hide {
            match last_move(monster).unwrap() {
                TG_DEFENSIVE => TG_ROLL,
                TG_ROLL => TG_TWIN,
                _ => unreachable!("invalid guardian move in defensive mode"),
            }
        } else {
            TG_DEFENSIVE
        }
    }
}

fn dummy_next_move(monster: &Monster, rng: &mut impl Rng) -> usize {
    if monster.move_current.is_none() {
        return rng.random_range(0..2);
    }
    match last_move(monster).unwrap() {
        0 => 1,
        1 => 0,
        _ => unreachable!(),
    }
}
