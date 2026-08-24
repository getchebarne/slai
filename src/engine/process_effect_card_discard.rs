use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::Combat;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::has_relic;

// Branches on `source`: Explicit bumps counter and fires on-discard; EndOfTurn honors retain/ethereal
pub fn process_effect_card_discard(
    id_target: Option<usize>,
    state: &mut GameState,
    source: DiscardSource,
) {
    assert!(
        state.combat.active,
        "process_effect_card_discard outside the Combat frame"
    );
    let Combat {
        id_card_hand,
        id_card_discard,
        this_turn_discards,
        ..
    } = &mut state.combat;
    let id_target = id_target.expect("CardDiscard requires id_target");
    match source {
        DiscardSource::EndOfTurn => {
            // Clear retain flags
            if state.entities[id_target].card_retain {
                state.entities[id_target].card_retain = false;
                return;
            }

            // Exhaust ethereal Cards
            if state.entities[id_target].card_ethereal {
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::CardExhaust,
                    id_source: None,
                    target: Target::Direct(Some(id_target)),
                });
                return;
            }

            // Runic Pyramid: unplayed Cards stay in hand at end of turn
            if has_relic(&state.id_relics, RelicName::RunicPyramid) {
                return;
            }

            // Move from hand to discard
            if let Some(pos) = id_card_hand.iter().position(|&id| id == id_target) {
                id_card_hand.remove(pos);
            }
            id_card_discard.push(id_target);
        }
        DiscardSource::Explicit => {
            if let Some(pos) = id_card_hand.iter().position(|&id| id == id_target) {
                id_card_hand.remove(pos);
            }
            id_card_discard.push(id_target);
            *this_turn_discards = this_turn_discards.saturating_add(1);

            // Queued behind the rest so a batch's discards all land before any trigger
            let effects_on_discard = state.entities[id_target].card_on_discard_effects;
            for effect in effects_on_discard {
                state.effect_queue.push_back(Effect {
                    id_source: Some(id_target),
                    ..*effect
                });
            }

            // Tingsha: each discard deals 3 thorns-type damage (unscaled, no Envenom)
            if has_relic(&state.id_relics, RelicName::Tingsha) {
                state.effect_queue.push_back(Effect {
                    kind: EffectKind::DamageDeal {
                        amount: 3,
                        lifesteal: false,
                    },
                    id_source: None,
                    target: Target::Resolve {
                        candidate_pool: CandidatePool::Monsters,
                        filter: CandidateFilter::Any,
                        selection_kind: SelectionKind::Random { count: 1 },
                    },
                });
            }

            // Tough Bandages: each discard grants 3 block
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
