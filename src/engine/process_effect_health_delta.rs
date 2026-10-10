use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::EntityKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::modifier::modifier_remove;
use crate::modifier::modifier_stacks;
use crate::monsters::lagavulin;
use crate::monsters::slime_acid_large;
use crate::monsters::slime_boss;
use crate::monsters::slime_spike_large;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::DeltaSign;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::utils::cards_grow_on_damage;
use crate::utils::get_id_actor;
use crate::utils::has_relic;
use crate::utils::resolve_health_fraction;

pub fn process_effect_health_delta(
    id_source: Option<usize>,
    id_target: Option<usize>,
    state: &mut GameState,
    sign: DeltaSign,
    amount: Amount,
) {
    let id_target = id_target.expect("HealthDelta requires id_target");

    // Resolve amount
    let amount = resolve_health_fraction(
        state.entities[id_target].vitals.health_max,
        state.event.health_max_at_open,
        amount,
    );

    // Apply amount
    match sign {
        DeltaSign::Gain => apply_gain(id_target, state, amount),
        DeltaSign::Loss => apply_loss(id_source, id_target, state, amount),
    }
}

fn apply_gain(id_target: usize, state: &mut GameState, amount: u16) {
    let vitals = &mut state.entities[id_target].vitals;
    vitals.health = (vitals.health + amount).min(vitals.health_max);
}

fn apply_loss(id_source: Option<usize>, id_target: usize, state: &mut GameState, amount: u16) {
    // Corpses take no losses: a re-queued Death would double the on-death triggers
    if state.entities[id_target].dead {
        return;
    }

    // A Card's hit on a Monster: its Intangible was clamped before block, ahead of The Boot's lift
    let from_card = id_source.is_some_and(|id| state.entities[id].kind == EntityKind::Card);
    let card_hit_on_monster = from_card && state.entities[id_target].kind == EntityKind::Monster;

    // Intangible clamps every other incoming instance, HP loss included
    let amount = if amount > 1
        && !card_hit_on_monster
        && has_modifier(
            &state.entities[id_target].modifiers,
            ModifierKind::Intangible,
        ) {
        1
    } else {
        amount
    };

    // Buffer: absorb one HP-loss instance outright, before anything reacts to it; nothing is lost
    if amount > 0 {
        let modifiers = &mut state.entities[id_target].modifiers;
        if has_modifier(modifiers, ModifierKind::Buffer) {
            if modifier_stacks(modifiers, ModifierKind::Buffer) <= 1 {
                modifier_remove(modifiers, ModifierKind::Buffer);
            } else {
                modifiers.stacks[ModifierKind::Buffer as usize] -= 1;
            }
            state.combat.last_health_lost = 0;
            return;
        }
    }

    // Tungsten Rod: every HP loss is reduced by 1, before anything reacts to it
    let amount = if id_target == state.id_character
        && amount > 0
        && has_relic(&state.id_relics, RelicName::TungstenRod)
    {
        amount - 1
    } else {
        amount
    };

    // Painful Stabs: a Monster's hit that costs the Character HP adds a Wound, once the Puzzle's draw is done
    if id_target == state.id_character
        && amount > 0
        && let Some(id_source) = id_source
        && state.entities[id_source].kind == EntityKind::Monster
        && has_modifier(
            &state.entities[id_source].modifiers,
            ModifierKind::PainfulStabs,
        )
    {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardAdd {
                card_name: CardName::Wound,
                card_pile: CardPile::Discard,
                count: 1,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }

    // Centennial Puzzle: the first actual HP loss each combat draws 3
    if id_target == state.id_character
        && amount > 0
        && state.combat.active
        && let Some(id_relic) = state.id_relics[RelicName::CentennialPuzzle as usize]
        && state.entities[id_relic].relic_counter == 0
    {
        state.entities[id_relic].relic_counter = 1;
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardDraw { count: 3 },
            id_source: None,
            target: Target::Direct(None),
        });
    }

    if id_target == state.id_character && amount > 0 && state.combat.active {
        cards_grow_on_damage(state);
    }

    // Plated Armor: only foreign attack damage strips a stack
    // TODO: improve this provenance
    let id_source_actor = id_source.map(|id| get_id_actor(&state.entities, state.id_character, id));
    let from_attack = match id_source_actor {
        Some(id) => {
            id != id_target
                && matches!(
                    state.entities[id].kind,
                    EntityKind::Character | EntityKind::Monster
                )
        }
        None => false,
    };
    let target = &mut state.entities[id_target];
    if from_attack && amount > 0 && has_modifier(&target.modifiers, ModifierKind::PlatedArmor) {
        // The strip lands behind everything queued, the rest of a Card included
        state.effect_queue.push_back(Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::PlatedArmor,
                stacks: -1,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }

    // Substract health
    let health_lost = amount.min(target.vitals.health);
    target.vitals.health = target.vitals.health.saturating_sub(amount);
    if state.combat.active {
        state.combat.last_health_lost = health_lost;
    }

    // Check if the target's dead. If so, queue death effect and return early
    if target.vitals.health == 0 {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::Death { with_leader: false },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
        return;
    }

    // Splittable: any damage at <= health_max / 2 overrides next move to Split. Splittable
    // stays until the split; a pending Split keeps a multi-hit from retriggering it
    if has_modifier(&target.modifiers, ModifierKind::Splittable)
        && target.vitals.health <= target.vitals.health_max / 2
    {
        let idx_split = match target.monster_name {
            MonsterName::SlimeAcidLarge => slime_acid_large::IDX_MOVE_SPLIT,
            MonsterName::SlimeSpikeLarge => slime_spike_large::IDX_MOVE_SPLIT,
            MonsterName::SlimeBoss => slime_boss::IDX_MOVE_SPLIT,
            _ => panic!(
                "Splittable on unexpected monster: {:?}",
                target.monster_name
            ),
        };
        if target.monster_move_current != Some(idx_split) {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::MoveUpdate {
                    move_override: Some(idx_split),
                },
                id_source: None,
                target: Target::Direct(Some(id_target)),
            });
        }
    }

    // Lagavulin: HP actually lost wakes him up
    if health_lost > 0 && has_modifier(&target.modifiers, ModifierKind::Asleep) {
        let idx_stunned = match target.monster_name {
            MonsterName::Lagavulin => lagavulin::IDX_MOVE_STUNNED,
            _ => panic!(
                "Unsupported monster name for Asleep modifier: {:?}",
                target.monster_name
            ),
        };
        // Executes in reverse:
        //     1. ModifierRemove Asleep
        //     2. MoveUpdate (stunned)
        state.effect_queue.push_front(Effect {
            kind: EffectKind::MoveUpdate {
                move_override: Some(idx_stunned),
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Asleep,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });

        // It comes out of its shell behind everything queued; its Metallicize stays until the removal the wake queues
        state.effect_queue.push_back(Effect {
            kind: EffectKind::LagavulinWake,
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }

    // Mode Shift (The Guardian): HP lost counts it down; the loss that spends it queues the switch behind everything queued, and the spent counter stays on, counting nothing more, until the removal the switch queues lands
    if has_modifier(&target.modifiers, ModifierKind::ModeShift) {
        let stacks = modifier_stacks(&target.modifiers, ModifierKind::ModeShift);
        if stacks > 0 {
            target.modifiers.stacks[ModifierKind::ModeShift as usize] = stacks - amount as i16;
            if stacks <= amount as i16 {
                state.effect_queue.push_back(Effect {
                    kind: EffectKind::DefensiveMode,
                    id_source: None,
                    target: Target::Direct(Some(id_target)),
                });
            }
        }
    }
}
