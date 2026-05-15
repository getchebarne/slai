use std::collections::VecDeque;

use rand::Rng;

use crate::consts::ELITE_TH_COMMON;
use crate::consts::ELITE_TH_UNCOMMON;
use crate::consts::GOLD_BOSS_MAX;
use crate::consts::GOLD_BOSS_MIN;
use crate::consts::GOLD_ELITE_MAX;
use crate::consts::GOLD_ELITE_MIN;
use crate::consts::GOLD_MONSTER_MAX;
use crate::consts::GOLD_MONSTER_MIN;
use crate::consts::MAP_HEIGHT;
use crate::consts::MAP_WIDTH;
use crate::consts::POTION_DROP_CHANCE_BASE;
use crate::consts::POTION_DROP_CHANCE_MOD_HIT;
use crate::consts::POTION_DROP_CHANCE_MOD_MAX;
use crate::consts::POTION_DROP_CHANCE_MOD_MIN;
use crate::consts::POTION_DROP_CHANCE_MOD_MISS;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::game::Location;
use crate::map::get_active_room_kind;
use crate::modifier::modifier_clear;
use crate::types::RoomKind;
use crate::types::Phase;

#[allow(clippy::too_many_arguments)]
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
    potion_drop_mod: &mut i8,
    rng: &mut impl Rng,
    effect_queue: &mut VecDeque<Effect>,
) -> Option<Phase> {
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
            push_gold_gain(
                rng,
                GOLD_BOSS_MIN,
                GOLD_BOSS_MAX,
                id_character,
                effect_queue,
            );
        }
        RoomKind::CombatMonster => {
            let gold_range = if escaped_this_combat {
                None
            } else {
                Some((GOLD_MONSTER_MIN, GOLD_MONSTER_MAX))
            };
            let potion_drop = roll_potion_drop(rng, potion_drop_mod);
            effect_queue.push_back(Effect {
                kind: EffectKind::RewardRollCombat {
                    gold_range,
                    relic_thresholds: None,
                    potion_drop,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }
        RoomKind::CombatElite => {
            let potion_drop = roll_potion_drop(rng, potion_drop_mod);
            effect_queue.push_back(Effect {
                kind: EffectKind::RewardRollCombat {
                    gold_range: Some((GOLD_ELITE_MIN, GOLD_ELITE_MAX)),
                    relic_thresholds: Some((ELITE_TH_COMMON, ELITE_TH_UNCOMMON)),
                    potion_drop,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }
        RoomKind::RestSite | RoomKind::Treasure | RoomKind::EventRoom | RoomKind::Shop => {
            unreachable!("combat end in non-combat room: {:?}", room)
        }
    }
    None
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

// +10 on miss, -10 on hit; clamps to [-30, +60] ([10%, 100%])
fn roll_potion_drop(rng: &mut impl Rng, potion_drop_mod: &mut i8) -> bool {
    let roll = rng.random_range(0..100) as u8;
    let chance = (POTION_DROP_CHANCE_BASE as i16 + *potion_drop_mod as i16).clamp(0, 100) as u8;

    if roll < chance {
        *potion_drop_mod = (*potion_drop_mod + POTION_DROP_CHANCE_MOD_HIT)
            .clamp(POTION_DROP_CHANCE_MOD_MIN, POTION_DROP_CHANCE_MOD_MAX);
        true
    } else {
        *potion_drop_mod = (*potion_drop_mod + POTION_DROP_CHANCE_MOD_MISS)
            .clamp(POTION_DROP_CHANCE_MOD_MIN, POTION_DROP_CHANCE_MOD_MAX);
        false
    }
}
