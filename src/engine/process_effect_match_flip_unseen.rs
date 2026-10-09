use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::EventName;
use crate::types::Focus;
use crate::utils::context_focus;

// Match and Keep!: reveals the next Card never flipped, then flips it as a seen one
pub fn process_effect_match_flip_unseen(state: &mut GameState) {
    assert!(
        context_focus(state) == Focus::Event,
        "MatchFlipUnseen outside the Event context"
    );
    assert!(
        state.event.name == EventName::MatchAndKeep,
        "MatchFlipUnseen outside a Match and Keep! event"
    );

    // The board is shuffled, so the next unseen Card is as good as any
    let id_card = state.event.id_match_unseen.remove(0);
    state.event.id_roll_card.push(id_card);
    state.effect_queue.push_front(Effect {
        kind: EffectKind::MatchFlipSeen,
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
}
