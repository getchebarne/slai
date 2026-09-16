use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::types::CardKind;
use crate::types::Combat;
use crate::types::CostScope;
use crate::utils::entity_requires_target;
use crate::utils::is_play_restriction_satisfied;
use crate::utils::play_cap_reached;

pub fn process_effect_card_play_from_draw_top(id_target: Option<usize>, state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_card_play_from_draw_top outside the Combat frame"
    );
    let Combat {
        id_card_draw,
        id_card_discard,
        id_card_hand,
        id_monsters,
        this_turn_cards_played,
        ..
    } = &mut state.combat;

    // Check if the draw pile is empty
    if id_card_draw.is_empty() {
        if id_card_discard.is_empty() {
            return;
        }

        // Executes in reverse:
        //     1. ShuffleDiscardPileIntoDrawPile
        //     2. CardPlayFromDrawTop (re-queued)
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardPlayFromDrawTop,
            id_source: None,
            target: Target::Direct(id_target),
        });
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ShuffleDiscardPileIntoDrawPile,
            id_source: None,
            target: Target::Direct(None),
        });
        return;
    }

    // Detached from the pile here; card_play's routing effects move it onward
    let id_card = id_card_draw.pop().unwrap();

    // canUse gates the autoplay; a refused Card is still routed by UseCardAction, untriggered
    let card = &state.entities[id_card];
    // A target frozen at potion use may have died since; cardPlayable then fails
    let target_gone = id_target.is_some_and(|id| !id_monsters.contains(&Some(id)));
    let entangled = has_modifier(
        &state.entities[state.id_character].modifiers,
        ModifierKind::Entangled,
    );
    let playable = is_play_restriction_satisfied(
        card.card_play_restriction,
        card.card_kind,
        id_card_draw,
        &state.id_relics,
    ) && !(entangled && card.card_kind == CardKind::Attack)
        && !(target_gone && entity_requires_target(card))
        && !play_cap_reached(
            id_card_hand,
            &state.entities,
            &state.id_relics,
            *this_turn_cards_played,
        );
    if !playable {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardPlayRelocate {
                exhaust: card.card_exhaust,
            },
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
        return;
    }

    // Executes in reverse:
    //     1. SetCostOverride
    //     2. TargetSet
    //     3. CardPlay
    //     4. TargetClear
    state.effect_queue.push_front(Effect {
        kind: EffectKind::TargetClear,
        id_source: None,
        target: Target::Direct(None),
    });
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardPlay {
            energy_on_use: None,
        },
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
    // Distilled Chaos froze its target at use; Mayhem rolls one now
    state.effect_queue.push_front(Effect {
        kind: EffectKind::TargetSet,
        id_source: None,
        target: match id_target {
            Some(id_monster) => Target::Direct(Some(id_monster)),
            None => Target::Resolve {
                candidate_pool: CandidatePool::Monsters,
                filter: CandidateFilter::Any,
                selection_kind: SelectionKind::Random { count: 1 },
            },
        },
    });
    state.effect_queue.push_front(Effect {
        kind: EffectKind::SetCostOverride {
            amount: 0,
            only_reduce: false,
            random: false,
            scope: CostScope::UntilPlayed,
        },
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
}
