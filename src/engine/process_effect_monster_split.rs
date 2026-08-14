use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::monsters::spawn_monster;
use crate::types::Frame;
use crate::types::MonsterName;
use crate::utils::frame_top_mut;
use crate::utils::push_entity;

pub fn process_effect_monster_split(
    id_source: Option<usize>,
    state: &mut GameState,
    name: MonsterName,
) {
    let Frame::Combat { id_monsters, .. } = frame_top_mut(&mut state.frame_stack) else {
        unreachable!("process_effect_monster_split outside the Combat frame")
    };
    let id_source = id_source.expect("MonsterSplit requires id_source");

    // Check that the split monster is a slime
    assert!(
        matches!(
            state.entities[id_source].monster_name,
            MonsterName::SlimeAcidLarge | MonsterName::SlimeSpikeLarge | MonsterName::SlimeBoss
        ),
        "MonsterSplit id_source must be a splitting slime, got {:?}",
        state.entities[id_source].monster_name,
    );

    // Create the child with the parent's current health
    let mut monster = spawn_monster(name, state.ascension, &mut state.rng);
    monster.vitals.health = state.entities[id_source].vitals.health;
    monster.vitals.health_max = state.entities[id_source].vitals.health;

    // Push it
    let id_monster = push_entity(&mut state.entities, monster);

    // Place it in the first empty monster slot
    let idx = id_monsters
        .iter()
        .position(|s| s.is_none())
        .expect("MonsterSplit would overflow id_monsters: no empty idx");
    id_monsters[idx] = Some(id_monster);

    // Queue an effect to update its move
    state.effect_queue.push_front(Effect {
        kind: EffectKind::MoveUpdate {
            move_override: None,
        },
        id_source: None,
        target: Target::Direct(Some(id_monster)),
    });
}
