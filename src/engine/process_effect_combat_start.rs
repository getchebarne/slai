use rand::Rng;

use crate::effect::{Effect, EffectKind};
use crate::engine::ProcessEffectResult;
use crate::state::{Entity, EntityKind};
use crate::types::EntityId;
use crate::utils::shuffle;

pub fn process_effect_combat_start(
    character: EntityId,
    deck: &[EntityId],
    entities: &mut Vec<Entity>,
    draw_pile: &mut Vec<EntityId>,
    hand: &mut Vec<EntityId>,
    discard_pile: &mut Vec<EntityId>,
    exhaust_pile: &mut Vec<EntityId>,
    card_target: &mut Option<EntityId>,
    monsters: &[EntityId],
    monster_count: u8,
    rng: &mut impl Rng,
) -> ProcessEffectResult {
    let mut innate_ids: Vec<EntityId> = Vec::new();
    let mut other_ids: Vec<EntityId> = Vec::new();

    for &deck_id in deck {
        let card = *entities[deck_id.0 as usize].kind.card_ref();
        let id = EntityId(entities.len() as u32);
        entities.push(Entity {
            kind: EntityKind::Card(card),
        });
        if card.innate {
            innate_ids.push(id);
        } else {
            other_ids.push(id);
        }
    }

    shuffle(&mut other_ids, rng);

    *draw_pile = other_ids;
    draw_pile.extend(innate_ids);

    hand.clear();
    discard_pile.clear();
    exhaust_pile.clear();
    *card_target = None;

    let mut effects: Vec<Effect> = Vec::new();
    for &id in &monsters[..monster_count as usize] {
        effects.push(Effect {
            kind: EffectKind::MoveUpdate,
            source: None,
            target: Some(id),
        });
    }
    effects.push(Effect {
        kind: EffectKind::TurnStart,
        source: None,
        target: Some(character),
    });

    ProcessEffectResult::AddAndContinue {
        top: effects,
        bot: Vec::new(),
    }
}
