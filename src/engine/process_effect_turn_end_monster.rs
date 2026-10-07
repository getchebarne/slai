use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_stacks;
use crate::monsters::snake_plant;

// A Monster unwinds its per-turn kit at its turn end
pub fn process_effect_turn_end_monster(id_target: Option<usize>, state: &mut GameState) {
    let id_monster = id_target.expect("TurnEndMonster requires id_target");

    // Corpses don't unwind Shackled or gain Metallicize block
    if state.entities[id_monster].dead {
        return;
    }
    let modifiers = &state.entities[id_monster].modifiers;

    if has_modifier(modifiers, ModifierKind::Shackled) {
        let stacks = modifier_stacks(modifiers, ModifierKind::Shackled);
        // Executes in reverse:
        //     1. ModifierGain Strength
        //     2. ModifierRemove Shackled
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Shackled,
            },
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks,
            },
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
    }

    for kind in [ModifierKind::Metallicize, ModifierKind::PlatedArmor] {
        if has_modifier(modifiers, kind) {
            let stacks = modifier_stacks(modifiers, kind);
            state.effect_queue.push_front(Effect {
                kind: EffectKind::BlockGain {
                    amount: stacks as u16,
                },
                id_source: Some(id_monster),
                target: Target::Direct(Some(id_monster)),
            });
        }
    }

    // Malleable: per-hit escalation resets to base at the owner's turn end
    if has_modifier(modifiers, ModifierKind::Malleable) {
        let stacks = modifier_stacks(modifiers, ModifierKind::Malleable);
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Malleable,
                stacks: snake_plant::MALLEABLE_BASE - stacks,
            },
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
    }
}
