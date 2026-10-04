use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::Combat;

// Damage equals draw-pile size at play time
pub fn process_effect_damage_mind_blast(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
) {
    assert!(
        state.combat.active,
        "process_effect_damage_mind_blast outside the Combat frame"
    );
    let Combat { id_card_draw, .. } = &mut state.combat;
    let id_target = id_target.expect("DamageMindBlast requires id_target");
    state.effect_queue.push_front(Effect {
        kind: EffectKind::DamagePhysical {
            amount: id_card_draw.len() as u16,
            lifesteal: false,
        },
        id_source,
        target: Target::Direct(Some(id_target)),
    });
}
