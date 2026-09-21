use strum::EnumCount;
use strum::EnumIter;

use crate::consts::GIRYA_LIFT_MAX;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::EFFECT_RELIC_GRANT_RANDOM;
use crate::effect::EFFECT_REST_HEAL;
use crate::effect::EFFECT_TARGET_SET;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::effect::effect_gold_gain;
use crate::effect::effect_gold_loss;
use crate::effect::effect_input_one;
use crate::effect::effect_untargeted;
use crate::engine::selection_candidates;
use crate::game::GameState;
use crate::potions::belt_has_room;
use crate::relics::iter_owned_relics;
use crate::types::Focus;
use crate::types::RelicName;
use crate::types::RewardKind;
use crate::types::ShopSlot;
use crate::utils::context_focus;
use crate::utils::entity_requires_target;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;

// A legal action: its kind, the answers to its chain's picks in order, and that chain
#[derive(Debug, Clone)]
pub struct Action {
    pub kind: ActionKind,
    pub id_input: Vec<usize>,
    pub effects: Vec<Effect>,
}

// Identity is what the engine runs; the effects describe it, they do not name it
impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.id_input == other.id_input
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount, EnumIter)]
pub enum ActionKind {
    CardPlay,
    ChestOpen,
    EventOptionSelect,
    PotionDiscard,
    PotionUse,
    Rest,
    RewardTakeCard,
    RewardTakeGold,
    RewardTakePotion,
    RewardTakeRelic,
    RoomExit,
    RoomSelect,
    ShopBuyCard,
    ShopBuyPotion,
    ShopBuyRelic,
    ShopPurge,
    TurnEnd,
    SelectionSkip,
    RestDig,
    RestLift,
    RestToke,
    RestSmith,
    RewardSingingBowl,
    EffectPendingResolve,
}

