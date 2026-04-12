use crate::effect::{Effect, EffectKind, Target};
use crate::engine::ProcessEffectResult;
use crate::modifier::modifier_clear;
use crate::state::{Entity, Map};
use crate::types::{EntityId, Phase, RoomType};

pub fn process_effect_combat_end(
    character: EntityId,
    hand: &mut Vec<EntityId>,
    draw_pile: &mut Vec<EntityId>,
    discard_pile: &mut Vec<EntityId>,
    exhaust_pile: &mut Vec<EntityId>,
    card_target: &mut Option<EntityId>,
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
    let room = map.active_room_type(entities).unwrap();

    // Clear character modifiers and monsters
    let (_, modifiers) = entities[character.0 as usize].kind.combatant_mut();
    modifier_clear(modifiers);
    *monster_count = 0;
    match room {
        RoomType::CombatBoss => ProcessEffectResult::Halt {
            phase_new: Phase::GameOver,
        },
        RoomType::CombatMonster => ProcessEffectResult::AddAndContinue {
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
