use crate::consts::MATCH_GAME_ATTEMPTS;
use crate::consts::MATCH_GAME_CARDS;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventKind;
use crate::game::GameState;
use crate::types::CardPile;
use crate::types::Mode;
use crate::utils::mode_top_mut;

// Match and Keep: flip a face-down card; a matched pair is obtained, a miss burns
// one of the five attempts. The flipped index is the option's position
pub fn process_effect_match_game_flip(id_source: Option<usize>, state: &mut GameState) {
    let id_option = id_source.expect("MatchGameFlip requires the option as id_source");
    let Mode::Event {
        kind:
            EventKind::GremlinMatchGame {
                board,
                matched,
                revealed,
                attempts,
            },
        id_options,
        ..
    } = mode_top_mut(&mut state.mode_stack)
    else {
        unreachable!("MatchGameFlip outside a Match and Keep event")
    };
    let idx = id_options
        .iter()
        .position(|&id| id == id_option)
        .expect("Flipped option is baked") as u8;

    // A lone revealed bit is the attempt's live first flip
    if revealed.count_ones() != 1 {
        // First flip of the attempt: any earlier miss flips back down
        *revealed = 1 << idx;
        return;
    }
    let first = revealed.trailing_zeros() as u8;

    // Second flip resolves the attempt
    *attempts += 1;
    if board[first as usize] == board[idx as usize] {
        *matched |= (1 << first) | (1 << idx);
        *revealed = 0;
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardAdd {
                card_name: board[idx as usize],
                pile: CardPile::Deck,
                count: 1,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    } else {
        // The miss stays face up until the next flip
        *revealed |= 1 << idx;
    }

    // Out of attempts, or the whole board cleared: the game ends
    if *attempts >= MATCH_GAME_ATTEMPTS || *matched == (1 << MATCH_GAME_CARDS) - 1 {
        state.effect_queue.push_back(EVENT_CONSUME_EFFECT);
    }
}
