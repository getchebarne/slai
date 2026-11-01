use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;

// A Strength gain that first cancels a negative Strength: the effect's source applies the stacks plus the Strength below 0 as it resolves, in one application
pub fn process_effect_strength_gain_past_negative(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
    stacks: i16,
) {
    let id_target = id_target.expect("StrengthGainPastNegative requires id_target");
    let modifiers = &state.entities[id_target].modifiers;
    let strength = if has_modifier(modifiers, ModifierKind::Strength) {
        modifier_stacks(modifiers, ModifierKind::Strength)
    } else {
        0
    };
    state.effect_queue.push_front(Effect {
        kind: EffectKind::ModifierDelta {
            kind: ModifierKind::Strength,
            stacks: stacks + (-strength).max(0),
        },
        id_source,
        target: Target::Direct(Some(id_target)),
    });
}
