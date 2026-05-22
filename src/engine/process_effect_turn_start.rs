use crate::consts::CARDS_DRAWN_PER_TURN;
use crate::consts::MAX_MONSTERS;
use crate::effect::CandidatePool;
use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::engine::flush_effects_from_buf_to_queue_front;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;
use crate::modifier::modifier_remove;
use crate::modifier::modifier_stacks;
use crate::types::CardName;
use crate::utils::fill_alive_monster_ids;

pub fn process_effect_turn_start(id_target: Option<usize>, state: &mut GameState) {
    let id_actor = id_target.expect("TurnStart requires id_target");
    let id_character = state.id_character;
    let energy_max = state.energy.max;
    let energy_current = state.energy.current;
    let nightmare_pending = state.id_card_nightmare.is_some();

    let mut buf_alive = [0usize; MAX_MONSTERS];
    let alive_n = fill_alive_monster_ids(state, &mut buf_alive);
    let id_monsters = &buf_alive[..alive_n];

    state.buf_effects.clear();

    let entity = &mut state.entities[id_actor];
    let modifiers = &mut entity.modifiers;
    let vitals = &mut entity.vitals;

    if modifier_has(modifiers, ModifierKind::Poison) {
        state
            .buf_effects
            .push(Effect::direct(EffectKind::PoisonTick, None, Some(id_actor)));
    }

    let mut new_block: u16 = 0;
    if modifier_has(modifiers, ModifierKind::Blur) {
        new_block += vitals.block;
    }
    if modifier_has(modifiers, ModifierKind::NextTurnBlock) {
        new_block += modifier_stacks(modifiers, ModifierKind::NextTurnBlock) as u16;
        modifier_remove(modifiers, ModifierKind::NextTurnBlock);
    }
    state.buf_effects.push(Effect {
        kind: EffectKind::BlockSet { amount: new_block },
        id_source: None,
        target: Target::Direct(Some(id_actor)),
    });

    if modifier_has(modifiers, ModifierKind::Phantasmal) {
        state.buf_effects.push(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::DoubleDamage,
                stacks: 1,
            },
            id_source: None,
            target: Target::Direct(Some(id_actor)),
        });
    }

    if id_actor == id_character {
        state.buf_effects.push(Effect {
            kind: EffectKind::CardDraw {
                count: CARDS_DRAWN_PER_TURN,
            },
            id_source: None,
            target: Target::Direct(None),
        });
        let energy_gain = energy_max.saturating_sub(energy_current);
        state.buf_effects.push(Effect {
            kind: EffectKind::EnergyGain {
                amount: energy_gain as u16,
            },
            id_source: None,
            target: Target::Direct(None),
        });

        state.buf_effects.push(Effect {
            kind: EffectKind::ModifierTick,
            id_source: None,
            target: Target::Direct(Some(id_character)),
        });
        for &id_monster in id_monsters {
            state.buf_effects.push(Effect {
                kind: EffectKind::ModifierTick,
                id_source: None,
                target: Target::Direct(Some(id_monster)),
            });
        }

        if modifier_has(modifiers, ModifierKind::NoxiousFumes) {
            let stacks = modifier_stacks(modifiers, ModifierKind::NoxiousFumes);
            for &id_monster in id_monsters {
                state.buf_effects.push(Effect {
                    kind: EffectKind::ModifierGain {
                        kind: ModifierKind::Poison,
                        stacks,
                    },
                    id_source: None,
                    target: Target::Direct(Some(id_monster)),
                });
            }
        }

        // Choke auto-removes at the next player turn start
        for &id_monster in id_monsters {
            state.buf_effects.push(Effect {
                kind: EffectKind::ModifierRemove {
                    kind: ModifierKind::Choke,
                },
                id_source: None,
                target: Target::Direct(Some(id_monster)),
            });
        }

        if nightmare_pending {
            state.buf_effects.push(Effect {
                kind: EffectKind::CardNightmareSpawn,
                id_source: None,
                target: Target::Direct(None),
            });
        }

        if modifier_has(modifiers, ModifierKind::DrawCardNextTurn) {
            let stacks = modifier_stacks(modifiers, ModifierKind::DrawCardNextTurn);
            state.buf_effects.push(Effect {
                kind: EffectKind::CardDraw {
                    count: stacks.max(0) as u16,
                },
                id_source: None,
                target: Target::Direct(None),
            });
            state.buf_effects.push(Effect {
                kind: EffectKind::ModifierRemove {
                    kind: ModifierKind::DrawCardNextTurn,
                },
                id_source: None,
                target: Target::Direct(Some(id_actor)),
            });
        }

        if modifier_has(modifiers, ModifierKind::ToolsOfTheTrade) {
            let stacks = modifier_stacks(modifiers, ModifierKind::ToolsOfTheTrade);
            state.buf_effects.push(Effect {
                kind: EffectKind::CardDraw {
                    count: stacks.max(0) as u16,
                },
                id_source: None,
                target: Target::Direct(None),
            });
            state.buf_effects.push(Effect {
                kind: EffectKind::CardDiscard {
                    source: DiscardSource::Explicit,
                },
                id_source: None,
                target: Target::Resolve {
                    candidates: CandidatePool::Hand,
                    selection: SelectionKind::Input {
                        count: stacks.max(0) as u16,
                    },
                },
            });
        }

        if modifier_has(modifiers, ModifierKind::NextTurnEnergy) {
            let stacks = modifier_stacks(modifiers, ModifierKind::NextTurnEnergy);
            state.buf_effects.push(Effect {
                kind: EffectKind::EnergyGain {
                    amount: stacks.max(0) as u16,
                },
                id_source: None,
                target: Target::Direct(None),
            });
            modifier_remove(modifiers, ModifierKind::NextTurnEnergy);
        }

        if modifier_has(modifiers, ModifierKind::InfiniteBlades) {
            let stacks = modifier_stacks(modifiers, ModifierKind::InfiniteBlades);
            state.buf_effects.push(Effect {
                kind: EffectKind::CardAddToHand {
                    card_name: CardName::Shiv,
                    count: stacks.max(0) as u16,
                    upgraded: false,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }
    }

    flush_effects_from_buf_to_queue_front(state);
}
