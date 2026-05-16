use std::collections::VecDeque;

use rand::Rng;

use crate::consts::GOLD_BOSS_MAX;
use crate::consts::GOLD_BOSS_MIN;
use crate::consts::MAP_HEIGHT;
use crate::consts::MAP_WIDTH;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::game::Location;
use crate::map::get_active_room_kind;
use crate::modifier::modifier_clear;
use crate::types::Phase;
use crate::types::RoomKind;

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
    rng: &mut impl Rng,
    effect_queue: &mut VecDeque<Effect>,
) -> Option<Phase> {
    id_hand.clear();
    id_pile_draw.clear();
    id_pile_discard.clear();
    id_pile_exhaust.clear();
    *id_card_nightmare = None;
    *id_card_target = None;

    modifier_clear(&mut entities[id_character].modifiers);

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

    *monster_count = 0;

    let room_kind = get_active_room_kind(id_rooms, location, entities).unwrap();
    match room_kind {
        RoomKind::CombatBoss => {
            // Boss defeated — drop any pending effects. derive_phase
            // returns GameOver from `location == BossRoom && monster_count == 0`
            effect_queue.clear();
            let amount = rng.random_range(GOLD_BOSS_MIN..=GOLD_BOSS_MAX);
            effect_queue.push_back(Effect {
                kind: EffectKind::GoldGain { amount },
                id_source: None,
                target: Target::Direct(Some(id_character)),
            });
        }
        RoomKind::CombatMonster | RoomKind::CombatElite => {
            effect_queue.push_back(Effect::direct(
                EffectKind::RewardRollCombat { room_kind },
                None,
                None,
            ));
        }
        RoomKind::RestSite | RoomKind::Treasure | RoomKind::EventRoom | RoomKind::Shop => {
            unreachable!("combat end in non-combat room: {:?}", room_kind)
        }
    }
    None
}
