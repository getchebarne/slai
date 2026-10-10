use crate::consts::POTION_SLOTS_MAX;
use crate::effect::EFFECT_DU_VU_DOLL_RECOUNT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RelicExclusion;
use crate::effect::Target;
use crate::game::GameState;
use crate::potions::get_potion;
use crate::relics::egg_upgrades_kind;
use crate::relics::get_relic;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::ChestKind;
use crate::types::EventName;
use crate::types::RelicName;
use crate::types::RelicTier;
use crate::types::reward_reset;
use crate::utils::draw_relic_excluding;
use crate::utils::push_entity;

pub fn process_effect_relic_adopt(id_target: Option<usize>, state: &mut GameState) {
    let id_relic = id_target.expect("RelicAdopt requires id_target");
    let name = state.entities[id_relic].relic_name;

    // A second Circlet only counts on the one already held
    if name == RelicName::Circlet
        && let Some(id_held) = state.id_relics[RelicName::Circlet as usize]
    {
        state.entities[id_held].relic_counter += 1;
        return;
    }

    // Flag Relic as owned and stamp its acquisition order
    state.id_relics[name as usize] = Some(id_relic);
    state.entities[id_relic].relic_seq = state.relic_seq_next;
    state.relic_seq_next += 1;

    // Queue the Relic's pickup effects
    queue_pickup_effects(state, id_relic);
}

fn queue_pickup_effects(state: &mut GameState, id_relic: usize) {
    let name = state.entities[id_relic].relic_name;

    // Pickup effects execute in slice order (push_front reverses)
    for &effect in state.entities[id_relic]
        .relic_effects_on_pickup
        .iter()
        .rev()
    {
        state.effect_queue.push_front(effect);
    }

    match name {
        // Potion Belt: gain 2 Potion slots
        RelicName::PotionBelt => {
            state.potion_slots_max = (state.potion_slots_max + 2).min(POTION_SLOTS_MAX as u8);
        }

        // An Egg upgrades the matching Cards on offer: the Shop's stock and the staged bundles
        RelicName::EggFrozen | RelicName::EggMolten | RelicName::EggToxic => {
            let mut id_cards_offered: Vec<usize> = Vec::new();
            if state.shop.active {
                id_cards_offered.extend(state.shop.id_cards_price.iter().map(|&(id, _)| id));
            }
            if state.reward.active {
                id_cards_offered.extend(state.reward.id_cards.iter().flatten());
            }
            for id_card in id_cards_offered {
                let card = &state.entities[id_card];
                if !card.card_upgraded && egg_upgrades_kind(card.card_kind, &state.id_relics) {
                    state.effect_queue.push_front(Effect {
                        kind: EffectKind::CardUpgrade,
                        id_source: None,
                        target: Target::Direct(Some(id_card)),
                    });
                }
            }
        }

        // Du-Vu Doll: its counter starts at the deck's Curse count
        RelicName::DuVuDoll => state.effect_queue.push_front(EFFECT_DU_VU_DOLL_RECOUNT),

        // Sacred Bark: every Potion already in the belt becomes its doubled variant
        RelicName::SacredBark => {
            for &id_potion in state.id_potions.iter() {
                let name = state.entities[id_potion].potion_name;
                state.entities[id_potion] = get_potion(name, true);
            }
        }

        // Pandora's Box: every starter Strike / Defend becomes a random Card
        RelicName::PandorasBox => {
            for &id in &state.id_card_pile_deck {
                if matches!(
                    state.entities[id].card_name,
                    CardName::Strike | CardName::Defend
                ) {
                    state.effect_queue.push_front(Effect {
                        kind: EffectKind::CardTransform { upgraded: false },
                        id_source: None,
                        target: Target::Direct(Some(id)),
                    });
                }
            }
        }

        // Calling Bell: gain Curse of the Bell plus a Common, an Uncommon, and a Rare Relic
        RelicName::CallingBell => {
            // The bell arrives from the boss chest's closed pick or Neow's consumed blessing
            assert!(
                !state.reward.active
                    && ((state.chest.active && state.chest.chest_kind == ChestKind::Boss)
                        || (state.event.active
                            && matches!(state.event.name, EventName::Neow)
                            && state.event.consumed)),
                "Calling Bell adopts from the boss chest or Neow"
            );

            // One screenless Relic per rarity; a fallback Circlet counts on one already held
            let mut id_relics = Vec::with_capacity(3);
            for tier in [RelicTier::Common, RelicTier::Uncommon, RelicTier::Rare] {
                let name = draw_relic_excluding(state, tier, RelicExclusion::Screenless);
                id_relics.push(push_entity(&mut state.entities, get_relic(name)));
            }

            // The staged offer replaces the host it came from: the boss chest or Neow's blessing closes
            state.chest.active = false;
            state.event.active = false;
            reward_reset(&mut state.reward);
            state.reward.id_relics = id_relics;
            state.reward.active = true;
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardAdd {
                    card_name: CardName::CurseOfTheBell,
                    card_pile: CardPile::Deck,
                    count: 1,
                    upgraded: false,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }

        // Ring of the Serpent: replaces the starter; RingOfTheSnake's combat-start draw is lost
        RelicName::RingOfTheSerpent => {
            state.id_relics[RelicName::RingOfTheSnake as usize] = None;
        }

        _ => {}
    }
}
