use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;

pub fn process_effect_strength_lose_temp(
    id_target: Option<usize>,
    state: &mut GameState,
    stacks: i16,
) {
    let id_target = id_target.expect("StrengthLoseTemp requires id_target");
    let modifiers = &state.entities[id_target].modifiers;

    // Executes in reverse:
    //     1. ModifierGain Strength
    //     2. ModifierGain Shackled (if no Artifact)
    if !has_modifier(modifiers, ModifierKind::Artifact) {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Shackled,
                stacks,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }
    state.effect_queue.push_front(Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Strength,
            stacks: -stacks,
        },
        id_source: None,
        target: Target::Direct(Some(id_target)),
    });
}
