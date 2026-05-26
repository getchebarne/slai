use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::utils::push_entity;
use crate::game::Energy;
use crate::game::GameState;
use crate::monsters::spawn_monster;
use crate::types::Screen;
use crate::types::MonsterName;

pub fn process_effect_monster_spawn(
    id_source: Option<usize>,
    state: &mut GameState,
    name: MonsterName,
) {
    // First spawn of a combat installs the Combat context
    if !matches!(state.screen, Screen::Combat) {
        state.screen = Screen::Combat;
        state.energy = Energy { current: 3, max: 3 };
    }

    let mut monster_child = spawn_monster(name, state.ascension, &mut state.rng);

    // Slime split: spawned child inherits the parent's current HP as max
    // health. Only the three splitting slimes use this path; gate on the
    // source's monster_name so a future non-slime caller passing a source
    // doesn't silently inherit HP
    if let Some(id) = id_source {
        let parent = &state.entities[id];
        assert!(
            matches!(
                parent.monster_name,
                MonsterName::SlimeAcidLarge | MonsterName::SlimeSpikeLarge | MonsterName::SlimeBoss
            ),
            "MonsterSpawn id_source must be a splitting slime, got {:?}",
            parent.monster_name,
        );
        let health_parent = parent.vitals.health;
        monster_child.vitals.health = health_parent;
        monster_child.vitals.health_max = health_parent;
    }

    let id_child = push_entity(&mut state.entities, monster_child);

    let slot = state
        .id_monsters
        .iter()
        .position(|s| s.is_none())
        .expect("MonsterSpawn would overflow id_monsters: no empty slot");
    state.id_monsters[slot] = Some(id_child);

    state.effect_queue.push_front(Effect {
        kind: EffectKind::MoveUpdate,
        id_source: None,
        target: Target::Direct(Some(id_child)),
    });
}
