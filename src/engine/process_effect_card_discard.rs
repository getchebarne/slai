use crate::effect::CandidatePool;
use crate::effect::DiscardSource;
use crate::effect::EFFECT_ACCURACY_RESYNC_HAND;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::CostOverride;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::types::CardName;
use crate::types::Combat;
use crate::types::CostScope;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::get_card_cost_this_turn;
use crate::utils::has_relic;

// Every source bumps the counter; the explicit ones also queue the Card's discard effects and fire the discard hooks, EndOfTurn honors retain
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
        id_card_pile_hand,
        id_card_pile_discard,
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

            // Move from hand to discard
            if let Some(pos) = id_card_pile_hand.iter().position(|&id| id == id_target) {
                id_card_pile_hand.remove(pos);
            }
            id_card_pile_discard.push(id_target);

            // The turn end's discards count too, though they set nothing off
            *this_turn_discards = this_turn_discards.saturating_add(1);
        }
        DiscardSource::Explicit | DiscardSource::ExplicitRelicsFirst => {
            if let Some(pos) = id_card_pile_hand.iter().position(|&id| id == id_target) {
                id_card_pile_hand.remove(pos);
            }
            id_card_pile_discard.push(id_target);
            *this_turn_discards = this_turn_discards.saturating_add(1);
            if source == DiscardSource::ExplicitRelicsFirst {
                fire_discard_hooks(state);
                queue_card_effects_discard(state, id_target);
            } else {
                queue_card_effects_discard(state, id_target);
                fire_discard_hooks(state);
            }
        }
    }

    // Accuracy: a discard resets every Shiv in the hand
    if has_modifier(
        &state.entities[state.id_character].modifiers,
        ModifierKind::Accuracy,
    ) {
        state.effect_queue.push_front(EFFECT_ACCURACY_RESYNC_HAND);
    }
}

// Eviscerate's discount and the discard Relics; once the turn has ended, a discard sets off neither
fn fire_discard_hooks(state: &mut GameState) {
    if state.combat.turn_ended {
        return;
    }
    let Combat {
        id_card_pile_hand,
        id_card_pile_draw,
        id_card_pile_discard,
        ..
    } = &state.combat;

    // Eviscerate in hand, draw or discard costs 1 less this turn, down to 0
    for &id_card in id_card_pile_hand
        .iter()
        .chain(id_card_pile_draw.iter())
        .chain(id_card_pile_discard.iter())
    {
        let card = &mut state.entities[id_card];
        let cost = get_card_cost_this_turn(card);
        if card.card_cost_kind == CardCostKind::MinusDiscardsThisTurn && cost > 0 {
            card.card_cost_override = Some(CostOverride {
                amount: cost - 1,
                scope: CostScope::Turn,
            });
        }
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
                filters: &[],
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

// On-discard effects: Tactician's energy lands right after its own discard
fn queue_card_effects_discard(state: &mut GameState, id_card: usize) {
    let card = &state.entities[id_card];
    for effect in card.card_effects_discard {
        let effect = Effect {
            id_source: Some(id_card),
            ..*effect
        };
        if card.card_name == CardName::Tactician {
            state.effect_queue.push_front(effect);
        } else {
            // Queued behind the rest so a batch's discards all land before any trigger
            state.effect_queue.push_back(effect);
        }
    }
}
