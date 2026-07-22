use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;

// Search: the rolled potions land on the reward screen, where the belt is
// interactive (discard-to-swap), matching the source's combatRewardScreen
const fn search(count: u8) -> [Effect; 2] {
    [
        // Consume first: RewardRollPotions replaces this event with Mode::Reward
        EVENT_CONSUME_EFFECT,
        Effect {
            kind: EffectKind::RewardRollPotions { count },
            id_source: None,
            target: Target::Direct(None),
        },
    ]
}
const OPTION_SEARCH_BASE: [Effect; 2] = search(3);
const OPTION_SEARCH_A15: [Effect; 2] = search(2);

// The source game offers no way to decline
const OPTIONS_BASE: &[(&str, &[Effect])] =
    &[("[Search] Obtain 3 random potions.", &OPTION_SEARCH_BASE)];
const OPTIONS_A15: &[(&str, &[Effect])] =
    &[("[Search] Obtain 2 random potions.", &OPTION_SEARCH_A15)];

pub fn options(ascension: u8) -> &'static [(&'static str, &'static [Effect])] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}
