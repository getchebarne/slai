use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::make_entity_event_option;

// Search: escalating elite-return chance; the AdventurerSearch processor draws
// the loot, advances the search count, and consumes after the third find
const OPTION_SEARCH: &[Effect] = &[Effect {
    kind: EffectKind::AdventurerSearch,
    id_source: None,
    target: Target::Direct(None),
}];

// Escape
const OPTION_ESCAPE: &[Effect] = &[EVENT_CONSUME_EFFECT];

pub static OPTIONS: &[Entity] = &[
    make_entity_event_option(
        "[Search] Find loot; whatever killed the adventurer may return.",
        OPTION_SEARCH,
    ),
    make_entity_event_option("[Escape] Leave with what you found.", OPTION_ESCAPE),
];
