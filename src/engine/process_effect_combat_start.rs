use rand::Rng;

use crate::cards::Card;
use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::state::{Entity, EntityKind};
use crate::types::EntityId;
use crate::utils::shuffle;

pub fn process_effect_combat_start(
    deck: &[Card],
    entities: &mut Vec<Option<Entity>>,
    draw_pile: &mut Vec<EntityId>,
    hand: &mut Vec<EntityId>,
    discard_pile: &mut Vec<EntityId>,
    exhaust_pile: &mut Vec<EntityId>,
    card_active: &mut Option<EntityId>,
    card_target: &mut Option<EntityId>,
    rng: &mut impl Rng,
) -> ProcessEffectResult {
    let mut innate_ids: Vec<EntityId> = Vec::new();
    let mut other_ids: Vec<EntityId> = Vec::new();

    for card in deck {
        let id = EntityId(entities.len() as u32);
        entities.push(Some(Entity { kind: EntityKind::Card(*card) }));
        if card.innate {
            innate_ids.push(id);
        } else {
            other_ids.push(id);
        }
    }

    shuffle(&mut other_ids, rng);

    *draw_pile = innate_ids;
    draw_pile.extend(other_ids);

    hand.clear();
    discard_pile.clear();
    exhaust_pile.clear();
    *card_active = None;
    *card_target = None;

    let monster_ids: Vec<EntityId> = entities.iter().enumerate()
        .filter(|(_, s)| matches!(s, Some(Entity { kind: EntityKind::Monster(..) })))
        .map(|(i, _)| EntityId(i as u32))
        .collect();

    let mut effects: Vec<Effect> = Vec::new();
    for &id in &monster_ids {
        effects.push(Effect::MoveUpdate { monster: id });
    }
    effects.push(Effect::TurnStart { actor: EntityId(0) });

    ProcessEffectResult::Continue {
        top: effects,
        bot: Vec::new(),
    }
}
