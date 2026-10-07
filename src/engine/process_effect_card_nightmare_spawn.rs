use crate::consts::NIGHTMARE_COPIES;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardPile;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::push_entity;

pub fn process_effect_card_nightmare_spawn(state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_card_nightmare_spawn outside the Combat frame"
    );
    assert!(
        !state.combat.id_card_nightmares.is_empty(),
        "CardNightmareSpawn with no pending snapshot"
    );

    // Each pending Nightmare adds its copies in play order, so the first fills the hand first
    state.effect_buf.clear();
    for idx in 0..state.combat.id_card_nightmares.len() {
        let card_template = state.entities[state.combat.id_card_nightmares[idx]];
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
    }
    state.combat.id_card_nightmares.clear();
    flush_effects_from_buf_to_queue_front(state);
}
