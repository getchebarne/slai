use strum::EnumCount;

use crate::cards::get_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::engine::process_effect_queue;
use crate::entity::Entity;
use crate::game::GameState;
use crate::game::create_game_state;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_stacks;
use crate::types::CardName;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::relics::get_relic;
use crate::utils::push_entity;

// Register ownership directly, skipping on-pickup effects (tests drive those explicitly)
pub fn grant_relic(
    name: RelicName,
    id_relics: &mut [Option<usize>; RelicName::COUNT],
    entities: &mut Vec<Entity>,
) {
    let id = push_entity(entities, get_relic(name));
    id_relics[name as usize] = Some(id);
}

pub fn combat_with_relic(relic: RelicName, monster: MonsterName) -> GameState {
    let mut state = create_game_state(0, 42, false);
    grant_relic(relic, &mut state.id_relics, &mut state.entities);
    for kind in [
        EffectKind::MonsterSpawn { name: monster },
        EffectKind::CombatStart,
    ] {
        state.effect_queue.push_back(Effect {
            kind,
            id_source: None,
            target: Target::Direct(None),
        });
    }
    process_effect_queue(&mut state);
    state
}

pub fn put_in_hand(state: &mut GameState, name: CardName) -> usize {
    let id = push_entity(&mut state.entities, get_card(name, false));
    state.id_hand.push(id);
    id
}

// Refill energy and play via the real TargetSet -> CardPlay -> TargetClear triple
pub fn play(state: &mut GameState, id_card: usize) {
    let id_monster = state
        .id_monsters
        .iter()
        .flatten()
        .copied()
        .next()
        .expect("combat has a monster");
    state.energy.energy_current = 3;
    for effect in [
        Effect {
            kind: EffectKind::TargetSet,
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        },
        Effect {
            kind: EffectKind::CardPlay,
            id_source: None,
            target: Target::Direct(Some(id_card)),
        },
        Effect {
            kind: EffectKind::TargetClear,
            id_source: None,
            target: Target::Direct(None),
        },
    ] {
        state.effect_queue.push_back(effect);
    }
    process_effect_queue(state);
}

// Runs the full round: character turn end, monster turns, next character turn start
pub fn end_turn(state: &mut GameState) {
    state.effect_queue.push_back(Effect {
        kind: EffectKind::TurnEnd,
        id_source: None,
        target: Target::Direct(Some(state.id_character)),
    });
    process_effect_queue(state);
}

pub fn set_relic_counter(state: &mut GameState, relic: RelicName, value: i16) {
    let id = state.id_relics[relic as usize].expect("relic owned");
    state.entities[id].relic_counter = value;
}

pub fn char_modifier(state: &GameState, kind: ModifierKind) -> i16 {
    modifier_stacks(&state.entities[state.id_character].modifiers, kind)
}

pub fn first_monster(state: &GameState) -> usize {
    state
        .id_monsters
        .iter()
        .flatten()
        .copied()
        .next()
        .expect("combat has a monster")
}
