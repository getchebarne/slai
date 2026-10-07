use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::types::Combat;
use crate::utils::release_stasis_card;

// Mark dead WITHOUT firing the on-death hook chain
pub fn process_effect_monster_escape(id_target: Option<usize>, state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_monster_escape outside the Combat frame"
    );
    let Combat {
        id_monsters,
        id_card_stasis,
        ..
    } = &mut state.combat;
    let id_target = id_target.expect("MonsterEscape requires id_target");
    state.entities[id_target].dead = true;
    if let Some(slot) = id_monsters.iter().position(|slot| *slot == Some(id_target)) {
        id_monsters[slot] = None;

        // An escaping Stasis holder relinquishes its hostage (unreachable today)
        release_stasis_card(
            slot,
            id_card_stasis,
            &state.entities,
            &mut state.effect_queue,
        );
    }
    let any_alive = id_monsters.iter().any(|slot| slot.is_some());

    // A thief's escape lets a later Smoke Bomb keep the reward
    if has_modifier(&state.entities[id_target].modifiers, ModifierKind::Thievery) {
        state.combat.this_combat_thief_escaped = true;
    }

    // A fight counts as escaped only if no Monster is left and none fell in battle
    if !any_alive && !state.combat.this_combat_monster_died {
        state.combat.this_combat_escaped = true;
    }
    if !any_alive {
        state.effect_queue.clear();
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CombatEnd {
                escaped_character: false,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
