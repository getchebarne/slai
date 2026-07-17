use rand::Rng;

use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::CandidatePoolMonstersFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::ADVENTURER_IDX_ENEMY;
use crate::events::ADVENTURER_IDX_REWARDS;
use crate::events::ADVENTURER_REWARD_GOLD;
use crate::events::ADVENTURER_REWARD_NOTHING;
use crate::events::ADVENTURER_REWARD_RELIC;
use crate::events::adventurer_enemy_encounter;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::monsters::encounters::spawn_encounter_monsters;
use crate::monsters::lagavulin;
use crate::types::DeltaSign;
use crate::types::MonsterEncounter;

// Dead Adventurer search: escalating chance the pre-rolled elite returns.
// event_state counts completed searches; event_rolls = [enemy, reward0..2]
pub fn process_effect_adventurer_search(id_source: Option<usize>, state: &mut GameState) {
    let id_event = id_source.expect("AdventurerSearch requires id_source");
    let search_num = state.entities[id_event].event_state as u16;
    let base: u16 = if state.ascension < 15 { 25 } else { 35 };
    let chance = base + 25 * search_num;

    if (state.rng.random_range(0..100) as u16) < chance {
        // Spawn elite. The event stays unconsumed — combat-end reward roll still needs its state
        let encounter =
            adventurer_enemy_encounter(state.entities[id_event].event_rolls[ADVENTURER_IDX_ENEMY]);
        spawn_encounter_monsters(state, encounter);

        // The event Lagavulin spawns awake: no sleep kit, opens with its attack
        if encounter == MonsterEncounter::Lagavulin {
            let all_monsters = Target::Resolve {
                candidate_pool: CandidatePool::Monsters {
                    filter: CandidatePoolMonstersFilter::All,
                },
                selection_kind: SelectionKind::All,
            };
            for kind in [
                EffectKind::ModifierRemove {
                    kind: ModifierKind::Asleep,
                },
                EffectKind::ModifierRemove {
                    kind: ModifierKind::Metallicize,
                },
                EffectKind::MoveUpdate {
                    move_override: Some(lagavulin::IDX_MOVE_ATTACK),
                },
            ] {
                state.effect_queue.push_back(Effect {
                    kind,
                    id_source: None,
                    target: all_monsters,
                });
            }
        }
        return;
    }

    // No encounter: draw one of the remaining rewards and remove it from the pool
    let event = &state.entities[id_event];
    let num_remaining = event.event_rolls_len as usize - ADVENTURER_IDX_REWARDS;
    let drawn_idx = ADVENTURER_IDX_REWARDS + state.rng.random_range(0..num_remaining);
    let reward = state.entities[id_event].event_rolls[drawn_idx];
    match reward {
        ADVENTURER_REWARD_GOLD => state.effect_queue.push_front(Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(30),
            },
            id_source: None,
            target: Target::Direct(None),
        }),
        ADVENTURER_REWARD_RELIC => state.effect_queue.push_front(Effect {
            kind: EffectKind::RelicGrantRandom,
            id_source: None,
            target: Target::Direct(None),
        }),
        ADVENTURER_REWARD_NOTHING => {}
        roll => unreachable!("adventurer reward roll out of range: {roll}"),
    }

    // Swap-remove the drawn slot and advance the search count (drives the chance)
    let event = &mut state.entities[id_event];
    event.event_rolls[drawn_idx] = event.event_rolls[event.event_rolls_len as usize - 1];
    event.event_rolls_len -= 1;
    event.event_state += 1;

    // All rewards found: the event is over
    if event.event_rolls_len as usize == ADVENTURER_IDX_REWARDS {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::EventConsume,
            id_source: Some(id_event),
            target: Target::Direct(None),
        });
    }
}
