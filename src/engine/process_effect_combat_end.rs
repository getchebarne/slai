use std::collections::VecDeque;

use rand::Rng;

use crate::consts::{
    ELITE_TH_COMMON, ELITE_TH_UNCOMMON, GOLD_BOSS_MAX, GOLD_BOSS_MIN, GOLD_ELITE_MAX,
    GOLD_ELITE_MIN, GOLD_MONSTER_MAX, GOLD_MONSTER_MIN, MAP_HEIGHT, MAP_WIDTH,
};
use crate::effect::{Effect, EffectKind, Target};
use crate::engine::DispatchResult;
use crate::entity::{Entity, EntityKind};
use crate::map::get_active_room_kind;
use crate::modifier::modifier_clear;
use crate::game::Location;
use crate::types::RoomKind;

pub fn process_effect_combat_end(
    id_character: usize,
    id_hand: &mut Vec<usize>,
    id_pile_draw: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    id_pile_exhaust: &mut Vec<usize>,
    id_card_target: &mut Option<usize>,
    entities: &mut Vec<Entity>,
    monster_count: &mut u8,
    id_card_nightmare: &mut Option<usize>,
    id_rooms: &[[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    location: Location,
    escaped_this_combat: bool,
    rng: &mut impl Rng,
    effect_queue: &mut VecDeque<Effect>,
) -> DispatchResult {
    // Clear card piles and target
    id_hand.clear();
    id_pile_draw.clear();
    id_pile_discard.clear();
    id_pile_exhaust.clear();
    *id_card_nightmare = None;
    *id_card_target = None;

    // Clear character's modifiers
    modifier_clear(&mut entities[id_character].modifiers);

    // Clear retained cards
    for entity in entities.iter_mut() {
        match entity.kind {
            EntityKind::Card => {
                entity.card_retain = false;
            }
            EntityKind::Monster => {
                // Prevent stale Poison/Shackled/etc. from leaking into views
                // after the next combat reuses (or doesn't reuse) the slot
                modifier_clear(&mut entity.modifiers);
            }
            _ => {}
        }
    }

    // Clear monsters
    *monster_count = 0;

    // Dispatch according to current room type
    let room = get_active_room_kind(id_rooms, location, entities).unwrap();
    match room {
        RoomKind::CombatBoss => {
            // Boss defeated — drop any pending effects. derive_phase
            // returns GameOver from `location == BossRoom && monster_count == 0`
            effect_queue.clear();
            push_gold_gain(rng, GOLD_BOSS_MIN, GOLD_BOSS_MAX, id_character, effect_queue);
        }
        RoomKind::CombatMonster => {
            if !escaped_this_combat {
                push_gold_gain(rng, GOLD_MONSTER_MIN, GOLD_MONSTER_MAX, id_character, effect_queue);
            }
            effect_queue.push_back(Effect {
                kind: EffectKind::CardRewardRoll,
                id_source: None,
                target: Target::Direct(None),
            });
        }
        RoomKind::CombatElite => {
            push_gold_gain(rng, GOLD_ELITE_MIN, GOLD_ELITE_MAX, id_character, effect_queue);
            effect_queue.push_back(Effect {
                kind: EffectKind::CardRewardRoll,
                id_source: None,
                target: Target::Direct(None),
            });
            effect_queue.push_back(Effect {
                kind: EffectKind::RelicRewardRoll {
                    th_common: ELITE_TH_COMMON,
                    th_uncommon: ELITE_TH_UNCOMMON,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }
        RoomKind::RestSite | RoomKind::Treasure | RoomKind::EventRoom | RoomKind::Shop => {
            unreachable!("combat end in non-combat room: {:?}", room)
        }
    }
    DispatchResult::Continue
}

fn push_gold_gain(
    rng: &mut impl Rng,
    min: u16,
    max: u16,
    id_character: usize,
    effect_queue: &mut VecDeque<Effect>,
) {
    let amount = rng.random_range(min..=max);
    effect_queue.push_back(Effect {
        kind: EffectKind::GoldGain { amount },
        id_source: None,
        target: Target::Direct(Some(id_character)),
    });
}