// The effects an Action of this kind enqueues in this state, in order
pub fn effects_for_action_kind(kind: ActionKind, state: &GameState) -> Vec<Effect> {
    let consume = effect_untargeted(EffectKind::RestSiteConsume);
    match kind {
        ActionKind::CardPlay => vec![effect_input_one(
            EffectKind::CardPlay,
            CandidatePool::Hand,
            CandidateFilter::Playable,
        )],
        ActionKind::PotionUse => vec![effect_input_one(
            EffectKind::PotionUse,
            CandidatePool::PotionsOwned,
            CandidateFilter::Usable,
        )],
        ActionKind::PotionDiscard => vec![effect_input_one(
            EffectKind::PotionDiscard,
            CandidatePool::PotionsOwned,
            CandidateFilter::Any,
        )],
        ActionKind::TurnEnd => vec![Effect {
            kind: EffectKind::TurnEnd,
            id_source: None,
            target: TARGET_CHARACTER,
        }],
        ActionKind::EventOptionSelect => vec![effect_input_one(
            EffectKind::EventOptionSelect,
            CandidatePool::EventOptions,
            CandidateFilter::EventOptionAvailable,
        )],
        ActionKind::RewardTakeCard => vec![effect_input_one(
            EffectKind::RewardTake {
                kind: RewardKind::Card,
            },
            CandidatePool::RewardCards,
            CandidateFilter::Any,
        )],
        ActionKind::RewardTakeRelic => vec![effect_input_one(
            EffectKind::RewardTake {
                kind: RewardKind::Relic,
            },
            CandidatePool::RewardRelics,
            CandidateFilter::Any,
        )],
        ActionKind::RewardTakePotion => vec![effect_input_one(
            EffectKind::RewardTake {
                kind: RewardKind::Potion,
            },
            CandidatePool::RewardPotions,
            CandidateFilter::Any,
        )],
        ActionKind::RewardTakeGold => vec![
            effect_gold_gain(state.reward.gold.expect("gold is offered")),
            effect_untargeted(EffectKind::RewardTake {
                kind: RewardKind::Gold,
            }),
        ],
        ActionKind::RewardSingingBowl => vec![effect_input_one(
            EffectKind::SingingBowlProc,
            CandidatePool::RewardCards,
            CandidateFilter::Any,
        )],
        ActionKind::RoomSelect => vec![effect_input_one(
            EffectKind::RoomSelect,
            CandidatePool::NextRooms,
            CandidateFilter::Reachable,
        )],
        ActionKind::RoomExit => vec![effect_untargeted(EffectKind::RoomExit)],
        ActionKind::ShopBuyCard => vec![effect_input_one(
            EffectKind::ShopBuy {
                slot: ShopSlot::Card,
            },
            CandidatePool::CardShop,
            CandidateFilter::Affordable,
        )],
        ActionKind::ShopBuyRelic => vec![effect_input_one(
            EffectKind::ShopBuy {
                slot: ShopSlot::Relic,
            },
            CandidatePool::RelicShop,
            CandidateFilter::Affordable,
        )],
        ActionKind::ShopBuyPotion => vec![effect_input_one(
            EffectKind::ShopBuy {
                slot: ShopSlot::Potion,
            },
            CandidatePool::PotionShop,
            CandidateFilter::Affordable,
        )],
        // Pay, purge the named Card, then the driver's bookkeeping
        ActionKind::ShopPurge => vec![
            effect_gold_loss(state.shop.purge_cost),
            effect_input_one(
                EffectKind::CardPurge,
                CandidatePool::Deck,
                CandidateFilter::Purgeable,
            ),
            effect_untargeted(EffectKind::ShopPurge),
        ],
        ActionKind::ChestOpen => vec![effect_untargeted(EffectKind::ChestOpen)],
        ActionKind::Rest => {
            let mut effects = vec![EFFECT_REST_HEAL, consume];

            // Rest Relic effects, in acquisition order, after the consume
            let mut id_relics: Vec<usize> = iter_owned_relics(&state.id_relics)
                .map(|(_, id)| id)
                .collect();
            id_relics.sort_unstable_by_key(|&id| state.entities[id].relic_seq);
            for id_relic in id_relics {
                effects.extend_from_slice(state.entities[id_relic].relic_effects_on_rest);
            }
            effects
        }
        ActionKind::RestSmith => vec![
            effect_input_one(
                EffectKind::CardUpgrade,
                CandidatePool::Deck,
                CandidateFilter::Upgradeable,
            ),
            consume,
        ],
        ActionKind::RestToke => vec![
            effect_input_one(
                EffectKind::CardPurge,
                CandidatePool::Deck,
                CandidateFilter::Purgeable,
            ),
            consume,
        ],
        ActionKind::RestDig => vec![EFFECT_RELIC_GRANT_RANDOM, consume],
        ActionKind::RestLift => vec![effect_untargeted(EffectKind::GiryaLift), consume],
        // The halt branch builds these from the parked effect
        ActionKind::SelectionSkip | ActionKind::EffectPendingResolve => Vec::new(),
    }
}

// Runs the idx-th legal action: its answers join the input, its chain the queue front
pub fn handle_action(state: &mut GameState, idx: usize) -> Result<(), String> {
    if state.game_over {
        return Err("GameOver".into());
    }
    let Some(action) = state.legal_actions.get(idx).cloned() else {
        return Err(format!(
            "Illegal action {idx} in current state ({} legal)",
            state.legal_actions.len()
        ));
    };

    // A halt answer's chain is the parked effect itself, so nothing here is kind-specific
    state.effect_pending = None;
    state.id_input.extend_from_slice(&action.id_input);
    state.effect_buf.clear();
    state.effect_buf.extend_from_slice(&action.effects);
    flush_effects_from_buf_to_queue_front(state);
    Ok(())
}

