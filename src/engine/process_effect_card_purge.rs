use crate::effect::Amount;
use crate::effect::EFFECT_DU_VU_DOLL_RECOUNT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::has_relic;

pub fn process_effect_card_purge(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardPurge requires id_target");

    // Parasite costs 3 max HP when removed from the master deck
    if state.entities[id_card].card_name == CardName::Parasite {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::MaxHealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(3),
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }
    if let Some(pos) = state.id_card_deck.iter().position(|&id| id == id_card) {
        state.id_card_deck.remove(pos);

        // Du-Vu Doll: a Card purged from the deck recounts its Curses
        if has_relic(&state.id_relics, RelicName::DuVuDoll) {
            state.effect_queue.push_front(EFFECT_DU_VU_DOLL_RECOUNT);
        }
    }
}
