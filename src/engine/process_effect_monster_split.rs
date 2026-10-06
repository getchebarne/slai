use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::monsters::spawn_monster;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::utils::has_relic;
use crate::utils::push_entity;

pub fn process_effect_monster_split(
    id_source: Option<usize>,
    state: &mut GameState,
    name: MonsterName,
    slot_offset: isize,
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

    // The child lands at the parent's slot plus its offset; offset 0 takes over the parent's slot
    let slot_parent = state
        .combat
        .id_monsters
        .iter()
        .position(|&slot| slot == Some(id_source))
        .expect("MonsterSplit parent must be on the roster");
    let slot = slot_parent
        .checked_add_signed(slot_offset)
        .expect("MonsterSplit child slot must be on the roster");
    assert!(
        slot == slot_parent || state.combat.id_monsters[slot].is_none(),
        "MonsterSplit child slot {slot} is taken"
    );

    // Create the child with the parent's current health
    let mut monster = spawn_monster(name, state.ascension, &mut state.rng);
    monster.vitals.health = state.entities[id_source].vitals.health;
    monster.vitals.health_max = state.entities[id_source].vitals.health;
    let id_monster = push_entity(&mut state.entities, monster);
    state.combat.id_monsters[slot] = Some(id_monster);

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
