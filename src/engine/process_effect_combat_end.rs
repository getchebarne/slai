use crate::effect::{Effect, EffectKind, Target};
use crate::engine::ProcessEffectResult;
use crate::entities::Entity;
use crate::map::active_room_type;
use crate::modifier::modifier_clear;
use crate::state::Map;
use crate::types::RoomType;

pub fn process_effect_combat_end(
    character: usize,
    hand: &mut Vec<usize>,
    draw_pile: &mut Vec<usize>,
    discard_pile: &mut Vec<usize>,
    exhaust_pile: &mut Vec<usize>,
    card_target: &mut Option<usize>,
    entities: &mut Vec<Entity>,
    monster_count: &mut u8,
    map: &Map,
) -> ProcessEffectResult {
    // Reset combat piles
    hand.clear();
    draw_pile.clear();
    discard_pile.clear();
    exhaust_pile.clear();
    *card_target = None;

    // Next step depends on room type — read it before mutating entities
    let room = active_room_type(&map, entities).unwrap();

    modifier_clear(&mut entities[character].modifiers);
    *monster_count = 0;
    match room {
        RoomType::CombatBoss => ProcessEffectResult::Replace(vec![
            Effect::direct(EffectKind::GameOver, None, None),
        ]),
        RoomType::CombatMonster => ProcessEffectResult::Continue {
            top: Vec::new(),
            bot: vec![Effect {
                kind: EffectKind::CardRewardRoll,
                source: None,
                target: Target::Direct(None),
            }],
        },
        RoomType::RestSite => unreachable!("combat end in rest site"),
    }
}
