use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::Combat;
use crate::utils::flush_effects_from_buf_to_queue_front;

// Each Bomb counts down on its own and detonates for its own damage
pub fn process_effect_bomb_tick(state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_bomb_tick outside the Combat frame"
    );
    let Combat {
        id_monsters, bombs, ..
    } = &mut state.combat;

    // Clear Effect buffer
    state.effect_buf.clear();

    let mut idx = 0;
    while idx < bombs.len() {
        // Decrease timer
        if bombs[idx].0 > 1 {
            bombs[idx].0 -= 1;
            idx += 1;
            continue;
        }

        // Explode
        let (_, damage) = bombs.remove(idx);
        for id_monster in id_monsters.iter().flatten().copied() {
            state.effect_buf.push(Effect {
                kind: EffectKind::DamageDeal {
                    amount: damage,
                    lifesteal: false,
                },
                id_source: None,
                target: Target::Direct(Some(id_monster)),
            });
        }
    }
    flush_effects_from_buf_to_queue_front(state);
}
