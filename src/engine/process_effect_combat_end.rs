use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::modifier::modifier_clear;
use crate::state::{Entity, Map};
use crate::types::{EntityId, RoomType};

pub fn process_effect_combat_end(
    hand: &mut Vec<EntityId>,
    draw_pile: &mut Vec<EntityId>,
    discard_pile: &mut Vec<EntityId>,
    exhaust_pile: &mut Vec<EntityId>,
    card_active: &mut Option<EntityId>,
    card_target: &mut Option<EntityId>,
    entities: &mut Vec<Option<Entity>>,
    map: &Map,
) -> ProcessEffectResult {
    hand.clear();
    draw_pile.clear();
    discard_pile.clear();
    exhaust_pile.clear();
    *card_active = None;
    *card_target = None;

    // Clear character modifiers and remove all non-character entities
    for (i, slot) in entities.iter_mut().enumerate() {
        if i == 0 {
            if let Some(entity) = slot {
                let (_, modifiers) = entity.kind.combatant_mut();
                modifier_clear(modifiers);
            }
        } else {
            *slot = None;
        }
    }
    entities.truncate(1);

    let room = map.active_room_type().unwrap();
    match room {
        RoomType::CombatBoss => ProcessEffectResult::Continue {
            top: vec![Effect::GameEnd],
            bot: Vec::new(),
        },
        RoomType::CombatMonster => ProcessEffectResult::Continue {
            top: Vec::new(),
            bot: vec![Effect::CardRewardRoll],
        },
        RoomType::RestSite => unreachable!("combat end in rest site"),
    }
}
