use rand::Rng;

use crate::cards::Card;
use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::types::ActorId;
use crate::utils::shuffle;

pub fn process_effect_combat_start(
    deck: &[Card],
    combat_cards: &mut Vec<Card>,
    draw_pile: &mut Vec<usize>,
    hand: &mut Vec<usize>,
    discard_pile: &mut Vec<usize>,
    exhaust_pile: &mut Vec<usize>,
    card_active: &mut Option<usize>,
    card_target: &mut Option<u8>,
    num_monsters: usize,
    rng: &mut impl Rng,
) -> ProcessEffectResult {
    *combat_cards = deck.to_vec();
    let num_cards = combat_cards.len();

    let mut idxs_innate: Vec<usize> = Vec::new();
    let mut idxs_other: Vec<usize> = Vec::new();
    for i in 0..num_cards {
        if combat_cards[i].innate {
            idxs_innate.push(i);
        } else {
            idxs_other.push(i);
        }
    }
    // Shuffle non-innate card indexes
    shuffle(&mut idxs_other, rng);

    // Assemble draw pile
    *draw_pile = idxs_innate;
    draw_pile.extend(idxs_other);

    // Intialize the remaining combat elements
    hand.clear();
    discard_pile.clear();
    exhaust_pile.clear();
    *card_active = None;
    *card_target = None;

    // Queue effects
    // Monsters' move updates
    let mut effects: Vec<Effect> = Vec::new();
    for i in 0..num_monsters {
        effects.push(Effect::MoveUpdate {
            monster_idx: i as u8,
        });
    }
    // Character's turn start
    effects.push(Effect::TurnStart {
        actor: ActorId::Character,
    });

    ProcessEffectResult::Continue {
        top: effects,
        bot: Vec::new(),
    }
}
