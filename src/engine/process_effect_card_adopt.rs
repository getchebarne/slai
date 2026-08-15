use crate::cards::get_card;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::relics::egg_upgrades_kind;
use crate::types::CardKind;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::has_relic;
use crate::utils::increase_max_hp;

pub fn process_effect_card_adopt(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardAdopt requires id_target");
    let card = state.entities[id_card];

    // Omamori: a charge negates the Curse outright; no other hook triggers
    if card.card_kind == CardKind::Curse
        && let Some(id_relic) = state.id_relics[RelicName::Omamori as usize]
        && state.entities[id_relic].relic_counter > 0
    {
        let relic = &mut state.entities[id_relic];
        relic.relic_counter -= 1;
        relic.relic_used_up = relic.relic_counter == 0;
        return;
    }

    // Push card
    state.id_deck.push(id_card);

    // Frozen / Molten / Toxic Egg: matching kinds join the deck upgraded
    if !card.card_upgraded && egg_upgrades_kind(card.card_kind, &state.id_relics) {
        state.entities[id_card] = get_card(card.card_name, true);
    }

    // Ceramic Fish: 9 gold per Card that actually joins the deck
    if has_relic(&state.id_relics, RelicName::CeramicFish) {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(9),
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Darkstone Periapt: obtaining a Curse raises max HP by 6 (healed)
    if card.card_kind == CardKind::Curse && has_relic(&state.id_relics, RelicName::DarkstonePeriapt)
    {
        increase_max_hp(state, state.id_character, 6);
    }
}
