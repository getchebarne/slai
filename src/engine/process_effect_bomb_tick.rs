use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::Combat;
use crate::utils::flush_effects_from_buf_to_queue_front;

// The Bomb stamped `seq` counts down, and detonates for its own damage
pub fn process_effect_bomb_tick(state: &mut GameState, seq: u32) {
    assert!(
        state.combat.active,
        "process_effect_bomb_tick outside the Combat frame"
    );
    let Combat {
        id_monsters, bombs, ..
    } = &mut state.combat;
    let idx = bombs
        .iter()
        .position(|&(_, _, seq_bomb)| seq_bomb == seq)
        .expect("BombTick requires a live Bomb");

    // Decrease timer
    if bombs[idx].0 > 1 {
        bombs[idx].0 -= 1;
        return;
    }

    // Explode
    state.effect_buf.clear();
    let (_, damage, _) = bombs.remove(idx);
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
    flush_effects_from_buf_to_queue_front(state);
}
