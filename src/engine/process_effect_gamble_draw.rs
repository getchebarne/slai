use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;

// Gamble's draw: one Card per discard made since it began
pub fn process_effect_gamble_draw(state: &mut GameState, discards_before: u16) {
    assert!(
        state.combat.active,
        "process_effect_gamble_draw outside the Combat frame"
    );
    let count = state
        .combat
        .this_turn_discards
        .saturating_sub(discards_before);
    if count > 0 {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardDraw { count },
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
