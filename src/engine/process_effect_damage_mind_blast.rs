use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;

// Damage equals draw-pile size at play time
pub fn process_effect_damage_mind_blast(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
) {
    let id_target = id_target.expect("DamageMindBlast requires id_target");
    state.effect_queue.push_front(Effect {
        kind: EffectKind::DamagePhysical {
            amount: state.id_pile_draw.len() as u16,
        },
        id_source,
        target: Target::Direct(Some(id_target)),
    });
}
