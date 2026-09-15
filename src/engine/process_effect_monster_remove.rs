use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::Combat;
use crate::utils::release_stasis_card;

pub fn process_effect_monster_remove(id_target: Option<usize>, state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_monster_remove outside the Combat frame"
    );
    let Combat {
        id_monsters,
        id_card_stasis,
        id_card_hand,
        id_card_discard,
        ..
    } = &mut state.combat;
    let id_target = id_target.expect("MonsterRemove requires id_target");

    // Mark as dead
    state.entities[id_target].dead = true;

    // Remove it
    if let Some(slot) = id_monsters.iter().position(|slot| *slot == Some(id_target)) {
        id_monsters[slot] = None;
        release_stasis_card(slot, id_card_stasis, id_card_hand, id_card_discard);
    }

    // Check for combat end
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