// Every legal action in this state, each carrying the effects choosing it enqueues
pub fn recompute_legal_actions(state: &mut GameState) {
    state.legal_actions.clear();
    if state.game_over {
        return;
    }

    // A halt: each candidate not yet input answers it, re-queuing the parked effect
    if let Some(effect_pending) = state.effect_pending {
        let Target::Resolve {
            candidate_pool,
            filter,
            selection_kind,
        } = effect_pending.target
        else {
            unreachable!("effect_pending carries a Resolve target")
        };
        for id in selection_candidates(state, candidate_pool, filter, effect_pending.id_source) {
            if !state.id_input.contains(&id) {
                push_action(
                    state,
                    ActionKind::EffectPendingResolve,
                    vec![id],
                    vec![effect_pending],
                );
            }
        }

        // An InputUpTo pick may close at what has been input
        if matches!(selection_kind, SelectionKind::InputUpTo { .. }) {
            let closed = Effect {
                target: Target::Resolve {
                    candidate_pool,
                    filter,
                    selection_kind: SelectionKind::Input {
                        count: state.id_input.len() as u16,
                    },
                },
                ..effect_pending
            };
            push_action(state, ActionKind::SelectionSkip, Vec::new(), vec![closed]);
        }
        return;
    }

    match context_focus(state) {
        Focus::Combat => legal_actions_combat(state),
        Focus::Reward => legal_actions_reward(state),
        Focus::Event => legal_actions_event(state),
        Focus::Shop => legal_actions_shop(state),
        Focus::Map => legal_actions_map(state),
        Focus::RestSite => legal_actions_rest_site(state),
        Focus::Chest => legal_actions_chest(state),
    }

    // The belt is always usable as long as there's no pending effect
    legal_actions_potions(state);
}

// One legal action
fn push_action(
    state: &mut GameState,
    kind: ActionKind,
    id_input: Vec<usize>,
    effects: Vec<Effect>,
) {
    state.legal_actions.push(Action {
        kind,
        id_input,
        effects,
    });
}

// The pool and filter of a chain's first Input pick
fn input_source(effects: &[Effect]) -> Option<(CandidatePool, CandidateFilter)> {
    effects.iter().find_map(|effect| match effect.target {
        Target::Resolve {
            candidate_pool,
            filter,
            selection_kind: SelectionKind::Input { .. },
        } => Some((candidate_pool, filter)),
        _ => None,
    })
}

// Every legal action of a kind: one per entity its pick may take, or one when it picks nothing
fn push_actions_for_kind(state: &mut GameState, kind: ActionKind) {
    let effects = effects_for_action_kind(kind, state);
    match input_source(&effects) {
        None => push_action(state, kind, Vec::new(), effects),
        Some((candidate_pool, filter)) => {
            for id in selection_candidates(state, candidate_pool, filter, None) {
                push_action(state, kind, vec![id], effects.clone());
            }
        }
    }
}

// A play: one action per Monster when the played entity wants one, its TargetSet heading the
// chain and its Monster heading the answers
fn push_plays_for_kind(state: &mut GameState, kind: ActionKind) {
    let effects = effects_for_action_kind(kind, state);
    let (candidate_pool, filter) = input_source(&effects).expect("a play picks its entity");
    for id in selection_candidates(state, candidate_pool, filter, None) {
        if !entity_requires_target(&state.entities[id]) {
            push_action(state, kind, vec![id], effects.clone());
            continue;
        }
        let id_monsters: Vec<usize> = state.combat.id_monsters.iter().flatten().copied().collect();
        for id_monster in id_monsters {
            let effects_targeted = [&[EFFECT_TARGET_SET], effects.as_slice()].concat();
            push_action(state, kind, vec![id_monster, id], effects_targeted);
        }
    }
}

fn legal_actions_combat(state: &mut GameState) {
    // Playable Cards
    push_plays_for_kind(state, ActionKind::CardPlay);

    // Turn end
    push_actions_for_kind(state, ActionKind::TurnEnd);
}

