use crate::cards::Card;
use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::modifier::{Modifiers, modifier_clear};
use crate::state::Map;
use crate::types::RoomType;

pub fn process_effect_combat_end(
    hand: &mut Vec<usize>,
    draw_pile: &mut Vec<usize>,
    discard_pile: &mut Vec<usize>,
    exhaust_pile: &mut Vec<usize>,
    combat_cards: &mut Vec<Card>,
    card_active: &mut Option<usize>,
    card_target: &mut Option<u8>,
    character_modifiers: &mut Modifiers,
    map: &Map,
) -> ProcessEffectResult {
    // Clear combat elements
    hand.clear();
    draw_pile.clear();
    discard_pile.clear();
    exhaust_pile.clear();
    combat_cards.clear();
    *card_active = None;
    *card_target = None;
    modifier_clear(character_modifiers);

    // Route according to current room type
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
