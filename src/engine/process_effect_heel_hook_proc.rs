use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::engine::first_target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::types::DeltaSign;

// The hit, then +1 energy +1 Card if the target was Weak before the hit
pub fn process_effect_heel_hook_proc(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
    damage: u16,
) {
    let id_target = first_target(
        state,
        EffectKind::HeelHookProc { damage },
        id_source,
        id_targets,
    );
    let id_target = id_target.expect("HeelHookProc requires id_target");

    // Executes in reverse:
    //     1. DamagePhysical
    //     2. EnergyDelta
    //     3. CardDraw
    if has_modifier(&state.entities[id_target].modifiers, ModifierKind::Weak) {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardDraw { count: 1 },
            id_source: None,
            target: Target::Direct(None),
        });
        state.effect_queue.push_front(Effect {
            kind: EffectKind::EnergyDelta {
                sign: DeltaSign::Gain,
                amount: 1,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }
    state.effect_queue.push_front(Effect {
        kind: EffectKind::DamagePhysical {
            amount: damage,
            instances: 1,
            lifesteal: false,
        },
        id_source,
        target: Target::Direct(Some(id_target)),
    });
}
