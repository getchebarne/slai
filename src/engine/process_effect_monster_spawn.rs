use crate::consts::DISCOVER_PICK_COUNT;
use crate::consts::MAX_MONSTERS;
use crate::consts::MAX_SIZE_DECK;
use crate::consts::MAX_SIZE_HAND;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_apply;
use crate::monsters::spawn_monster;
use crate::types::Energy;
use crate::types::Mode;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::utils::has_relic;
use crate::utils::mode_top;
use crate::utils::mode_top_mut;
use crate::utils::push_entity;

pub fn process_effect_monster_spawn(state: &mut GameState, name: MonsterName, minion: bool) {
    // A monster spawning implies a combat: the first spawn of a fight constructs it
    if !matches!(mode_top(&state.mode_stack), Mode::Combat { .. }) {
        // Event fights replace the consumed Event frame; room fights push over Map
        let combat = Mode::Combat {
            id_hand: Vec::with_capacity(MAX_SIZE_HAND),
            id_pile_draw: Vec::with_capacity(MAX_SIZE_DECK),
            id_pile_discard: Vec::with_capacity(MAX_SIZE_DECK),
            id_pile_exhaust: Vec::with_capacity(MAX_SIZE_DECK),
            id_monsters: [None; MAX_MONSTERS],
            id_picked_monster: None,
            id_card_last_drawn: None,
            id_card_nightmare: None,
            id_discover: Vec::with_capacity(DISCOVER_PICK_COUNT as usize),
            energy: Energy {
                energy_current: 0,
                energy_max: 0,
            },
            this_turn_discards: 0,
            this_turn_attacks: 0,
            this_turn_cards_played: 0,
            this_turn_panache: 0,
            this_combat_damage_instances_taken: 0,
            this_combat_escaped: false,
            bomb_countdown: 0,
            event_gold: None,
            event_relic: None,
            event_relic_roll: false,
        };
        if matches!(mode_top(&state.mode_stack), Mode::Event { .. }) {
            state.mode_stack.pop();
        }
        state.mode_stack.push(combat);
    }
    let Mode::Combat { id_monsters, .. } = mode_top_mut(&mut state.mode_stack) else {
        unreachable!("Constructed above")
    };

    // Create the monster `Entity`; summons carry Minion from birth
    let mut monster = spawn_monster(name, state.ascension, &mut state.rng);
    if minion {
        modifier_apply(&mut monster.modifiers, ModifierKind::Minion, 1);
    }

    // Push it
    let id_monster = push_entity(&mut state.entities, monster);

    // Place it in the first empty monster slot
    let idx = id_monsters
        .iter()
        .position(|s| s.is_none())
        .expect("MonsterSpawn would overflow id_monsters: no empty idx");
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
