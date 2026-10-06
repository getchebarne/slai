use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::monsters::spawn_monster;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::utils::has_relic;
use crate::utils::place_monster;
use crate::utils::push_entity;

pub fn process_effect_monster_split(
    id_source: Option<usize>,
    state: &mut GameState,
    name: MonsterName,
    dx: i16,
) {
    assert!(
        state.combat.active,
        "process_effect_monster_split outside the Combat frame"
    );
    let id_source = id_source.expect("MonsterSplit requires id_source");

    // Check that the split Monster is a slime
    assert!(
        matches!(
            state.entities[id_source].monster_name,
            MonsterName::SlimeAcidLarge | MonsterName::SlimeSpikeLarge | MonsterName::SlimeBoss
        ),
        "MonsterSplit id_source must be a splitting slime, got {:?}",
        state.entities[id_source].monster_name,
    );

    // Create the child with the parent's current health, standing `dx` right of the parent
    let mut monster = spawn_monster(name, state.ascension, &mut state.rng);
    monster.vitals.health = state.entities[id_source].vitals.health;
    monster.vitals.health_max = state.entities[id_source].vitals.health;
    monster.monster_x = state.entities[id_source].monster_x + dx;
    let id_monster = push_entity(&mut state.entities, monster);
    place_monster(&mut state.combat, &state.entities, id_monster);

    // Philosopher's Stone: split children get the Strength too
    if has_relic(&state.id_relics, RelicName::PhilosopherStone) {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: 1,
            },
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
    }

    // Queue an effect to update its move
    state.effect_queue.push_front(Effect {
        kind: EffectKind::MoveUpdate {
            move_override: None,
        },
        id_source: None,
        target: Target::Direct(Some(id_monster)),
    });
}
