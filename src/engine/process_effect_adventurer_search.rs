use rand::Rng;

use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::CandidatePoolMonstersFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::DeadAdventurerReward;
use crate::events::EventPayload;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::monsters::encounters::spawn_encounter_monsters;
use crate::monsters::lagavulin;
use crate::types::DeltaSign;
use crate::types::MonsterEncounter;

// Dead Adventurer search: escalating chance the pre-rolled elite returns
pub fn process_effect_adventurer_search(state: &mut GameState) {
    let event = state
        .event
        .expect("AdventurerSearch without an active event");
    let EventPayload::DeadAdventurer {
        encounter,
        rewards,
        rewards_len,
        searches,
    } = event.payload
    else {
        unreachable!(
            "AdventurerSearch on non-adventurer event: {:?}",
            event.payload
        )
    };

    let base: u16 = if state.ascension < 15 { 25 } else { 35 };
    let chance = base + 25 * searches as u16;

    if (state.rng.random_range(0..100) as u16) < chance {
        // Spawn elite. The event stays unconsumed — combat-end reward roll still needs its state
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
    let drawn_idx = state.rng.random_range(0..rewards_len as usize);
    match rewards[drawn_idx] {
        DeadAdventurerReward::Gold => state.effect_queue.push_front(Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(30),
            },
            id_source: None,
            target: Target::Direct(None),
        }),
        DeadAdventurerReward::Relic => state.effect_queue.push_front(Effect {
            kind: EffectKind::RelicGrantRandom,
            id_source: None,
            target: Target::Direct(None),
        }),
        DeadAdventurerReward::Nothing => {}
    }

    // Swap-remove the drawn slot and advance the search count (drives the chance)
    let event = state.event.as_mut().expect("checked above");
    let EventPayload::DeadAdventurer {
        rewards,
        rewards_len,
        searches,
        ..
    } = &mut event.payload
    else {
        unreachable!("checked above")
    };
    rewards[drawn_idx] = rewards[*rewards_len as usize - 1];
    *rewards_len -= 1;
    *searches += 1;

    // All rewards found: the event is over
    if *rewards_len == 0 {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::EventConsume,
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
