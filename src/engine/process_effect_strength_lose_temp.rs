use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::utils::flush_effects_from_buf_to_queue_front;

pub fn process_effect_strength_lose_temp(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
    stacks: i16,
) {
    assert!(
        !id_targets.is_empty(),
        "StrengthLoseTemp requires id_target"
    );

    // Each Monster's Strength loss then its Shackled (if no Artifact), both applied by the effect's source, in the list's order
    state.effect_buf.clear();
    for &id_target in id_targets {
        // A corpse or a half-dead Monster takes no debuff
        let target = &state.entities[id_target];
        if target.dead || target.monster_half_dead {
            continue;
        }
        let artifact = has_modifier(&target.modifiers, ModifierKind::Artifact);
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Strength,
                stacks: -stacks,
            },
            id_source,
            target: Target::Direct(Some(id_target)),
        });
        if !artifact {
            state.effect_buf.push(Effect {
                kind: EffectKind::ModifierDelta {
                    kind: ModifierKind::Shackled,
                    stacks,
                },
                id_source,
                target: Target::Direct(Some(id_target)),
            });
        }
    }
    flush_effects_from_buf_to_queue_front(state);
}
