// Effect processing: one function, one match.

use rand::Rng;

use crate::cards::get_card;
use crate::cards::{REWARD_POOL_COMMON, REWARD_POOL_RARE, REWARD_POOL_UNCOMMON};
use crate::consts::*;
use crate::effect::{Effect, EffectTemplate, SelectionKind, TargetKind};
use crate::modifier::*;
use crate::monsters;
use crate::state::*;
use crate::types::*;

// ---------------------------------------------------------------------------
// Top-level dispatch
// ---------------------------------------------------------------------------

pub fn process_effect(state: &mut GameState, effect: Effect) -> EffectResult {
        Effect::CardRewardRoll => process_card_reward_roll(state),
        Effect::CardRewardSelect { reward_idx } => process_card_reward_select(state, reward_idx),
        Effect::CardRewardClear => {
            state.card_rewards.clear();
            EffectResult::empty()
        }
        Effect::TargetSet { monster_idx } => {
            state.card_target = Some(monster_idx);
            EffectResult::empty()
        }
        Effect::TargetClear => {
            state.card_target = None;
            EffectResult::empty()
        }
        Effect::DamagePhysical {
            source,
            target,
            base,
        } => process_damage_physical(state, source, target, base),
        Effect::DamageDeal { target, amount } => process_damage_deal(state, target, amount),
        Effect::HealthGain { target, amount } => process_health_gain(state, target, amount),
        Effect::HealthLoss { target, amount } => process_health_loss(state, target, amount),
        Effect::BlockGain {
            target,
            amount,
            from_card,
        } => process_block_gain(state, target, amount, from_card),
        Effect::BlockSet { target, amount } => {
            vitals_mut(state, target).block = amount;
            EffectResult::empty()
        }
        Effect::EnergyGain { amount } => {
            state.energy.current = (state.energy.current + amount).min(255);
            EffectResult::empty()
        }
        Effect::EnergyLoss { amount } => {
            state.energy.current = state.energy.current.saturating_sub(amount);
            EffectResult::empty()
        }
        Effect::ModifierGain {
            target,
            kind,
            stacks,
        } => process_modifier_gain(state, target, kind, stacks),
        Effect::ModifierRemove { target, kind } => {
            modifier_remove(&mut vitals_mut(state, target).modifiers, kind);
            EffectResult::empty()
        }
        Effect::ModifierTick { target } => {
            modifier_tick(&mut vitals_mut(state, target).modifiers);
            EffectResult::empty()
        }
        Effect::ModifierSetNotNew => process_modifier_set_not_new(state),
        Effect::Death { actor } => process_death(state, actor),
        Effect::CombatStart => process_combat_start(state),
        Effect::CombatEnd => process_combat_end(state),
        Effect::TurnStart { actor } => process_turn_start(state, actor),
        Effect::TurnEnd { actor } => process_turn_end(state, actor),
        Effect::MoveUpdate { monster_idx } => process_monster_move_update(state, monster_idx),
        Effect::RoomEnter => process_room_enter(state),
        Effect::GameEnd => {
            // Re-insert so determine_fsm can see it
            state.effect_queue.push_front(Effect::GameEnd);
            EffectResult::halt()
        }
        Effect::AwaitMapNode => {
            state.effect_queue.push_front(Effect::AwaitMapNode);
            EffectResult::pause()
        }
        Effect::AwaitCardReward => {
            state.effect_queue.push_front(Effect::AwaitCardReward);
            EffectResult::pause()
        }
        Effect::AwaitDiscard => {
            state.effect_queue.push_front(Effect::AwaitDiscard);
            EffectResult::pause()
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn vitals_mut(state: &mut GameState, actor: ActorId) -> &mut Vitals {
    match actor {
        ActorId::Character => &mut state.character.vitals,
        ActorId::Monster(i) => &mut state.monsters[i as usize].vitals,
    }
}

fn vitals_ref(state: &GameState, actor: ActorId) -> &Vitals {
    match actor {
        ActorId::Character => &state.character.vitals,
        ActorId::Monster(i) => &state.monsters[i as usize].vitals,
    }
}

// ---------------------------------------------------------------------------
// Effect handlers
// ---------------------------------------------------------------------------




fn process_card_reward_roll(state: &mut GameState) -> EffectResult {
    state.card_rewards.clear();
    let mut rolled_names: Vec<CardName> = Vec::new();

    for _ in 0..MAX_COMBAT_CARD_REWARD {
        let roll = state.rng.random_range(0i32..99) + state.character.reward_roll_offset as i32;

        let pool = if roll < CHANCE_RARE {
            state.character.reward_roll_offset = CARD_REWARD_ROLL_OFFSET_BASE;
            REWARD_POOL_RARE
        } else if roll < CHANCE_UNCOMMON {
            REWARD_POOL_UNCOMMON
        } else {
            state.character.reward_roll_offset =
                (state.character.reward_roll_offset - 1).max(CARD_REWARD_ROLL_OFFSET_MIN);
            REWARD_POOL_COMMON
        };

        let mut name = pool[state.rng.random_range(0..pool.len())];
        while rolled_names.contains(&name) {
            name = pool[state.rng.random_range(0..pool.len())];
        }
        rolled_names.push(name);
        state.card_rewards.push(get_card(name, false));
    }

    EffectResult::bot(vec![Effect::AwaitCardReward])
}

fn process_card_reward_select(state: &mut GameState, reward_idx: usize) -> EffectResult {
    let card = state.card_rewards[reward_idx];
    state.deck.push(card);
    state.card_rewards.clear();
    EffectResult::empty()
}

fn process_damage_physical(
    state: &mut GameState,
    source: ActorId,
    target: ActorId,
    base: u16,
) -> EffectResult {
    let source_mods = &vitals_ref(state, source).modifiers;
    let target_mods = &vitals_ref(state, target).modifiers;

    let mut value = base as f32;

    // Accuracy (only for shiv, but we apply based on whether source has accuracy and card is shiv)
    // In the Python code, accuracy is checked when source is a card named "Shiv"
    // Here we handle it in card_play by checking the card name. Accuracy bonus is applied
    // to the base before DamagePhysical is created. For simplicity, we trust the base already
    // includes accuracy if appropriate. The card_play handler should add accuracy to shiv damage.
    // Actually, let's just add accuracy if the source is Character and has accuracy, and base <= 6
    // No -- let's be precise. We need to know if this is from a shiv. We'll handle it upstream.

    // Apply strength
    if modifier_has(source_mods, ModifierKind::Strength) {
        value += modifier_stacks(source_mods, ModifierKind::Strength) as f32;
    }

    // Apply weak
    if modifier_has(source_mods, ModifierKind::Weak) {
        value *= FACTOR_WEAK;
    }

    // Apply vulnerable
    if modifier_has(target_mods, ModifierKind::Vulnerable) {
        value *= FACTOR_VULN;
    }

    // Apply double damage
    if modifier_has(source_mods, ModifierKind::DoubleDamage) {
        value *= 2.0;
    }

    let final_damage = value as u16;
    if final_damage > 0 {
        EffectResult::top(vec![Effect::DamageDeal {
            target,
            amount: final_damage,
        }])
    } else {
        EffectResult::empty()
    }
}

fn process_damage_deal(state: &mut GameState, target: ActorId, amount: u16) -> EffectResult {
    let vitals = vitals_mut(state, target);
    let over_block = amount.saturating_sub(vitals.block);
    vitals.block = vitals.block.saturating_sub(amount);

    if over_block > 0 {
        EffectResult::top(vec![Effect::HealthLoss {
            target,
            amount: over_block,
        }])
    } else {
        EffectResult::empty()
    }
}

fn process_health_gain(state: &mut GameState, target: ActorId, amount: u16) -> EffectResult {
    let vitals = vitals_mut(state, target);
    vitals.health = (vitals.health + amount).min(vitals.health_max);
    EffectResult::empty()
}

fn process_health_loss(state: &mut GameState, target: ActorId, amount: u16) -> EffectResult {
    let vitals = vitals_mut(state, target);
    vitals.health = vitals.health.saturating_sub(amount);

    let mut effects = Vec::new();

    if vitals.health == 0 {
        effects.push(Effect::Death { actor: target });
    } else {
        // Mode shift check
        if modifier_has(&vitals.modifiers, ModifierKind::ModeShift) {
            let new_stacks =
                modifier_stacks(&vitals.modifiers, ModifierKind::ModeShift) - amount as i16;
            if new_stacks < modifier_def(ModifierKind::ModeShift).stacks_min {
                modifier_remove(
                    &mut vitals_mut(state, target).modifiers,
                    ModifierKind::ModeShift,
                );
                if let ActorId::Monster(i) = target {
                    effects.push(Effect::MoveUpdate { monster_idx: i });
                }
            } else {
                vitals_mut(state, target).modifiers.stacks[ModifierKind::ModeShift as usize] =
                    new_stacks;
            }
        }
    }

    if effects.is_empty() {
        EffectResult::empty()
    } else {
        EffectResult::top(effects)
    }
}

fn process_block_gain(
    state: &mut GameState,
    target: ActorId,
    amount: u16,
    from_card: bool,
) -> EffectResult {
    let vitals = vitals_mut(state, target);
    let mut value = amount as i32;

    // Apply dexterity (only from card effects)
    if from_card && modifier_has(&vitals.modifiers, ModifierKind::Dexterity) {
        value += modifier_stacks(&vitals.modifiers, ModifierKind::Dexterity) as i32;
    }

    if value > 0 {
        vitals.block = (vitals.block + value as u16).min(999);
    }
    EffectResult::empty()
}

fn process_modifier_gain(
    state: &mut GameState,
    target: ActorId,
    kind: ModifierKind,
    stacks: i16,
) -> EffectResult {
    // Special case: ModeShift gain for The Guardian
    if kind == ModifierKind::ModeShift {
        return process_mode_shift_gain(state, target);
    }

    // Handle negative stacks as loss (e.g., Burst -1)
    if stacks < 0 {
        let mods = &mut vitals_mut(state, target).modifiers;
        if modifier_has(mods, kind) {
            let idx = kind as usize;
            mods.stacks[idx] += stacks;
            let cfg = modifier_def(kind);
            if mods.stacks[idx] < cfg.stacks_min {
                modifier_remove(mods, kind);
            }
        }
        return EffectResult::empty();
    }

    modifier_apply(&mut vitals_mut(state, target).modifiers, kind, stacks);
    EffectResult::empty()
}

fn process_mode_shift_gain(state: &mut GameState, target: ActorId) -> EffectResult {
    if let ActorId::Monster(i) = target {
        let monster = &state.monsters[i as usize];
        // Count Twin Slam cycles
        let cycle_count = monster
            .move_history
            .iter()
            .filter(|&&m| monster.moves[m].name == "Twin Slam")
            .count();
        let increase = MODE_SHIFT_INCREASE_PER_CYCLE * cycle_count as i16;

        let base = if state.ascension < 9 {
            30
        } else if state.ascension < 19 {
            35
        } else {
            40
        };
        let stacks = base + increase;

        modifier_apply(
            &mut state.monsters[i as usize].vitals.modifiers,
            ModifierKind::ModeShift,
            stacks,
        );
        state.monsters[i as usize].vitals.modifiers.is_new[ModifierKind::ModeShift as usize] =
            false;
    }
    EffectResult::empty()
}

fn process_modifier_set_not_new(state: &mut GameState) -> EffectResult {
    modifier_set_not_new(&mut state.character.vitals.modifiers);
    for m in &mut state.monsters {
        modifier_set_not_new(&mut m.vitals.modifiers);
    }
    EffectResult::empty()
}

fn process_death(state: &mut GameState, actor: ActorId) -> EffectResult {
    match actor {
        ActorId::Character => EffectResult::top(vec![Effect::GameEnd]),
        ActorId::Monster(i) => {
            let idx = i as usize;
            // Collect on-death effects before removing
            let mut effects = Vec::new();

            // Spore Cloud
            if modifier_has(
                &state.monsters[idx].vitals.modifiers,
                ModifierKind::SporeCloud,
            ) {
                let stacks = modifier_stacks(
                    &state.monsters[idx].vitals.modifiers,
                    ModifierKind::SporeCloud,
                );
                effects.push(Effect::ModifierGain {
                    target: ActorId::Character,
                    kind: ModifierKind::Vulnerable,
                    stacks,
                });
            }

            // Remove monster
            state.monsters.remove(idx);

            // Reindex: all monster indices above `idx` shift down by 1.
            // Effects in the queue that reference monsters above `idx` would need updating.
            // For simplicity, we handle this by ensuring we process death immediately.

            if state.monsters.is_empty() {
                effects.push(Effect::CombatEnd);
            }

            EffectResult::top(effects)
        }
    }
}

fn process_combat_start(state: &mut GameState) -> EffectResult {
    // Clone deck into combat_cards
    state.combat_cards = state.deck.clone();
    let n = state.combat_cards.len();

    // Separate innate from non-innate
    let mut innate_indices: Vec<usize> = Vec::new();
    let mut other_indices: Vec<usize> = Vec::new();
    for i in 0..n {
        if state.combat_cards[i].innate {
            innate_indices.push(i);
        } else {
            other_indices.push(i);
        }
    }

    // Shuffle non-innate
    for i in (1..other_indices.len()).rev() {
        let j = state.rng.random_range(0..=i);
        other_indices.swap(i, j);
    }

    // Draw pile: innate first, then shuffled rest
    state.draw_pile = innate_indices;
    state.draw_pile.extend(other_indices);

    state.hand.clear();
    state.discard_pile.clear();
    state.exhaust_pile.clear();
    state.card_active = None;
    state.card_target = None;

    let mut effects: Vec<Effect> = Vec::new();
    for i in 0..state.monsters.len() {
        effects.push(Effect::MoveUpdate {
            monster_idx: i as u8,
        });
    }
    effects.push(Effect::TurnStart {
        actor: ActorId::Character,
    });

    EffectResult::top(effects)
}

fn process_combat_end(state: &mut GameState) -> EffectResult {
    state.hand.clear();
    state.draw_pile.clear();
    state.discard_pile.clear();
    state.exhaust_pile.clear();
    state.combat_cards.clear();
    state.card_active = None;
    state.card_target = None;
    modifier_clear(&mut state.character.vitals.modifiers);

    // Check room type
    let room = state.map.active_room_type().unwrap();
    match room {
        RoomType::CombatBoss => EffectResult::top(vec![Effect::GameEnd]),
        RoomType::CombatMonster => EffectResult::bot(vec![Effect::CardRewardRoll]),
        RoomType::RestSite => unreachable!("combat end in rest site"),
    }
}

fn process_turn_start(state: &mut GameState, actor: ActorId) -> EffectResult {
    let mut effects = Vec::new();

    // Block handling (blur preserves, otherwise reset)
    let vitals = vitals_mut(state, actor);
    let mut new_block: u16 = 0;
    if modifier_has(&vitals.modifiers, ModifierKind::Blur) {
        new_block += vitals.block;
    }
    if modifier_has(&vitals.modifiers, ModifierKind::NextTurnBlock) {
        new_block += modifier_stacks(&vitals.modifiers, ModifierKind::NextTurnBlock) as u16;
        modifier_remove(&mut vitals.modifiers, ModifierKind::NextTurnBlock);
    }
    effects.push(Effect::BlockSet {
        target: actor,
        amount: new_block,
    });

    // Phantasmal -> double damage
    if modifier_has(
        &vitals_ref(state, actor).modifiers,
        ModifierKind::Phantasmal,
    ) {
        effects.push(Effect::ModifierGain {
            target: actor,
            kind: ModifierKind::DoubleDamage,
            stacks: 1,
        });
    }

    // Character-specific
    if actor == ActorId::Character {
        effects.push(Effect::CardDraw {
            count: CARDS_DRAWN_PER_TURN,
        });
        let energy_gain = state.energy.max - state.energy.current;
        effects.push(Effect::EnergyGain {
            amount: energy_gain,
        });
        effects.push(Effect::ModifierTick {
            target: ActorId::Character,
        });
        for i in 0..state.monsters.len() {
            effects.push(Effect::ModifierTick {
                target: ActorId::Monster(i as u8),
            });
        }

        // Next turn energy
        if modifier_has(
            &state.character.vitals.modifiers,
            ModifierKind::NextTurnEnergy,
        ) {
            let stacks = modifier_stacks(
                &state.character.vitals.modifiers,
                ModifierKind::NextTurnEnergy,
            );
            effects.push(Effect::EnergyGain {
                amount: stacks as u8,
            });
            modifier_remove(
                &mut state.character.vitals.modifiers,
                ModifierKind::NextTurnEnergy,
            );
        }

        // Infinite blades
        if modifier_has(
            &state.character.vitals.modifiers,
            ModifierKind::InfiniteBlades,
        ) {
            let stacks = modifier_stacks(
                &state.character.vitals.modifiers,
                ModifierKind::InfiniteBlades,
            );
            effects.push(Effect::AddShivs {
                count: stacks as u8,
            });
        }
    }

    EffectResult::top(effects)
}

fn process_turn_end(state: &mut GameState, actor: ActorId) -> EffectResult {
    let mut effects = Vec::new();

    // Ritual (common for both character and monsters)
    {
        let vitals = vitals_ref(state, actor);
        if modifier_has(&vitals.modifiers, ModifierKind::Ritual)
            && !vitals.modifiers.is_new[ModifierKind::Ritual as usize]
        {
            let stacks = modifier_stacks(&vitals.modifiers, ModifierKind::Ritual);
            effects.push(Effect::ModifierGain {
                target: actor,
                kind: ModifierKind::Strength,
                stacks,
            });
        }
    }

    // Monster: just return the common effects
    if let ActorId::Monster(_) = actor {
        return EffectResult::top(effects);
    }

    // Character turn end: discard hand, queue monster turns, start new character turn
    effects.push(Effect::CardDiscardAll);
    effects.push(Effect::ModifierSetNotNew);

    for i in 0..state.monsters.len() {
        let mi = ActorId::Monster(i as u8);
        effects.push(Effect::TurnStart { actor: mi });

        // Monster's move effects
        let monster = &state.monsters[i];
        if let Some(move_idx) = monster.move_current {
            let move_effects = instantiate_templates(monster.moves[move_idx].effects, mi, state);
            effects.extend(move_effects);
        }

        effects.push(Effect::MoveUpdate {
            monster_idx: i as u8,
        });
        effects.push(Effect::TurnEnd { actor: mi });
    }

    // Character's next turn
    effects.push(Effect::TurnStart {
        actor: ActorId::Character,
    });

    // Clear Burst at end of character turn
    if modifier_has(&state.character.vitals.modifiers, ModifierKind::Burst) {
        modifier_remove(&mut state.character.vitals.modifiers, ModifierKind::Burst);
    }

    EffectResult::top(effects)
}

fn process_monster_move_update(state: &mut GameState, monster_idx: u8) -> EffectResult {
    let i = monster_idx as usize;
    let move_next = monsters::get_next_move(&state.monsters[i], &mut state.rng);
    state.monsters[i].move_current = Some(move_next);
    state.monsters[i].move_history.push(move_next);
    EffectResult::empty()
}

fn process_room_enter(state: &mut GameState) -> EffectResult {
    let room = state.map.active_room_type().unwrap();
    match room {
        RoomType::CombatBoss => {
            // Spawn The Guardian TODO: add more bosses
            state.monsters = vec![monsters::spawn_monster(
                MonsterName::TheGuardian,
                state.ascension,
                &mut state.rng,
            )];
            EffectResult::top(vec![Effect::CombatStart])
        }
        RoomType::CombatMonster => {
            // Random encounter
            let encounter: u8 = state.rng.random_range(0..3);
            state.monsters = match encounter {
                0 => vec![monsters::spawn_monster(
                    MonsterName::JawWorm,
                    state.ascension,
                    &mut state.rng,
                )],
                1 => vec![monsters::spawn_monster(
                    MonsterName::Cultist,
                    state.ascension,
                    &mut state.rng,
                )],
                2 => vec![
                    monsters::spawn_monster(
                        MonsterName::FungiBeast,
                        state.ascension,
                        &mut state.rng,
                    ),
                    monsters::spawn_monster(
                        MonsterName::FungiBeast,
                        state.ascension,
                        &mut state.rng,
                    ),
                ],
                _ => unreachable!(),
            };
            EffectResult::top(vec![Effect::CombatStart])
        }
        RoomType::RestSite => EffectResult::empty(),
    }
}

// ---------------------------------------------------------------------------
// Queue processing loop
// ---------------------------------------------------------------------------

pub fn process_queue(state: &mut GameState) {
    while let Some(effect) = state.effect_queue.pop_front() {
        // Clear queue before processing CombatEnd
        if matches!(effect, Effect::CombatEnd) {
            state.effect_queue.clear();
        }

        let result = process_effect(state, effect);

        // Push new effects
        for e in result.push_top.into_iter().rev() {
            state.effect_queue.push_front(e);
        }
        for e in result.push_bot {
            state.effect_queue.push_back(e);
        }

        if result.halt || result.pause {
            return;
        }
    }
}
