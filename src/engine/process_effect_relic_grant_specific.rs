use crate::effect::CandidatePool;
use crate::effect::CandidatePoolDeckFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::HealthDeltaAmount;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::grant_relic;

pub fn process_effect_relic_grant_specific(
    state: &mut GameState,
    name: RelicName,
    fallback_circlet: bool,
) {
    let owns_target = state.id_relics[name as usize].is_some();
    let target = match (owns_target, fallback_circlet) {
        (false, _) => name,
        (true, true) => RelicName::Circlet,
        (true, false) => return,
    };
    grant_relic(target, &mut state.id_relics, &mut state.entities);

    // On-pickup effects; keyed on `target` so a Circlet fallback never procs.
    // Shop-tier relics are only ever granted here, so this is their single pickup site
    match target {
        RelicName::DollysMirror => {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardDuplicate,
                id_source: None,
                target: Target::Resolve {
                    candidate_pool: CandidatePool::Deck {
                        filter: CandidatePoolDeckFilter::Any,
                    },
                    selection_kind: SelectionKind::Input { count: 1 },
                },
            });
        }
        RelicName::LeesWaffle => {
            // Max HP first so the heal tops up to the new ceiling
            state.effect_queue.push_front(Effect {
                kind: EffectKind::HealthDelta {
                    sign: DeltaSign::Gain,
                    amount: HealthDeltaAmount::Relative {
                        numerator: 1,
                        denominator: 1,
                    },
                },
                id_source: None,
                target: Target::Direct(Some(state.id_character)),
            });
            state.effect_queue.push_front(Effect {
                kind: EffectKind::MaxHealthDelta {
                    sign: DeltaSign::Gain,
                    amount: HealthDeltaAmount::Absolute(7),
                },
                id_source: None,
                target: Target::Direct(Some(state.id_character)),
            });
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use crate::action::Action;
    use crate::action::handle_action;
    use crate::action::recompute_legal_actions;
    use crate::effect::Effect;
    use crate::effect::EffectKind;
    use crate::effect::Target;
    use crate::engine::process_effect_queue;
    use crate::game::create_game_state;
    use crate::types::RelicName;

    fn grant(state: &mut crate::game::GameState, name: RelicName) {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::RelicGrantSpecific {
                name,
                fallback_circlet: false,
            },
            id_source: None,
            target: Target::Direct(None),
        });
        process_effect_queue(state);
    }

    #[test]
    fn lees_waffle_grants_max_hp_and_heals_to_full() {
        let mut state = create_game_state(0, 42, false);
        let id_character = state.id_character;
        let max_before = state.entities[id_character].vitals.health_max;
        state.entities[id_character].vitals.health = 40;
        grant(&mut state, RelicName::LeesWaffle);
        let vitals = &state.entities[id_character].vitals;
        assert_eq!(vitals.health_max, max_before + 7);
        assert_eq!(vitals.health, vitals.health_max);
    }

    #[test]
    fn dollys_mirror_duplicates_the_picked_deck_card() {
        let mut state = create_game_state(0, 42, false);
        let deck_before = state.id_deck.len();
        grant(&mut state, RelicName::DollysMirror);
        // Grant halts on the deck pick
        assert!(state.effect_pending.is_some());
        let picked_name = state.entities[state.id_deck[0]].card_name;
        recompute_legal_actions(&mut state);
        handle_action(&mut state, Action::CardDuplicate { idx: 0 }).unwrap();
        process_effect_queue(&mut state);
        assert_eq!(state.id_deck.len(), deck_before + 1);
        let added = *state.id_deck.last().unwrap();
        assert_eq!(state.entities[added].card_name, picked_name);
    }
}
