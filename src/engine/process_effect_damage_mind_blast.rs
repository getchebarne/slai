use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::Frame;
use crate::utils::frame_top_mut;

// Damage equals draw-pile size at play time
pub fn process_effect_damage_mind_blast(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
) {
    let Frame::Combat { id_pile_draw, .. } = frame_top_mut(&mut state.frame_stack) else {
        unreachable!("process_effect_damage_mind_blast outside the Combat frame")
    };
    let id_target = id_target.expect("DamageMindBlast requires id_target");
    state.effect_queue.push_front(Effect {
        kind: EffectKind::DamagePhysical {
            amount: id_pile_draw.len() as u16,
            lifesteal: false,
        },
        id_source,
        target: Target::Direct(Some(id_target)),
    });
}
