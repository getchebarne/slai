use crate::cards::get_card;
use crate::engine::ProcessEffectResult;
use crate::entities::Entity;

pub fn process_effect_card_upgrade(
    target: usize,
    entities: &mut [Entity],
) -> ProcessEffectResult {
    let e = &mut entities[target];
    let upgraded = get_card(e.card_name, true);
    e.card_kind = upgraded.kind;
    e.card_cost = upgraded.cost;
    e.card_effects = upgraded.effects;
    e.card_upgraded = upgraded.upgraded;
    e.card_exhaust = upgraded.exhaust;
    e.card_innate = upgraded.innate;
    e.card_requires_target = upgraded.requires_target;

    ProcessEffectResult::Continue { top: vec![], bot: vec![] }
}
