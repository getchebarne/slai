use crate::cards::get_card_template;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_stacks;
use crate::types::CardName;
use crate::utils::card_damage_delta;

// Accuracy: a Shiv's damage resets to its printed damage plus the Character's Accuracy
pub fn process_effect_accuracy_resync(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("AccuracyResync requires id_target");
    if state.entities[id_card].card_name != CardName::Shiv {
        return;
    }
    let accuracy = modifier_stacks(
        &state.entities[state.id_character].modifiers,
        ModifierKind::Accuracy,
    );
    let card = &mut state.entities[id_card];
    card.card_effects = get_card_template(CardName::Shiv, card.card_upgraded).effects;
    card_damage_delta(card, accuracy);
}
