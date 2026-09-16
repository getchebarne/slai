use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::utils::push_entity;

pub fn process_effect_card_duplicate(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardDuplicate requires id_target");
    let mut copy = state.entities[id_card];
    copy.card_bottled = false;
    copy.card_cost_override = None;
    let id_copy = push_entity(&mut state.entities, copy);
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardAdopt,
        id_source: None,
        target: Target::Direct(Some(id_copy)),
    });
}
