use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
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
        id_card_hand,
        id_card_discard,
        this_combat_escaped,
        ..
    } = &mut state.combat;
    let id_target = id_target.expect("MonsterEscape requires id_target");
    state.entities[id_target].dead = true;
    if let Some(slot) = id_monsters.iter().position(|slot| *slot == Some(id_target)) {
        id_monsters[slot] = None;

        // An escaping Stasis holder relinquishes its hostage (unreachable today)
        release_stasis_card(slot, id_card_stasis, id_card_hand, id_card_discard);
    }
    *this_combat_escaped = true;

    let any_alive = id_monsters.iter().any(|slot| slot.is_some());
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
