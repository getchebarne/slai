use crate::effect::CandidatePool;
use crate::effect::CandidatePoolMonstersFilter;
use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::RelicName;
use crate::utils::has_relic;

// Branches on `source`: Explicit bumps counter and fires on-discard; EndOfTurn honors retain/ethereal
pub fn process_effect_card_discard(
    id_target: Option<usize>,
    state: &mut GameState,
    source: DiscardSource,
) {
    let Mode::Combat {
        id_hand,
        id_pile_discard,
        this_turn_discards,
        ..
    } = &mut state.mode
    else {
        unreachable!("process_effect_card_discard outside Combat mode")
    };
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
            // Runic Pyramid: unplayed cards stay in hand at end of turn
            if has_relic(&state.id_relics, RelicName::RunicPyramid) {
                return;
            }
            if let Some(pos) = id_hand.iter().position(|&v| v == id_target) {
                id_hand.remove(pos);
            }
            id_pile_discard.push(id_target);
        }
        DiscardSource::Explicit => {
            if let Some(pos) = id_hand.iter().position(|&v| v == id_target) {
                id_hand.remove(pos);
            }
            id_pile_discard.push(id_target);
            *this_turn_discards = this_turn_discards.saturating_add(1);

            // Push reversed so first effect runs first when queue resumes
            let effects_on_discard = state.entities[id_target].card_on_discard_effects;
            for effect in effects_on_discard.iter().rev() {
                state.effect_queue.push_front(Effect {
                    id_source: Some(id_target),
                    ..*effect
                });
            }

            // Thorns-type: unscaled, no Envenom
            if has_relic(&state.id_relics, RelicName::Tingsha) {
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
            if has_relic(&state.id_relics, RelicName::ToughBandages) {
                state.effect_queue.push_back(Effect {
                    kind: EffectKind::BlockGain { amount: 3 },
                    id_source: None,
                    target: Target::Direct(Some(state.id_character)),
                });
            }
            // Hovering Kite: the first discard each turn grants 1 energy (counter resets per turn)
            if let Some(id) = state.id_relics[RelicName::HoveringKite as usize]
                && state.entities[id].relic_counter == 0
            {
                state.entities[id].relic_counter = 1;
                state.effect_queue.push_back(Effect {
                    kind: EffectKind::EnergyDelta {
                        sign: DeltaSign::Gain,
                        amount: 1,
                    },
                    id_source: None,
                    target: Target::Direct(None),
                });
            }
        }
    }
}
