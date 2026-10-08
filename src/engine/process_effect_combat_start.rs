use crate::consts::ENERGY_MAX_BASE;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::relics::RELIC_COUNTERS_PER_COMBAT;
use crate::types::Combat;
use crate::types::DeltaSign;
use crate::types::Energy;
use crate::types::MonsterKind;
use crate::types::RelicName;
use crate::utils::has_relic;
use crate::utils::push_entity;
use crate::utils::shuffle;

pub fn process_effect_combat_start(state: &mut GameState, elite: bool) {
    // MonsterSpawn opens the combat reset; only what CombatStart computes is written here
    assert!(state.combat.active, "CombatStart outside combat");
    let Combat {
        id_card_draw,
        id_monsters,
        id_card_origins,
        energy,
        ..
    } = &mut state.combat;

    let is_fight_boss = id_monsters
        .iter()
        .flatten()
        .any(|&id| state.entities[id].monster_kind == MonsterKind::Boss);

    // Energy Relics: +1 for each owned one
    let mut energy_max = ENERGY_MAX_BASE;
    for name in [
        RelicName::PhilosopherStone,
        RelicName::CoffeeDripper,
        RelicName::FusionHammer,
        RelicName::Sozu,
        RelicName::CursedKey,
        RelicName::BustedCrown,
        RelicName::Ectoplasm,
        RelicName::VelvetChoker,
    ] {
        if has_relic(&state.id_relics, name) {
            energy_max += 1;
        }
    }

    // Slaver's Collar: +1 max energy in elite and boss fights only
    if has_relic(&state.id_relics, RelicName::SlaversCollar) && (elite || is_fight_boss) {
        energy_max += 1;
    }

    // Energy starts empty; the turn-1 refill fills it
    *energy = Energy {
        energy_current: 0,
        energy_max,
    };

    // Per-combat Relic counters start from zero; the per-turn ones reset at the turn start
    for &name in RELIC_COUNTERS_PER_COMBAT {
        if let Some(id) = state.id_relics[name as usize] {
            state.entities[id].relic_counter = 0;
        }
    }

    // Innate and bottled Cards sit on top of the draw pile, ahead of the shuffled rest
    let idx_other = id_card_draw.len();
    let mut ids_innate: Vec<usize> = Vec::new();
    for idx in 0..state.id_card_deck.len() {
        let id_card_src = state.id_card_deck[idx];
        let card = state.entities[id_card_src];
        let id_card = push_entity(&mut state.entities, card);
        id_card_origins.push((id_card, id_card_src));
        if card.card_innate || card.card_bottled {
            ids_innate.push(id_card);
        } else {
            id_card_draw.push(id_card);
        }
    }

    shuffle(&mut id_card_draw[idx_other..], &mut state.rng);
    shuffle(&mut ids_innate, &mut state.rng);
    id_card_draw.extend_from_slice(&ids_innate);

    // The whole opening roster stands now: its first moves roll ahead of everything
    state.effect_queue.push_front(Effect {
        kind: EffectKind::MoveUpdate {
            move_override: None,
        },
        id_source: None,
        target: TARGET_MONSTERS_ALL,
    });

    // Ancient Tea Set: primed by the last rest site (counter 1), spends it for 2 energy
    if let Some(id) = state.id_relics[RelicName::AncientTeaSet as usize]
        && state.entities[id].relic_counter == 1
    {
        state.entities[id].relic_counter = 0;
        state.effect_queue.push_back(Effect {
            kind: EffectKind::EnergyDelta {
                sign: DeltaSign::Gain,
                amount: 2,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }

    // Preserved Insect: elite Monsters above 3/4 HP drop to it
    if has_relic(&state.id_relics, RelicName::PreservedInsect) && elite {
        for id in id_monsters.iter().flatten().copied() {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::HealthLowerTo {
                    amount: Amount::Relative {
                        numerator: 3,
                        denominator: 4,
                    },
                },
                id_source: None,
                target: Target::Direct(Some(id)),
            });
        }
    }

    // Neow's Lament: spend a charge to set every spawned Monster to 1 HP (after Preserved Insect)
    if let Some(id) = state.id_relics[RelicName::NeowsLament as usize]
        && state.entities[id].relic_counter > 0
    {
        let relic = &mut state.entities[id];

        // Decrease counter
        relic.relic_counter -= 1;
        relic.relic_used_up = relic.relic_counter == 0;

        // One effect for every spawned Monster
        for id_monster in id_monsters.iter().flatten().copied() {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::HealthLowerTo {
                    amount: Amount::Absolute(1),
                },
                id_source: None,
                target: Target::Direct(Some(id_monster)),
            });
        }
    }

    // Girya: combats open with Strength equal to lifts
    if let Some(id) = state.id_relics[RelicName::Girya as usize]
        && state.entities[id].relic_counter > 0
    {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Strength,
                stacks: state.entities[id].relic_counter,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Du-Vu Doll: combat starts with 1 Strength per Curse its counter holds
    if let Some(id) = state.id_relics[RelicName::DuVuDoll as usize]
        && state.entities[id].relic_counter > 0
    {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Strength,
                stacks: state.entities[id].relic_counter,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Sling of Courage: Elite fights open with 2 Strength
    if has_relic(&state.id_relics, RelicName::SlingOfCourage) && elite {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Strength,
                stacks: 2,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // The first TurnStartCharacter waits for everything already queued for the combat start
    state.effect_queue.push_back(Effect {
        kind: EffectKind::TurnStartCharacter,
        id_source: None,
        target: Target::Direct(None),
    });
}
