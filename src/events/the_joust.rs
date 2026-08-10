use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::make_entity_event_option;

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

pub static OPTIONS: &[Entity] = &[
    make_entity_event_option(
        "[Bet on the Murderer] Wager 50 Gold at 70% to win 100.",
        &OPTION_MURDERER,
    ),
    make_entity_event_option(
        "[Bet on the Owner] Wager 50 Gold at 30% to win 250.",
        &OPTION_OWNER,
    ),
];
