use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::game::GameState;
use crate::types::Combat;

// The Bomb stamped `seq` counts down, and detonates for its own damage
pub fn process_effect_bomb_tick(state: &mut GameState, seq: u32) {
    assert!(
        state.combat.active,
        "process_effect_bomb_tick outside the Combat frame"
    );
    let Combat { bombs, .. } = &mut state.combat;
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
    let (_, damage, _) = bombs.remove(idx);
    state.effect_queue.push_front(Effect {
        kind: EffectKind::DamageDeal {
            amount: damage,
            lifesteal: false,
        },
        id_source: None,
        target: TARGET_MONSTERS_ALL,
    });
}
