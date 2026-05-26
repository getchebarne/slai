use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;

pub fn process_effect_card_duplicate(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardDuplicate requires id_target");
    let card = &state.entities[id_card];
    let card_name = card.card_name;
    let upgraded = card.card_upgraded;
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardAddToDeck {
            card_name,
            upgraded,
        },
        id_source: None,
        target: Target::Direct(None),
    });
}
