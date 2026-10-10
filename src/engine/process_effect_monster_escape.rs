use crate::consts::MAX_SIZE_CARD_PILE_HAND;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::types::Combat;
use crate::utils::flush_effects_from_buf_to_queue_front;

// Mark dead WITHOUT firing the on-death hook chain
pub fn process_effect_monster_escape(id_target: Option<usize>, state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_monster_escape outside the Combat frame"
    );
    let Combat {
        id_monsters,
        id_card_pile_hand,
        id_card_pile_stasis,
        ..
    } = &mut state.combat;
    let id_target = id_target.expect("MonsterEscape requires id_target");
    state.entities[id_target].dead = true;
    state.effect_buf.clear();
    if let Some(slot) = id_monsters.iter().position(|slot| *slot == Some(id_target)) {
        id_monsters[slot] = None;

        // An escaping Stasis holder relinquishes its hostage (unreachable today)
        if let Some(id_card) = id_card_pile_stasis[slot].take() {
            state.effect_buf.push(Effect {
                kind: EffectKind::CardStasisReturn {
                    hand_full: id_card_pile_hand.len() >= MAX_SIZE_CARD_PILE_HAND,
                },
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });
        }
    }
    let any_alive = id_monsters.iter().any(|slot| slot.is_some());
    flush_effects_from_buf_to_queue_front(state);

    // A thief's escape lets a later Smoke Bomb keep the reward
    if has_modifier(&state.entities[id_target].modifiers, ModifierKind::Thievery) {
        state.combat.this_combat_thief_escaped = true;
    }

    // A fight counts as escaped only if no Monster is left and none fell in battle
    if !any_alive && !state.combat.this_combat_monster_died {
        state.combat.this_combat_escaped = true;
    }

    // The last escape closes the combat behind every queued effect and waiting Card play; no turn phase starts
    if !any_alive {
        state.phase_queue.clear();
        state.phase_queue.push_back(Effect {
            kind: EffectKind::CombatEnd {
                escaped_character: false,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
