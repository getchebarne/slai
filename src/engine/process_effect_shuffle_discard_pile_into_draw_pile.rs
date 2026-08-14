use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::relics::trigger_relic_counter;
use crate::types::DeltaSign;
use crate::types::Frame;
use crate::types::RelicName;
use crate::utils::frame_top_mut;
use crate::utils::has_relic;
use crate::utils::shuffle;

pub fn process_effect_shuffle_discard_pile_into_draw_pile(state: &mut GameState) {
    let Frame::Combat {
        id_pile_draw,
        id_pile_discard,
        ..
    } = frame_top_mut(&mut state.frame_stack)
    else {
        unreachable!("process_effect_shuffle_discard_pile_into_draw_pile outside the Combat frame")
    };
    id_pile_draw.append(id_pile_discard);
    shuffle(&mut id_pile_draw[..], &mut state.rng);

    // Abacus: reshuffling the discard pile grants 6 block
    // Relic-sourced block: id_source None skips Dex / Frail scaling
    if has_relic(&state.id_relics, RelicName::Abacus) {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::BlockGain { amount: 6 },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Persistent reshuffle counter; every 3rd fires
    if trigger_relic_counter(RelicName::Sundial, 3, &state.id_relics, &mut state.entities) {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::EnergyDelta {
                sign: DeltaSign::Gain,
                amount: 2,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
