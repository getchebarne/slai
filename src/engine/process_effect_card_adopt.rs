use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::RelicName;

pub fn process_effect_card_adopt(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardAdopt requires id_target");
    state.id_deck.push(id_card);

    // Ceramic Fish: 9 gold per card that actually joins the deck
    if state.id_relics[RelicName::CeramicFish as usize].is_some() {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(9),
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }
}
