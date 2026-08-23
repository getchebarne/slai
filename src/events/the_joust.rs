use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::opt;

pub const JOUST_STAKE: u16 = 50;
pub const JOUST_OWNER_WIN_CHANCE: f64 = 0.3;
pub const JOUST_PAYOUT_MURDERER: u16 = 100;
pub const JOUST_PAYOUT_OWNER: u16 = 250;

const fn bet(on_owner: bool) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::JoustBet { on_owner },
            id_source: None,
            target: Target::Direct(None),
        },
        EVENT_CONSUME_EFFECT,
    ]
}
const OPTION_MURDERER: [Effect; 2] = bet(false);
const OPTION_OWNER: [Effect; 2] = bet(true);

pub static OPTIONS: &[&[Effect]] = &[opt(&OPTION_MURDERER), opt(&OPTION_OWNER)];
