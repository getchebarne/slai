use crate::cards::get_random_card_names;
use crate::effect::EFFECT_ACCURACY_RESYNC_HAND;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::types::CardColor;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::Combat;
use crate::types::RelicName;
use crate::utils::has_relic;

pub fn process_effect_card_exhaust(id_target: Option<usize>, state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_card_exhaust outside the Combat frame"
    );
    let Combat {
        id_card_pile_hand,
        id_card_pile_exhaust,
        ..
    } = &mut state.combat;
    let id_card = id_target.expect("CardExhaust requires id_target");
    if let Some(pos) = id_card_pile_hand.iter().position(|&id| id == id_card) {
        id_card_pile_hand.remove(pos);
    }
    id_card_pile_exhaust.push(id_card);

    // An exhausted Card's cost this turn drops back to its combat cost, after any waiting replay reads it
    if !state
        .card_play_queue
        .iter()
        .any(|card_play| card_play.id_card == id_card)
    {
        state.entities[id_card].card_cost_override = None;
    }

    // Dead Branch: every exhaust conjures a random Silent Card into the hand
    // (all green Cards are rewardable, so no kind/rarity filter is needed)
    if has_relic(&state.id_relics, RelicName::DeadBranch) {
        let card_name =
            get_random_card_names(CardColor::Green, None, None, &[], true, 1, &mut state.rng)[0];
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CardAdd {
                card_name,
                card_pile: CardPile::Hand,
                count: 1,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }

    // Necronomicurse: exhausting it returns a copy to hand, behind Dead Branch's Card
    if state.entities[id_card].card_name == CardName::Necronomicurse {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CardAdd {
                card_name: CardName::Necronomicurse,
                card_pile: CardPile::Hand,
                count: 1,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }

    // Accuracy: an exhaust resets every Shiv in the hand
    if has_modifier(
        &state.entities[state.id_character].modifiers,
        ModifierKind::Accuracy,
    ) {
        state.effect_queue.push_front(EFFECT_ACCURACY_RESYNC_HAND);
    }
}
