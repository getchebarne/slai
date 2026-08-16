use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_apply;
use crate::monsters::count_monsters_named;
use crate::monsters::spawn_monster;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::types::combat_reset;
use crate::utils::has_relic;
use crate::utils::push_entity;

pub fn process_effect_monster_spawn(
    state: &mut GameState,
    name: MonsterName,
    minion: bool,
    cap: Option<u8>,
) {
    // A monster spawning implies a combat: the first spawn of a fight opens it
    if !state.combat.active {
        combat_reset(&mut state.combat);
        state.combat.active = true;
    }
    let id_monsters = &mut state.combat.id_monsters;

    // Capped spawns top the roster up (Collector's Torch Heads): skip at the cap
    if let Some(cap) = cap
        && count_monsters_named(&state.entities, id_monsters, name) >= cap as usize
    {
        return;
    }

    // A full roster fizzles the spawn (Collector revive, mid-combat summons)
    let Some(idx) = id_monsters.iter().position(|slot| slot.is_none()) else {
        return;
    };

    // Create the monster `Entity`; summons carry Minion from birth
    let mut monster = spawn_monster(name, state.ascension, &mut state.rng);
    if minion {
        modifier_apply(&mut monster.modifiers, ModifierKind::Minion, 1);
    }
    let id_monster = push_entity(&mut state.entities, monster);
    id_monsters[idx] = Some(id_monster);

    // Queue an effect to update its move
    state.effect_queue.push_front(Effect {
        kind: EffectKind::MoveUpdate {
            move_override: None,
        },
        id_source: None,
        target: Target::Direct(Some(id_monster)),
    });

    // Philosopher's Stone: every monster (including mid-combat spawns) gains 1 Strength
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
}
