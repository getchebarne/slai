use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardKind;
use crate::types::Mode;
use crate::utils::mode_top_mut;

// `damage` per Skill in hand (Flechettes itself already moved to discard)
pub fn process_effect_damage_flechettes(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
    damage: u16,
) {
    let Mode::Combat { id_hand, .. } = mode_top_mut(&mut state.mode_stack) else {
        unreachable!("process_effect_damage_flechettes outside Combat mode")
    };
    let id_target = id_target.expect("DamageFlechettes requires id_target");
    let num_skills_in_hand = id_hand
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
