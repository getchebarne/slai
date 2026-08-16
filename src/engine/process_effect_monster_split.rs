use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::monsters::spawn_monster;
use crate::types::Combat;
use crate::types::MonsterName;
use crate::utils::push_entity;

pub fn process_effect_monster_split(
    id_source: Option<usize>,
    state: &mut GameState,
    name: MonsterName,
) {
    assert!(
        state.combat.active,
        "process_effect_monster_split outside the Combat frame"
    );
    let Combat { id_monsters, .. } = &mut state.combat;
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
        .position(|slot| slot.is_none())
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
