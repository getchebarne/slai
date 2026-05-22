use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardKind;

// Deal `damage` per Skill currently in hand. Hand snapshot at handler time;
// Flechettes itself was already moved to discard by card_play so it can't be
// in hand
pub fn process_effect_flechettes_damage(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
    damage: u16,
) {
    let id_target = id_target.expect("FlechettesDamage requires id_target");
    let num_skills_in_hand = state
        .id_hand
        .iter()
        .filter(|&&id| state.entities[id].card_kind == CardKind::Skill)
        .count();
    for _ in 0..num_skills_in_hand {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::DamagePhysical { amount: damage },
            id_source,
            target: Target::Direct(Some(id_target)),
        });
    }
}
