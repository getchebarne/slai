use crate::game::GameState;
use crate::types::CardRarity;
use crate::types::Frame;
use crate::utils::frame_top_mut;
use rand::Rng;

// Bronze Orb's Stasis: exile a card from the draw pile (discard as fallback) until
// the orb dies. Prefers the highest rarity present; random among ties
pub fn process_effect_stasis_steal(id_source: Option<usize>, state: &mut GameState) {
    let Frame::Combat {
        id_monsters,
        id_stasis_cards,
        id_pile_draw,
        id_pile_discard,
        ..
    } = frame_top_mut(&mut state.frame_stack)
    else {
        unreachable!("process_effect_stasis_steal outside the Combat frame")
    };
    let id_source = id_source.expect("StasisSteal requires id_source");

    // Pick pile to steal from. Prefers draw over discard
    let id_pile: &mut Vec<usize> = if !id_pile_draw.is_empty() {
        id_pile_draw
    } else if !id_pile_discard.is_empty() {
        id_pile_discard
    } else {
        return;
    };

    // Highest rarity tier present wins; Special / Basic cards are the last resort
    let mut best_idx = 0;
    let mut best_rank = -1i8;
    let mut ties: u16 = 0;
    for (idx, &id_card) in id_pile.iter().enumerate() {
        let rank = match state.entities[id_card].card_rarity {
            CardRarity::Rare => 3,
            CardRarity::Uncommon => 2,
            CardRarity::Common => 1,
            _ => 0,
        };
        if rank > best_rank {
            best_rank = rank;
            best_idx = idx;
            ties = 1;
        } else if rank == best_rank {
            // Reservoir-sample among equals for a uniform pick
            ties += 1;
            if state.rng.random_range(0..ties) == 0 {
                best_idx = idx;
            }
        }
    }

    // Remove stolen Card from pile
    let id_card = id_pile.remove(best_idx);

    // Store it mirroring the source Monster's index in `id_stasis_cards`
    let idx_monster = id_monsters
        .iter()
        .position(|s| *s == Some(id_source))
        .expect("StasisSteal source is not on the roster");
    id_stasis_cards[idx_monster] = Some(id_card);
}
