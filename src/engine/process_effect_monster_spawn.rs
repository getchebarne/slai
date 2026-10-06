use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_apply;
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
    slot: usize,
) {
    // A Monster spawning implies a combat: the first spawn of a fight opens it
    if !state.combat.active {
        combat_reset(&mut state.combat);
        state.combat.active = true;
    }

    // A taken slot fizzles the spawn: Revive refills only fallen Torch Heads
    if state.combat.id_monsters[slot].is_some() {
        return;
    }

    // Create the Monster `Entity`; summons carry Minion from birth
    let mut monster = spawn_monster(name, state.ascension, &mut state.rng);
    if minion {
        modifier_apply(&mut monster.modifiers, ModifierKind::Minion, 1);
    }
    let id_monster = push_entity(&mut state.entities, monster);
    state.combat.id_monsters[slot] = Some(id_monster);

    // The opening roster rolls together at CombatStart; arrivals into a running fight roll at once
    if state.combat.turn > 0 {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::MoveUpdate {
                move_override: None,
            },
            id_source: None,
            target: Target::Direct(Some(id_monster)),
        });
    }

    // Philosopher's Stone: every Monster (including mid-combat spawns) gains 1 Strength
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
