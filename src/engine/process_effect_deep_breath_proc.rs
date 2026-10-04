use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;

// Deep Breath reshuffles only when the discard pile has Cards
pub fn process_effect_deep_breath_proc(state: &mut GameState) {
    if state.combat.id_card_discard.is_empty() {
        return;
    }
    state.effect_queue.push_front(Effect {
        kind: EffectKind::ShuffleDiscardPileIntoDrawPile,
        id_source: None,
        target: Target::Direct(None),
    });
}
