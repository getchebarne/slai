use crate::effect::{Effect, EffectKind};
use crate::engine::ProcessEffectResult;
use crate::modifier::modifier_clear;
use crate::state::{Entity, Map};
use crate::types::{EntityId, RoomType};

pub fn process_effect_combat_end(
    hand: &mut Vec<EntityId>,
    draw_pile: &mut Vec<EntityId>,
    discard_pile: &mut Vec<EntityId>,
    exhaust_pile: &mut Vec<EntityId>,
    card_target: &mut Option<EntityId>,
    entities: &mut Vec<Entity>,
    monster_count: &mut u8,
    map: &Map,
) -> ProcessEffectResult {
    hand.clear();
    draw_pile.clear();
    discard_pile.clear();
    exhaust_pile.clear();
    *card_target = None;

    let (_, modifiers) = entities[0].kind.combatant_mut();
    modifier_clear(modifiers);
    *monster_count = 0;

    let room = map.active_room_type().unwrap();
    match room {
        RoomType::CombatBoss => ProcessEffectResult::AddAndContinue {
            top: vec![Effect {
                kind: EffectKind::GameEnd,
                source: None,
                target: None,
            }],
            bot: Vec::new(),
        },
        RoomType::CombatMonster => ProcessEffectResult::AddAndContinue {
            top: Vec::new(),
            bot: vec![Effect {
                kind: EffectKind::CardRewardRoll,
                source: None,
                target: None,
            }],
        },
        RoomType::RestSite => unreachable!("combat end in rest site"),
    }
}
