use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardKind;
use crate::types::Combat;

// `damage` per Skill in hand (Flechettes itself already moved to discard)
pub fn process_effect_damage_flechettes(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
    damage: u16,
) {
    assert!(
        state.combat.active,
        "process_effect_damage_flechettes outside the Combat frame"
    );
    let Combat {
        id_card_pile_hand, ..
    } = &mut state.combat;
    let id_target = id_target.expect("DamageFlechettes requires id_target");
    let num_skills_in_hand = id_card_pile_hand
        .iter()
        .filter(|&&id| state.entities[id].card_kind == CardKind::Skill)
        .count();
    if num_skills_in_hand > 0 {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::DamagePhysical {
                amount: damage,
                instances: num_skills_in_hand as u16,
                lifesteal: false,
            },
            id_source,
            target: Target::Direct(Some(id_target)),
        });
    }
}
