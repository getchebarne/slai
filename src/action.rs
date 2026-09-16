use strum::EnumCount;
use strum::EnumIter;

use crate::consts::GIRYA_LIFT_MAX;
use crate::consts::MAP_HEIGHT;
use crate::effect::EFFECT_RELIC_GRANT_RANDOM;
use crate::effect::EFFECT_REST_HEAL;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_SELECTED;
use crate::effect::Target;
use crate::effect::effect_gold_gain;
use crate::effect::effect_gold_loss;
use crate::effect::effect_untargeted;
use crate::engine::selection_candidates;
use crate::entity::Entity;
use crate::events::event_option_available;
use crate::game::GameState;
use crate::game::Location;
use crate::potions::belt_has_room;
use crate::relics::iter_owned_relics;
use crate::types::Focus;
use crate::types::RelicName;
use crate::types::RewardKind;
use crate::types::ShopSlot;
use crate::utils::card_is_playable;
use crate::utils::card_is_purgeable;
use crate::utils::card_is_upgradable;
use crate::utils::context_focus;
use crate::utils::entity_requires_target;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;
use crate::utils::potion_is_usable;
use crate::utils::room_is_reachable;

// A legal action: its kind, the entities it names, and the effects choosing it enqueues
#[derive(Debug, Clone)]
pub struct Action {
    pub kind: ActionKind,
    pub id_selected: Option<usize>,
    pub id_monster_target: Option<usize>,
    pub effects: Vec<Effect>,
}

// Identity is what the engine runs; the effects describe it, they do not name it
impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.id_selected == other.id_selected
            && self.id_monster_target == other.id_monster_target
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

// An effect acting on the entity the Action named
const fn effect_on_selected(kind: EffectKind) -> Effect {
    Effect {
        kind,
        id_source: None,
        target: TARGET_SELECTED,
    }
}

// The effects an Action of this kind enqueues in this state, in order
pub fn effects_for_action_kind(kind: ActionKind, state: &GameState) -> Vec<Effect> {
    let consume = effect_untargeted(EffectKind::RestSiteConsume);
    match kind {
        ActionKind::CardPlay => vec![effect_on_selected(EffectKind::CardPlay)],
        ActionKind::PotionUse => vec![effect_on_selected(EffectKind::PotionUse)],
        ActionKind::PotionDiscard => vec![effect_on_selected(EffectKind::PotionDiscard)],
        ActionKind::TurnEnd => vec![Effect {
            kind: EffectKind::TurnEnd,
            id_source: None,
            target: TARGET_CHARACTER,
        }],
        ActionKind::EventOptionSelect => vec![effect_on_selected(EffectKind::EventOptionSelect)],
        ActionKind::RewardTakeCard => vec![effect_on_selected(EffectKind::RewardTake {
            kind: RewardKind::Card,
        })],
        ActionKind::RewardTakeRelic => vec![effect_on_selected(EffectKind::RewardTake {
            kind: RewardKind::Relic,
        })],
        ActionKind::RewardTakePotion => vec![effect_on_selected(EffectKind::RewardTake {
            kind: RewardKind::Potion,
        })],
        ActionKind::RewardTakeGold => vec![
            effect_gold_gain(state.reward.gold.expect("gold is offered")),
            effect_untargeted(EffectKind::RewardTake {
                kind: RewardKind::Gold,
            }),
        ],
        ActionKind::RewardSingingBowl => vec![effect_on_selected(EffectKind::SingingBowlProc)],
        ActionKind::RoomSelect => vec![effect_on_selected(EffectKind::RoomSelect)],
        ActionKind::RoomExit => vec![effect_untargeted(EffectKind::RoomExit)],
        ActionKind::ShopBuyCard => vec![effect_on_selected(EffectKind::ShopBuy {
            slot: ShopSlot::Card,
        })],
        ActionKind::ShopBuyRelic => vec![effect_on_selected(EffectKind::ShopBuy {
            slot: ShopSlot::Relic,
        })],
        ActionKind::ShopBuyPotion => vec![effect_on_selected(EffectKind::ShopBuy {
            slot: ShopSlot::Potion,
        })],
        // Pay, purge the named Card, then the driver's bookkeeping
        ActionKind::ShopPurge => vec![
            effect_gold_loss(state.shop.purge_cost),
            effect_on_selected(EffectKind::CardPurge),
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
        ActionKind::RestSmith => vec![effect_on_selected(EffectKind::CardUpgrade), consume],
        ActionKind::RestToke => vec![effect_on_selected(EffectKind::CardPurge), consume],
        ActionKind::RestDig => vec![EFFECT_RELIC_GRANT_RANDOM, consume],
        ActionKind::RestLift => vec![effect_untargeted(EffectKind::GiryaLift), consume],
        ActionKind::SelectionSkip | ActionKind::EffectPendingResolve => Vec::new(),
    }
}

// Runs the idx-th legal action: its ids go to `id_input`, its chain to the queue
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

    // Handlers push their effects into effect_buf; flush drains them to the queue front (reversed)
    state.effect_buf.clear();
    match action.kind {
        ActionKind::EffectPendingResolve => handle_effect_pending_resolve(
            state,
            action.id_selected.expect("halt answer carries its id"),
        ),
        ActionKind::SelectionSkip => handle_selection_skip(state),
        // The Action names its entities; its chain resolves against them
        _ => {
            state.id_selected = action.id_selected;
            state.combat.id_monster_target = action.id_monster_target;
            state.effect_buf.extend_from_slice(&action.effects);
        }
    }
    flush_effects_from_buf_to_queue_front(state);
    Ok(())
}

