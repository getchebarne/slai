use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::opt;

// Search: the rolled Potions land on the Reward context, where the belt is
// interactive (discard-to-swap), matching the source's combatRewardScreen
const fn search(count: u8) -> [Effect; 2] {
    [
        // Consume first: the staged Reward overlays this frame until RoomExit
        EVENT_CONSUME_EFFECT,
        Effect {
            kind: EffectKind::RewardRollPotions {
                count,
                uniform: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ]
}
const OPTION_SEARCH_BASE: [Effect; 2] = search(3);
const OPTION_SEARCH_A15: [Effect; 2] = search(2);

// The source game offers no way to decline
static OPTIONS_BASE: &[&[Effect]] = &[opt(&OPTION_SEARCH_BASE)];
static OPTIONS_A15: &[&[Effect]] = &[opt(&OPTION_SEARCH_A15)];

pub fn options(ascension: u8) -> &'static [&'static [Effect]] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}
