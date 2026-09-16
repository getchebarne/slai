use crate::game::GameState;
use crate::types::CardKind;
use crate::types::RelicName;

// Unregisters the targeted Relic; the entity stays orphaned in the arena
pub fn process_effect_relic_lose(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("RelicLose requires id_target");
    let name = state.entities[id_target].relic_name;
    assert!(
        state.id_relics[name as usize] == Some(id_target),
        "RelicLose without owning {name:?}"
    );
    state.id_relics[name as usize] = None;

    // onUnequip: a bottle leaving the belt frees its Card
    let bottled_kind = match name {
        RelicName::BottledFlame => Some(CardKind::Attack),
        RelicName::BottledLightning => Some(CardKind::Skill),
        RelicName::BottledTornado => Some(CardKind::Power),
        _ => None,
    };
    if let Some(kind) = bottled_kind {
        for &id_card in state.id_card_deck.iter() {
            let card = &mut state.entities[id_card];
            if card.card_bottled && card.card_kind == kind {
                card.card_bottled = false;
            }
        }
    }
}
