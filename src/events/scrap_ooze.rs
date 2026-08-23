use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::OPTION_LEAVE;
use crate::events::opt;

// Reach in; +1 HP and +10% per miss until the 105% rung, which cannot fail
const fn reach(dmg: u16, chance: u8, advance_on_miss: bool) -> [Effect; 1] {
    [Effect {
        kind: EffectKind::ScrapOozeReach {
            dmg,
            chance,
            advance_on_miss,
        },
        id_source: None,
        target: Target::Direct(None),
    }]
}
const OPTIONS_REACH_BASE: [[Effect; 1]; 9] = [
    reach(3, 25, true),
    reach(4, 35, true),
    reach(5, 45, true),
    reach(6, 55, true),
    reach(7, 65, true),
    reach(8, 75, true),
    reach(9, 85, true),
    reach(10, 95, true),
    reach(11, 105, false),
];

// Base damage 3 -> 5 at A15
const OPTIONS_REACH_A15: [[Effect; 1]; 9] = [
    reach(5, 25, true),
    reach(6, 35, true),
    reach(7, 45, true),
    reach(8, 55, true),
    reach(9, 65, true),
    reach(10, 75, true),
    reach(11, 85, true),
    reach(12, 95, true),
    reach(13, 105, false),
];

// Leave
static OPTIONS_BASE: &[&[Effect]] = &[
    opt(&OPTIONS_REACH_BASE[0]),
    opt(&OPTIONS_REACH_BASE[1]),
    opt(&OPTIONS_REACH_BASE[2]),
    opt(&OPTIONS_REACH_BASE[3]),
    opt(&OPTIONS_REACH_BASE[4]),
    opt(&OPTIONS_REACH_BASE[5]),
    opt(&OPTIONS_REACH_BASE[6]),
    opt(&OPTIONS_REACH_BASE[7]),
    opt(&OPTIONS_REACH_BASE[8]),
    OPTION_LEAVE,
];
static OPTIONS_A15: &[&[Effect]] = &[
    opt(&OPTIONS_REACH_A15[0]),
    opt(&OPTIONS_REACH_A15[1]),
    opt(&OPTIONS_REACH_A15[2]),
    opt(&OPTIONS_REACH_A15[3]),
    opt(&OPTIONS_REACH_A15[4]),
    opt(&OPTIONS_REACH_A15[5]),
    opt(&OPTIONS_REACH_A15[6]),
    opt(&OPTIONS_REACH_A15[7]),
    // Base game also reads 105%. I'm innocent
    opt(&OPTIONS_REACH_A15[8]),
    OPTION_LEAVE,
];

pub fn options(ascension: u8) -> &'static [&'static [Effect]] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}

pub fn option_available(attempts: u8, idx: usize) -> bool {
    match idx {
        0..=8 => idx as u8 == attempts,
        9 => true,
        _ => unreachable!("Scrap ooze option out of range: {idx}"),
    }
}
