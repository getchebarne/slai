use rand::Rng;

use crate::cards::POOL_COMMON_GREEN_CARD;
use crate::cards::POOL_RARE_GREEN_CARD;
use crate::cards::POOL_UNCOMMON_GREEN_CARD;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::RelicName;

pub fn process_effect_card_exhaust(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardExhaust requires id_target");
    if let Some(pos) = state.id_hand.iter().position(|&v| v == id_card) {
        state.id_hand.remove(pos);
    }
    state.id_pile_exhaust.push(id_card);

    // Dead Branch: every exhaust conjures a random Silent card into the hand
    if state.id_relics[RelicName::DeadBranch as usize].is_some() {
        let mut buf = [CardName::Strike; 96];
        let mut n = 0;
        for pool in [
            POOL_COMMON_GREEN_CARD,
            POOL_UNCOMMON_GREEN_CARD,
            POOL_RARE_GREEN_CARD,
        ] {
            for &name in pool {
                buf[n] = name;
                n += 1;
            }
        }
        let card_name = buf[state.rng.random_range(0..n)];
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

#[cfg(test)]
mod tests {
    use crate::engine::test_support::combat_with_relic;
    use crate::engine::test_support::play;
    use crate::engine::test_support::put_in_hand;
    use crate::types::CardName;
    use crate::types::MonsterName;
    use crate::types::RelicName;

    #[test]
    fn dead_branch_conjures_a_card_on_exhaust() {
        let mut state = combat_with_relic(RelicName::DeadBranch, MonsterName::JawWorm);
        let hand_before = state.id_hand.len();
        let id = put_in_hand(&mut state, CardName::Defend);
        state.entities[id].card_exhaust = true;
        play(&mut state, id);
        // Defend exhausted, Dead Branch conjured a replacement
        assert_eq!(state.id_hand.len(), hand_before + 1);
        assert_eq!(state.id_pile_exhaust.len(), 1);
    }
}
