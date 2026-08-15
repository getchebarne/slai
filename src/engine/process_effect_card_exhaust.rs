use crate::cards::get_random_cards;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardColor;
use crate::types::CardPile;
use crate::types::Frame;
use crate::types::RelicName;
use crate::utils::frame_top_mut;
use crate::utils::has_relic;

pub fn process_effect_card_exhaust(id_target: Option<usize>, state: &mut GameState) {
    let Frame::Combat {
        id_hand,
        id_pile_exhaust,
        ..
    } = frame_top_mut(&mut state.frame_stack)
    else {
        unreachable!("process_effect_card_exhaust outside the Combat frame")
    };
    let id_card = id_target.expect("CardExhaust requires id_target");
    if let Some(pos) = id_hand.iter().position(|&v| v == id_card) {
        id_hand.remove(pos);
    }
    id_pile_exhaust.push(id_card);

    // Dead Branch: every exhaust conjures a random Silent Card into the hand
    // (all green Cards are rewardable, so no kind/rarity filter is needed)
    if has_relic(&state.id_relics, RelicName::DeadBranch) {
        let card_name =
            get_random_cards(CardColor::Green, None, None, &[], 1, &mut state.rng)[0].card_name;
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CardAdd {
                card_name,
                pile: CardPile::Hand,
                count: 1,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
