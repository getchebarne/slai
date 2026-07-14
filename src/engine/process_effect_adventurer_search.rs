use rand::Rng;

use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::CandidatePoolMonstersFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::ADVENTURER_REWARD_GOLD;
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
    let searches_done = state.entities[id_event].event_state as u16;
    let base: u16 = if state.ascension < 15 { 25 } else { 35 };
    let chance = base + 25 * searches_done;

    if (state.rng.random_range(0..100) as u16) < chance {
        // The elite returns. The event stays unconsumed: combat-end reward
        // injection still needs event_state and event_rolls
        let encounter = adventurer_enemy_encounter(state.event_rolls[0]);
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

    // No encounter: grant the next hidden reward and advance the stage
    match state.event_rolls[1 + searches_done as usize] {
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
        _ => {}
    }
    state.entities[id_event].event_state += 1;

    // All three rewards found: the event is over
    if state.entities[id_event].event_state >= 3 {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::EventConsume,
            id_source: Some(id_event),
            target: Target::Direct(None),
        });
    }
}
