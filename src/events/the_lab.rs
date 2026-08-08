use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RewardSource;
use crate::effect::Target;
use crate::entity::Entity;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::make_entity_event_option;

// Search: the rolled Potions land on the reward screen, where the belt is
// interactive (discard-to-swap), matching the source's combatRewardScreen
const fn search(count: u8) -> [Effect; 2] {
    [
        // Consume first: RewardRollPotions replaces this event with Mode::Reward
        EVENT_CONSUME_EFFECT,
        Effect {
            kind: EffectKind::RewardRoll {
                source: RewardSource::Potions {
                    count,
                    uniform: false,
                },
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ]
}
const OPTION_SEARCH_BASE: [Effect; 2] = search(3);
const OPTION_SEARCH_A15: [Effect; 2] = search(2);

// The source game offers no way to decline
static OPTIONS_BASE: &[Entity] = &[make_entity_event_option(
    "[Search] Obtain 3 random potions.",
    &OPTION_SEARCH_BASE,
)];
static OPTIONS_A15: &[Entity] = &[make_entity_event_option(
    "[Search] Obtain 2 random potions.",
    &OPTION_SEARCH_A15,
)];

pub fn options(ascension: u8) -> &'static [Entity] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}
