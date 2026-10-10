use crate::effect::CardPlay;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::PlaySource;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::Combat;

pub fn process_effect_card_play_from_draw_top(id_target: Option<usize>, state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_card_play_from_draw_top outside the Combat frame"
    );
    let Combat {
        id_card_draw,
        id_card_discard,
        energy,
        ..
    } = &mut state.combat;

    // Check if the draw pile is empty
    if id_card_draw.is_empty() {
        if id_card_discard.is_empty() {
            return;
        }

        // Executes in reverse:
        //     1. ShuffleDiscardPileIntoDrawPile
        //     2. CardPlayFromDrawTop (re-queued)
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardPlayFromDrawTop,
            id_source: None,
            target: Target::Direct(id_target),
        });
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ShuffleDiscardPileIntoDrawPile,
            id_source: None,
            target: Target::Direct(None),
        });
        return;
    }

    // The top Card leaves the draw pile now, with X and the draw pile's size fixed as it leaves; it waits behind
    // the plays already queued and is gated when it starts
    let id_card = id_card_draw.pop().unwrap();
    state.card_play_queue.push_back(CardPlay {
        id_card,
        id_target,
        play_source: PlaySource::DrawTop,
        energy: energy.energy_current,
        draw_pile_size: id_card_draw.len() as u16,
    });
}