fn legal_actions_reward(state: &mut GameState) {
    // Cards
    push_actions_for_kind(state, ActionKind::RewardTakeCard);

    // Singing Bowl: forfeit the picked Card's whole bundle for +2 max HP
    if has_relic(&state.id_relics, RelicName::SingingBowl) {
        push_actions_for_kind(state, ActionKind::RewardSingingBowl);
    }

    // Relics
    push_actions_for_kind(state, ActionKind::RewardTakeRelic);

    // Potions
    if can_take_potions(state) {
        push_actions_for_kind(state, ActionKind::RewardTakePotion);
    }

    // Gold
    if state.reward.gold.is_some() {
        push_actions_for_kind(state, ActionKind::RewardTakeGold);
    }

    // Exit
    push_actions_for_kind(state, ActionKind::RoomExit);
}

fn legal_actions_event(state: &mut GameState) {
    // TODO: investigate pushing `EffectKind::RoomExit` at once
    if state.event.consumed {
        push_actions_for_kind(state, ActionKind::RoomExit);
        return;
    }
    push_actions_for_kind(state, ActionKind::EventOptionSelect);
}

fn legal_actions_shop(state: &mut GameState) {
    // Shop is always exitable
    push_actions_for_kind(state, ActionKind::RoomExit);

    // Cards
    push_actions_for_kind(state, ActionKind::ShopBuyCard);

    // Relics
    push_actions_for_kind(state, ActionKind::ShopBuyRelic);

    // Potions
    if can_take_potions(state) {
        push_actions_for_kind(state, ActionKind::ShopBuyPotion);
    }

    // Purge
    // TODO: make `shop_price` a field of `Shop` again
    let char_gold = state.entities[state.id_character].character_gold;
    if !state.shop.purged && char_gold >= state.shop.purge_cost {
        push_actions_for_kind(state, ActionKind::ShopPurge);
    }
}

fn legal_actions_map(state: &mut GameState) {
    push_actions_for_kind(state, ActionKind::RoomSelect);
}

fn legal_actions_rest_site(state: &mut GameState) {
    let consumed = state.rest_site.consumed;
    let id_relics = state.id_relics;

    // Coffee Dripper: restless
    if !consumed && !has_relic(&id_relics, RelicName::CoffeeDripper) {
        push_actions_for_kind(state, ActionKind::Rest);
    }

    // Fusion Hammer: cannot upgrade
    if !consumed && !has_relic(&id_relics, RelicName::FusionHammer) {
        push_actions_for_kind(state, ActionKind::RestSmith);
    }

    // Girya: can lift to gain Strength
    if !consumed
        && id_relics[RelicName::Girya as usize]
            .is_some_and(|id| state.entities[id].relic_counter < GIRYA_LIFT_MAX)
    {
        push_actions_for_kind(state, ActionKind::RestLift);
    }

    // Peace Pipe: can toke to purge a Card
    if !consumed && has_relic(&id_relics, RelicName::PeacePipe) {
        push_actions_for_kind(state, ActionKind::RestToke);
    }

    // Shovel: can dig to find a random Relic
    if !consumed && has_relic(&id_relics, RelicName::Shovel) {
        push_actions_for_kind(state, ActionKind::RestDig);
    }

    // Leave once the site is used, or when no option is left (no soft-lock)
    if state.legal_actions.is_empty() {
        push_actions_for_kind(state, ActionKind::RoomExit);
    }
}

fn legal_actions_chest(state: &mut GameState) {
    if !state.chest.chest_opened {
        push_actions_for_kind(state, ActionKind::ChestOpen);
    }
    push_actions_for_kind(state, ActionKind::RoomExit);
}

fn legal_actions_potions(state: &mut GameState) {
    // Potion use
    push_plays_for_kind(state, ActionKind::PotionUse);

    // Potion discard
    push_actions_for_kind(state, ActionKind::PotionDiscard);
}

fn can_take_potions(state: &GameState) -> bool {
    belt_has_room(&state.id_potions, state.potion_slots_max)
        // Sozu: Potions can't be obtained, so neither taken nor bought
        && !has_relic(&state.id_relics, RelicName::Sozu)
}