// Every legal action in this state, each carrying the effects choosing it enqueues
pub fn recompute_legal_actions(state: &mut GameState) {
    state.legal_actions.clear();
    if state.game_over {
        return;
    }

    // A halt: the pick's candidates not yet input answer it, nothing else is legal
    if let Some(effect_pending) = state.effect_pending {
        let Target::Resolve {
            candidate_pool,
            filter,
            selection_kind,
        } = effect_pending.target
        else {
            unreachable!("effect_pending carries a Resolve target")
        };
        // TODO: should be set by the resolver
        for id in selection_candidates(state, candidate_pool, filter, effect_pending.id_source) {
            if !state.id_input.contains(&id) {
                state.legal_actions.push(Action {
                    kind: ActionKind::EffectPendingResolve,
                    id_selected: Some(id),
                    id_monster_target: None,
                    effects: Vec::new(),
                });
            }
        }

        // `SelectionKind::InputUpTo` is skippable
        if matches!(selection_kind, SelectionKind::InputUpTo { .. }) {
            state.legal_actions.push(Action {
                kind: ActionKind::SelectionSkip,
                id_selected: None,
                id_monster_target: None,
                effects: Vec::new(),
            });
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

// One legal action, carrying the chain its kind enqueues
fn push_action(
    state: &mut GameState,
    kind: ActionKind,
    id_selected: Option<usize>,
    id_monster_target: Option<usize>,
) {
    let effects = effects_for_action_kind(kind, state);
    state.legal_actions.push(Action {
        kind,
        id_selected,
        id_monster_target,
        effects,
    });
}

// One legal action per entity the kind names
fn push_selections(state: &mut GameState, kind: ActionKind, ids: Vec<usize>) {
    for id in ids {
        push_action(state, kind, Some(id), None);
    }
}

// A play is one action per Monster when the entity it names wants a target, else one action
fn push_play_or_use(state: &mut GameState, kind: ActionKind, id_selected: usize) {
    if !entity_requires_target(&state.entities[id_selected]) {
        push_action(state, kind, Some(id_selected), None);
        return;
    }
    let id_monsters: Vec<usize> = state.combat.id_monsters.iter().flatten().copied().collect();
    for id_monster_target in id_monsters {
        push_action(state, kind, Some(id_selected), Some(id_monster_target));
    }
}

fn legal_actions_combat(state: &mut GameState) {
    // Playable Cards
    let id_cards_playable: Vec<usize> = state
        .combat
        .id_card_hand
        .iter()
        .copied()
        .filter(|&id| card_is_playable(state, &state.entities[id]))
        .collect();
    for id_card in id_cards_playable {
        push_play_or_use(state, ActionKind::CardPlay, id_card);
    }

    // Turn end
    push_action(state, ActionKind::TurnEnd, None, None);
}

fn legal_actions_reward(state: &mut GameState) {
    let id_cards_flat: Vec<usize> = state.reward.id_cards.iter().flatten().copied().collect();
    let id_relics = state.reward.id_relics.clone();
    let id_potions = state.reward.id_potions.clone();

    // Cards
    push_selections(state, ActionKind::RewardTakeCard, id_cards_flat.clone());

    // Singing Bowl: forfeit the picked Card's whole bundle for +2 max HP
    if has_relic(&state.id_relics, RelicName::SingingBowl) {
        push_selections(state, ActionKind::RewardSingingBowl, id_cards_flat);
    }

    // Relics
    push_selections(state, ActionKind::RewardTakeRelic, id_relics);

    // Potions
    if can_take_potions(state) {
        push_selections(state, ActionKind::RewardTakePotion, id_potions);
    }

    // Gold
    if state.reward.gold.is_some() {
        push_action(state, ActionKind::RewardTakeGold, None, None);
    }

    // Exit
    push_action(state, ActionKind::RoomExit, None, None);
}

fn legal_actions_event(state: &mut GameState) {
    // TODO: investigate pushing `EffectKind::RoomExit` at once
    if state.event.consumed {
        push_action(state, ActionKind::RoomExit, None, None);
        return;
    }

    let id_event_option_av: Vec<usize> = state
        .event
        .id_event_options
        .iter()
        .copied()
        .enumerate()
        .filter(|&(idx, _)| event_option_available(state, idx))
        .map(|(_, id)| id)
        .collect();

    push_selections(state, ActionKind::EventOptionSelect, id_event_option_av);
}

fn legal_actions_shop(state: &mut GameState) {
    // Shop is always exitable
    push_action(state, ActionKind::RoomExit, None, None);

    // TODO: make `shop_price` a field of `Shop` again
    let char_gold = state.entities[state.id_character].character_gold;
    let id_cards_aff = filter_affordable(state, &state.shop.id_cards);
    let id_relics_aff = filter_affordable(state, &state.shop.id_relics);
    let id_potions_aff = filter_affordable(state, &state.shop.id_potions);

    // Cards
    push_selections(state, ActionKind::ShopBuyCard, id_cards_aff);

    // Relics
    push_selections(state, ActionKind::ShopBuyRelic, id_relics_aff);

    // Potions
    if can_take_potions(state) {
        push_selections(state, ActionKind::ShopBuyPotion, id_potions_aff);
    }

    // Purge
    if !state.shop.purged && char_gold >= state.shop.purge_cost {
        let id_cards_purge = deck_cards(state, card_is_purgeable);
        push_selections(state, ActionKind::ShopPurge, id_cards_purge);
    }
}

fn legal_actions_map(state: &mut GameState) {
    let y_next = match state.location {
        Location::Start => Some(0),
        Location::Overworld { y, .. } => (y + 1 < MAP_HEIGHT).then_some(y + 1),
        Location::BossRoom => None,
    };
    let Some(y_next) = y_next else {
        return;
    };

    let id_rooms_reach: Vec<usize> = state.id_rooms[y_next]
        .iter()
        .flatten()
        .copied()
        .filter(|&id| room_is_reachable(state, &state.entities[id]))
        .collect();
    push_selections(state, ActionKind::RoomSelect, id_rooms_reach);
}

fn legal_actions_rest_site(state: &mut GameState) {
    let consumed = state.rest_site.consumed;
    let id_relics = state.id_relics;

    // Coffee Dripper: restless
    if !consumed && !has_relic(&id_relics, RelicName::CoffeeDripper) {
        push_action(state, ActionKind::Rest, None, None);
    }

    // Fusion Hammer: cannot upgrade
    if !consumed && !has_relic(&id_relics, RelicName::FusionHammer) {
        let upgradable = deck_cards(state, card_is_upgradable);
        push_selections(state, ActionKind::RestSmith, upgradable);
    }

    // Girya: can lift to gain Strength
    if !consumed
        && id_relics[RelicName::Girya as usize]
            .is_some_and(|id| state.entities[id].relic_counter < GIRYA_LIFT_MAX)
    {
        push_action(state, ActionKind::RestLift, None, None);
    }

    // Peace Pipe: can toke to purge a Card
    if !consumed && has_relic(&id_relics, RelicName::PeacePipe) {
        let purgeable = deck_cards(state, card_is_purgeable);
        push_selections(state, ActionKind::RestToke, purgeable);
    }

    // Shovel: can dig to find a random Relic
    if !consumed && has_relic(&id_relics, RelicName::Shovel) {
        push_action(state, ActionKind::RestDig, None, None);
    }

    // Leave once the site is used, or when no option is left (no soft-lock)
    if state.legal_actions.is_empty() {
        push_action(state, ActionKind::RoomExit, None, None);
    }
}

fn legal_actions_chest(state: &mut GameState) {
    if !state.chest.chest_opened {
        push_action(state, ActionKind::ChestOpen, None, None);
    }
    push_action(state, ActionKind::RoomExit, None, None);
}

fn legal_actions_potions(state: &mut GameState) {
    let id_potions_usable: Vec<usize> = state
        .id_potions
        .iter()
        .copied()
        .filter(|&id| potion_is_usable(state, &state.entities[id]))
        .collect();

    // Potion use
    for id_potion in id_potions_usable {
        push_play_or_use(state, ActionKind::PotionUse, id_potion);
    }

    // Potion discard
    let id_potions_own = state.id_potions.clone();
    push_selections(state, ActionKind::PotionDiscard, id_potions_own);
}

fn filter_affordable(state: &GameState, stock: &[usize]) -> Vec<usize> {
    let char_gold = state.entities[state.id_character].character_gold;
    stock
        .iter()
        .copied()
        .filter(|&id| char_gold >= state.entities[id].shop_price)
        .collect()
}

fn deck_cards(state: &GameState, accepts: fn(&Entity) -> bool) -> Vec<usize> {
    state
        .id_card_deck
        .iter()
        .copied()
        .filter(|&id| accepts(&state.entities[id]))
        .collect()
}

fn can_take_potions(state: &GameState) -> bool {
    belt_has_room(&state.id_potions, state.potion_slots_max)
        // Sozu: Potions can't be obtained, so neither taken nor bought
        && !has_relic(&state.id_relics, RelicName::Sozu)
}

// The id joins the inputs of the parked pick and the effect runs again, resolving once its
// whole count is there
fn handle_effect_pending_resolve(state: &mut GameState, id: usize) {
    let effect_pending = state
        .effect_pending
        .take()
        .expect("EffectPendingResolve requires a pending effect");
    state.id_input.push(id);
    state.effect_buf.push(effect_pending);
}

// Ends an InputUpTo halt early: the pick closes at what has been input, nothing more
fn handle_selection_skip(state: &mut GameState) {
    let effect_pending = state
        .effect_pending
        .take()
        .expect("SelectionSkip requires a pending effect");
    let Target::Resolve {
        candidate_pool,
        filter,
        ..
    } = effect_pending.target
    else {
        unreachable!("effect_pending carries a Resolve target")
    };
    state.effect_buf.push(Effect {
        target: Target::Resolve {
            candidate_pool,
            filter,
            selection_kind: SelectionKind::Input {
                count: state.id_input.len() as u16,
            },
        },
        ..effect_pending
    });
}
