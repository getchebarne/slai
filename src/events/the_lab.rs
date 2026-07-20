use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;

// Search: the rolled potions land on the reward screen, where the belt is
// interactive (discard-to-swap), matching the source's combatRewardScreen
const fn search(count: u8) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::RewardRollPotions { count },
            id_source: None,
            target: Target::Direct(None),
        },
        EVENT_CONSUME_EFFECT,
    ]
}
static OPTION_SEARCH_BASE: [Effect; 2] = search(3);
static OPTION_SEARCH_A15: [Effect; 2] = search(2);

// The source game offers no way to decline
const LABELS_BASE: &[&str] = &["[Search] Obtain 3 random potions."];
const LABELS_A15: &[&str] = &["[Search] Obtain 2 random potions."];

pub fn labels(ascension: u8) -> &'static [&'static str] {
    if ascension < 15 {
        LABELS_BASE
    } else {
        LABELS_A15
    }
}

pub fn push_option_effects(buf: &mut Vec<Effect>, ascension: u8, idx: usize) {
    buf.extend_from_slice(match idx {
        0 if ascension < 15 => &OPTION_SEARCH_BASE,
        0 => &OPTION_SEARCH_A15,
        _ => unreachable!("the lab option out of range: {idx}"),
    });
}
