use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;

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
const OPTION_LEAVE: &[Effect] = &[EVENT_CONSUME_EFFECT];

const OPTIONS_BASE: &[(&str, &[Effect])] = &[
    (
        "[Reach Inside] Lose 3 HP. 25% chance for a Relic.",
        &OPTIONS_REACH_BASE[0],
    ),
    (
        "[Reach Inside] Lose 4 HP. 35% chance for a Relic.",
        &OPTIONS_REACH_BASE[1],
    ),
    (
        "[Reach Inside] Lose 5 HP. 45% chance for a Relic.",
        &OPTIONS_REACH_BASE[2],
    ),
    (
        "[Reach Inside] Lose 6 HP. 55% chance for a Relic.",
        &OPTIONS_REACH_BASE[3],
    ),
    (
        "[Reach Inside] Lose 7 HP. 65% chance for a Relic.",
        &OPTIONS_REACH_BASE[4],
    ),
    (
        "[Reach Inside] Lose 8 HP. 75% chance for a Relic.",
        &OPTIONS_REACH_BASE[5],
    ),
    (
        "[Reach Inside] Lose 9 HP. 85% chance for a Relic.",
        &OPTIONS_REACH_BASE[6],
    ),
    (
        "[Reach Inside] Lose 10 HP. 95% chance for a Relic.",
        &OPTIONS_REACH_BASE[7],
    ),
    (
        "[Reach Inside] Lose 11 HP. 105% chance for a Relic.",
        &OPTIONS_REACH_BASE[8],
    ),
    ("[Leave] Nothing happens.", OPTION_LEAVE),
];
const OPTIONS_A15: &[(&str, &[Effect])] = &[
    (
        "[Reach Inside] Lose 5 HP. 25% chance for a Relic.",
        &OPTIONS_REACH_A15[0],
    ),
    (
        "[Reach Inside] Lose 6 HP. 35% chance for a Relic.",
        &OPTIONS_REACH_A15[1],
    ),
    (
        "[Reach Inside] Lose 7 HP. 45% chance for a Relic.",
        &OPTIONS_REACH_A15[2],
    ),
    (
        "[Reach Inside] Lose 8 HP. 55% chance for a Relic.",
        &OPTIONS_REACH_A15[3],
    ),
    (
        "[Reach Inside] Lose 9 HP. 65% chance for a Relic.",
        &OPTIONS_REACH_A15[4],
    ),
    (
        "[Reach Inside] Lose 10 HP. 75% chance for a Relic.",
        &OPTIONS_REACH_A15[5],
    ),
    (
        "[Reach Inside] Lose 11 HP. 85% chance for a Relic.",
        &OPTIONS_REACH_A15[6],
    ),
    (
        "[Reach Inside] Lose 12 HP. 95% chance for a Relic.",
        &OPTIONS_REACH_A15[7],
    ),
    // Base game also reads 105%. I'm innocent
    (
        "[Reach Inside] Lose 13 HP. 105% chance for a Relic.",
        &OPTIONS_REACH_A15[8],
    ),
    ("[Leave] Nothing happens.", OPTION_LEAVE),
];

pub fn options(ascension: u8) -> &'static [(&'static str, &'static [Effect])] {
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
        _ => unreachable!("scrap ooze option out of range: {idx}"),
    }
}
