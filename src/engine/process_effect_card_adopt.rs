use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardKind;
use crate::types::RelicName;

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

    // The Card lands once the queued work settles, so a Relic granted beside it sees it land
    state.effect_queue.push_back(Effect {
        kind: EffectKind::CardObtain,
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
}
