use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::utils::unceasing_top_fires;

// Unceasing Top at the turn end's rest, behind the turn-end Relics, their picks and the Cards played meanwhile: an empty hand draws 1 before the landing pass
pub fn process_effect_unceasing_top_draw(state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_unceasing_top_draw outside the Combat frame"
    );
    if unceasing_top_fires(state) {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardDraw { count: 1 },
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
