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
static OPTIONS_REACH_BASE: [[Effect; 1]; 9] = [
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
static OPTIONS_REACH_A15: [[Effect; 1]; 9] = [
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

const LABELS_BASE: &[&str] = &[
    "[Reach Inside] Lose 3 HP. 25% chance for a Relic.",
    "[Reach Inside] Lose 4 HP. 35% chance for a Relic.",
    "[Reach Inside] Lose 5 HP. 45% chance for a Relic.",
    "[Reach Inside] Lose 6 HP. 55% chance for a Relic.",
    "[Reach Inside] Lose 7 HP. 65% chance for a Relic.",
    "[Reach Inside] Lose 8 HP. 75% chance for a Relic.",
    "[Reach Inside] Lose 9 HP. 85% chance for a Relic.",
    "[Reach Inside] Lose 10 HP. 95% chance for a Relic.",
    "[Reach Inside] Lose 11 HP. 105% chance for a Relic.",
    "[Leave] Nothing happens.",
];
const LABELS_A15: &[&str] = &[
    "[Reach Inside] Lose 5 HP. 25% chance for a Relic.",
    "[Reach Inside] Lose 6 HP. 35% chance for a Relic.",
    "[Reach Inside] Lose 7 HP. 45% chance for a Relic.",
    "[Reach Inside] Lose 8 HP. 55% chance for a Relic.",
    "[Reach Inside] Lose 9 HP. 65% chance for a Relic.",
    "[Reach Inside] Lose 10 HP. 75% chance for a Relic.",
    "[Reach Inside] Lose 11 HP. 85% chance for a Relic.",
    "[Reach Inside] Lose 12 HP. 95% chance for a Relic.",
    "[Reach Inside] Lose 13 HP. 105% chance for a Relic.", // Base game also reads 105%. I'm innocent
    "[Leave] Nothing happens.",
];

pub fn labels(ascension: u8) -> &'static [&'static str] {
    if ascension < 15 {
        LABELS_BASE
    } else {
        LABELS_A15
    }
}

pub fn push_option_effects(buf: &mut Vec<Effect>, ascension: u8, idx: usize) {
    buf.extend_from_slice(match idx {
        0..=8 if ascension < 15 => &OPTIONS_REACH_BASE[idx],
        0..=8 => &OPTIONS_REACH_A15[idx],
        9 => OPTION_LEAVE,
        _ => unreachable!("scrap ooze option out of range: {idx}"),
    });
}

pub fn option_available(attempts: u8, idx: usize) -> bool {
    match idx {
        0..=8 => idx as u8 == attempts,
        9 => true,
        _ => unreachable!("scrap ooze option out of range: {idx}"),
    }
}
