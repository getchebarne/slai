use std::collections::VecDeque;

use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::map::active_room_type;
use crate::modifier::modifier_clear;
use crate::state::Map;
use crate::types::RoomType;

pub fn process_effect_combat_end(
    id_character: usize,
    id_hand: &mut Vec<usize>,
    id_draw_pile: &mut Vec<usize>,
    id_discard_pile: &mut Vec<usize>,
    id_exhaust_pile: &mut Vec<usize>,
    id_card_target: &mut Option<usize>,
    entities: &mut Vec<Entity>,
    monster_count: &mut u8,
    map: &Map,
    queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    id_hand.clear();
    id_draw_pile.clear();
    id_discard_pile.clear();
    id_exhaust_pile.clear();
    *id_card_target = None;

    let room = active_room_type(map, entities).unwrap();

    modifier_clear(&mut entities[id_character].modifiers);
    *monster_count = 0;
    match room {
        RoomType::CombatBoss => {
            queue.clear();
            queue.push_back(Effect::direct(EffectKind::GameOver, None, None));
        }
        RoomType::CombatMonster => {
            queue.push_back(Effect {
                kind: EffectKind::CardRewardRoll,
                id_source: None,
                target: Target::Direct(None),
            });
        }
        RoomType::RestSite => unreachable!("combat end in rest site"),
    }
    DispatchResult::Continue
}
