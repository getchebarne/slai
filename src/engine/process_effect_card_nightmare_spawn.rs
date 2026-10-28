use crate::consts::NIGHTMARE_COPIES;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardPile;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::push_entity;

// The targeted pending Nightmare adds its copies to the hand and leaves the pending list
pub fn process_effect_card_nightmare_spawn(id_target: Option<usize>, state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_card_nightmare_spawn outside the Combat frame"
    );
    let id_target = id_target.expect("CardNightmareSpawn requires id_target");
    let idx = state
        .combat
        .id_card_pile_nightmare
        .iter()
        .position(|&(id, _)| id == id_target)
        .expect("CardNightmareSpawn requires a pending snapshot");
    state.combat.id_card_pile_nightmare.remove(idx);

    state.effect_buf.clear();
    let card_template = state.entities[id_target];
    for _ in 0..NIGHTMARE_COPIES {
        let id_card = push_entity(&mut state.entities, card_template);
        state.effect_buf.push(Effect {
            kind: EffectKind::CardPlace {
                pile: CardPile::Hand,
            },
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }
    flush_effects_from_buf_to_queue_front(state);
}
