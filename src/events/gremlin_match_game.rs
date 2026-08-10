use crate::cards::POOL_COMMON_GREEN_CARD;
use crate::cards::POOL_CURSE_CARD;
use crate::cards::POOL_RARE_GREEN_CARD;
use crate::cards::POOL_UNCOMMON_COLORLESS_CARD;
use crate::cards::POOL_UNCOMMON_GREEN_CARD;
use crate::consts::MATCH_GAME_CARDS;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::events::EventKind;
use crate::events::make_entity_event_option;
use crate::game::GameState;
use crate::types::CardName;
use crate::utils::shuffle;
use rand::Rng;

const EFFECT_FLIP: &[Effect] = &[Effect {
    kind: EffectKind::MatchGameFlip,
    id_source: None,
    target: Target::Direct(None),
}];

pub static OPTIONS: &[Entity] = &[
    make_entity_event_option("[Flip card 1]", EFFECT_FLIP),
    make_entity_event_option("[Flip card 2]", EFFECT_FLIP),
    make_entity_event_option("[Flip card 3]", EFFECT_FLIP),
    make_entity_event_option("[Flip card 4]", EFFECT_FLIP),
    make_entity_event_option("[Flip card 5]", EFFECT_FLIP),
    make_entity_event_option("[Flip card 6]", EFFECT_FLIP),
    make_entity_event_option("[Flip card 7]", EFFECT_FLIP),
    make_entity_event_option("[Flip card 8]", EFFECT_FLIP),
    make_entity_event_option("[Flip card 9]", EFFECT_FLIP),
    make_entity_event_option("[Flip card 10]", EFFECT_FLIP),
    make_entity_event_option("[Flip card 11]", EFFECT_FLIP),
    make_entity_event_option("[Flip card 12]", EFFECT_FLIP),
];

// Six pairs: rare, uncommon and common class cards, a colorless uncommon (a second
// curse at A15+), a random curse, and the class starter
pub fn spawn_event_gremlin_match_game(state: &mut GameState) -> EventKind {
    let rng = &mut state.rng;
    let mut pick = |pool: &[CardName]| pool[rng.random_range(0..pool.len())];
    let sixth = if state.ascension < 15 {
        pick(POOL_UNCOMMON_COLORLESS_CARD)
    } else {
        pick(POOL_CURSE_CARD)
    };
    let pairs = [
        pick(POOL_RARE_GREEN_CARD),
        pick(POOL_UNCOMMON_GREEN_CARD),
        pick(POOL_COMMON_GREEN_CARD),
        pick(POOL_CURSE_CARD),
        CardName::Survivor,
        sixth,
    ];

    let mut board = [CardName::Survivor; MATCH_GAME_CARDS];
    for (idx, &name) in pairs.iter().enumerate() {
        board[idx * 2] = name;
        board[idx * 2 + 1] = name;
    }
    shuffle(&mut board, &mut state.rng);

    EventKind::GremlinMatchGame {
        board,
        matched: 0,
        revealed: 0,
        attempts: 0,
    }
}

pub fn option_available(matched: u16, revealed: u16, idx: usize) -> bool {
    // A lone revealed bit is the attempt's live first flip and can't be re-picked
    let first_flip_live = revealed.count_ones() == 1 && revealed & (1 << idx) != 0;
    matched & (1 << idx) == 0 && !first_flip_live
}
