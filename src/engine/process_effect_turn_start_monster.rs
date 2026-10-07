use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::monsters::byrd;
use crate::utils::flush_effects_from_buf_to_queue_front;

pub fn process_effect_turn_start_monster(id_target: Option<usize>, state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_turn_start_monster outside the Combat frame"
    );
    let id_monster = id_target.expect("TurnStartMonster requires id_target");

    // Clear effect buffer
    state.effect_buf.clear();

    // Get mutable references
    let entity = &mut state.entities[id_monster];
    let modifiers = &mut entity.modifiers;

    // Block expires; Barricade keeps it
    let new_block = if has_modifier(modifiers, ModifierKind::Barricade) {
        entity.vitals.block
    } else {
        0
    };
    state.effect_buf.push(Effect {
        kind: EffectKind::BlockSet { amount: new_block },
        id_source: None,
        target: Target::Direct(Some(id_monster)),
    });

    // Flight: stacks refresh to the spawn value at the owner's turn start
    if has_modifier(modifiers, ModifierKind::Flight) {
        modifiers.stacks[ModifierKind::Flight as usize] = byrd::flight_stacks(state.ascension);
    }

    // Choke wears off at its Monster's turn start, before any Monster acts
    if has_modifier(modifiers, ModifierKind::Choke) {
        state.effect_buf.push(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Choke,
            },
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
    }

    flush_effects_from_buf_to_queue_front(state);
}
