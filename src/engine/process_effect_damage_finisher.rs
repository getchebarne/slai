use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;

// Subtract 1 because card_play increments the counter before this effect fires
pub fn process_effect_damage_finisher(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
    damage: u16,
) {
    let id_target = id_target.expect("DamageFinisher requires id_target");
    let num_attacks = state.this_turn_attacks_played.saturating_sub(1);
    for _ in 0..num_attacks {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::DamagePhysical { amount: damage },
            id_source,
            target: Target::Direct(Some(id_target)),
        });
    }
}
