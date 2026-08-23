use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::OPTION_LEAVE;
use crate::events::opt;

const OPTION_OPEN: &[Effect] = &[
    Effect {
        kind: EffectKind::MausoleumOpen,
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

pub static OPTIONS: &[&[Effect]] = &[opt(OPTION_OPEN), OPTION_LEAVE];
