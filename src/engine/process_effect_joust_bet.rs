use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::JOUST_OWNER_WIN_CHANCE;
use crate::events::JOUST_PAYOUT_MURDERER;
use crate::events::JOUST_PAYOUT_OWNER;
use crate::events::JOUST_STAKE;
use crate::game::GameState;
use crate::types::DeltaSign;
use rand::Rng;

// The Joust: 50 gold on the line; the owner wins 30% of the time
pub fn process_effect_joust_bet(state: &mut GameState, on_owner: bool) {
    let owner_wins = state.rng.random_bool(JOUST_OWNER_WIN_CHANCE);
    let won = owner_wins == on_owner;

    // Executes in reverse: the stake leaves first, then any payout lands
    if won {
        let payout = if on_owner {
            JOUST_PAYOUT_OWNER
        } else {
            JOUST_PAYOUT_MURDERER
        };
        state.effect_queue.push_front(Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(payout),
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }
    state.effect_queue.push_front(Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(JOUST_STAKE),
        },
        id_source: None,
        target: Target::Direct(None),
    });
}
