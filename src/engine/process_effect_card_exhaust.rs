use crate::cards::get_random_cards;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardColor;
use crate::types::RelicName;
use crate::utils::has_relic;

pub fn process_effect_card_exhaust(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardExhaust requires id_target");
    if let Some(pos) = state.id_hand.iter().position(|&v| v == id_card) {
        state.id_hand.remove(pos);
    }
    state.id_pile_exhaust.push(id_card);

    // Dead Branch: every exhaust conjures a random Silent card into the hand
    // (all green cards are rewardable, so no kind/rarity filter is needed)
    if has_relic(&state.id_relics, RelicName::DeadBranch) {
        let card_name =
            get_random_cards(CardColor::Green, None, None, &[], 1, &mut state.rng)[0].card_name;
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CardAddToHand {
                card_name,
                count: 1,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
