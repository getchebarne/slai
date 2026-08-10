use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::OPTION_LEAVE;
use crate::events::make_entity_event_option;

const OPTION_OPEN: &[Effect] = &[
    Effect {
        kind: EffectKind::MausoleumOpen,
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

pub static OPTIONS: &[Entity] = &[
    make_entity_event_option(
        "[Open the coffin] Obtain a random Relic. Chance of becoming Cursed - Writhe.",
        OPTION_OPEN,
    ),
    OPTION_LEAVE,
];
