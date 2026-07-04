use crate::effect::CandidatePool;
use crate::effect::CandidatePoolMonstersFilter;
use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::RelicName;

// Branches on `source`: Explicit bumps counter and fires on-discard; EndOfTurn honors retain/ethereal
pub fn process_effect_card_discard(
    id_target: Option<usize>,
    state: &mut GameState,
    source: DiscardSource,
) {
    let id_target = id_target.expect("CardDiscard requires id_target");
    match source {
        DiscardSource::EndOfTurn => {
            if state.entities[id_target].card_retain {
                state.entities[id_target].card_retain = false;
                return;
            }
            if state.entities[id_target].card_ethereal {
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::CardExhaust,
                    id_source: None,
                    target: Target::Direct(Some(id_target)),
                });
                return;
            }
            if let Some(pos) = state.id_hand.iter().position(|&v| v == id_target) {
                state.id_hand.remove(pos);
            }
            state.id_pile_discard.push(id_target);
        }
        DiscardSource::Explicit => {
            if let Some(pos) = state.id_hand.iter().position(|&v| v == id_target) {
                state.id_hand.remove(pos);
            }
            state.id_pile_discard.push(id_target);
            state.this_turn_discards = state.this_turn_discards.saturating_add(1);

            // Push reversed so first effect runs first when queue resumes
            let effects_on_discard = state.entities[id_target].card_on_discard_effects;
            for effect in effects_on_discard.iter().rev() {
                state.effect_queue.push_front(Effect {
                    id_source: Some(id_target),
                    ..*effect
                });
            }

            // Thorns-type damage: id_source None skips Strength/Weak scaling and Envenom
            if state.id_relics[RelicName::Tingsha as usize].is_some() {
                state.effect_queue.push_back(Effect {
                    kind: EffectKind::DamageDeal { amount: 3 },
                    id_source: None,
                    target: Target::Resolve {
                        candidate_pool: CandidatePool::Monsters {
                            filter: CandidatePoolMonstersFilter::All,
                        },
                        selection_kind: SelectionKind::Random { count: 1 },
                    },
                });
            }
            // Relic-sourced block: id_source None skips Dex/Frail scaling
            if state.id_relics[RelicName::ToughBandages as usize].is_some() {
                state.effect_queue.push_back(Effect {
                    kind: EffectKind::BlockGain { amount: 3 },
                    id_source: None,
                    target: Target::Direct(Some(state.id_character)),
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::effect::DiscardSource;
    use crate::effect::Effect;
    use crate::effect::EffectKind;
    use crate::effect::Target;
    use crate::engine::process_effect_queue;
    use crate::engine::test_support::combat_with_relic;
    use crate::engine::test_support::first_monster;
    use crate::engine::test_support::put_in_hand;
    use crate::game::GameState;
    use crate::types::CardName;
    use crate::types::MonsterName;
    use crate::types::RelicName;

    fn discard(state: &mut GameState, id_card: usize, source: DiscardSource) {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CardDiscard { source },
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
        process_effect_queue(state);
    }

    #[test]
    fn tingsha_damages_a_random_enemy_on_manual_discard() {
        let mut state = combat_with_relic(RelicName::Tingsha, MonsterName::JawWorm);
        let id_monster = first_monster(&state);
        let hp_before = state.entities[id_monster].vitals.health;
        let id = put_in_hand(&mut state, CardName::Strike);
        discard(&mut state, id, DiscardSource::Explicit);
        assert_eq!(state.entities[id_monster].vitals.health, hp_before - 3);
    }

    #[test]
    fn tough_bandages_blocks_on_manual_discard_only() {
        let mut state = combat_with_relic(RelicName::ToughBandages, MonsterName::JawWorm);
        let id = put_in_hand(&mut state, CardName::Strike);
        discard(&mut state, id, DiscardSource::EndOfTurn);
        assert_eq!(state.entities[state.id_character].vitals.block, 0);
        let id = put_in_hand(&mut state, CardName::Strike);
        discard(&mut state, id, DiscardSource::Explicit);
        assert_eq!(state.entities[state.id_character].vitals.block, 3);
    }
}
