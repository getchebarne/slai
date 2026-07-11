use crate::utils::has_relic;
use crate::consts::MAX_SIZE_DECK;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::map::get_active_room_kind;
use crate::modifier::ModifierKind;
use crate::relics::RELIC_COUNTERS_PER_COMBAT;
use crate::relics::RELIC_COUNTERS_PER_TURN;
use crate::relics::iter_owned_relics;
use crate::types::CardKind;
use crate::types::RelicName;
use crate::types::RoomKind;
use crate::utils::push_entity;
use crate::utils::shuffle;

pub fn process_effect_combat_start(state: &mut GameState) {
    state.this_combat_damage_instances_taken = 0;
    state.this_combat_escaped = false;
    state.this_turn_cards_played = 0;

    // Combat can end mid-turn, skipping the turn-end reset
    for &name in RELIC_COUNTERS_PER_TURN
        .iter()
        .chain(RELIC_COUNTERS_PER_COMBAT)
    {
        if let Some(id) = state.id_relics[name as usize] {
            state.entities[id].relic_counter = 0;
        }
    }

    // Innate cards sit on top of the draw pile, ahead of the shuffled rest
    let mut other_ids: [usize; MAX_SIZE_DECK] = [0; MAX_SIZE_DECK];
    let mut other_n: usize = 0;
    let mut innate_ids: [usize; MAX_SIZE_DECK] = [0; MAX_SIZE_DECK];
    let mut innate_n: usize = 0;

    for i in 0..state.id_deck.len() {
        let id_card_src = state.id_deck[i];
        let card = state.entities[id_card_src];
        let id_card = push_entity(&mut state.entities, card);
        if card.card_innate {
            innate_ids[innate_n] = id_card;
            innate_n += 1;
        } else {
            other_ids[other_n] = id_card;
            other_n += 1;
        }
    }

    shuffle(&mut other_ids[..other_n], &mut state.rng);

    state.id_pile_draw.clear();
    for &id in &other_ids[..other_n] {
        state.id_pile_draw.push(id);
    }
    for &id in &innate_ids[..innate_n] {
        state.id_pile_draw.push(id);
    }

    state.id_picked_monster = None;

    // Monster MoveUpdates already queued at MonsterSpawn; queue character TurnStart
    state.effect_queue.push_front(Effect {
        kind: EffectKind::TurnStart,
        id_source: None,
        target: Target::Direct(Some(state.id_character)),
    });

    for (_name, id_relic) in iter_owned_relics(&state.id_relics) {
        for &eff in state.entities[id_relic].relic_effects_on_combat_start {
            state.effect_queue.push_back(eff);
        }
    }

    // Ancient Tea Set
    if let Some(id) = state.id_relics[RelicName::AncientTeaSet as usize]
        && state.entities[id].relic_counter == 1
    {
        state.entities[id].relic_counter = 0;
        state.effect_queue.push_back(Effect {
            kind: EffectKind::EnergyGain { amount: 2 },
            id_source: None,
            target: Target::Direct(None),
        });
    }

    // Pen Nib: a charge primed at combat end (counter 9) survives as the counter but not
    // the modifier, so re-apply the double-next-attack charge here
    if let Some(id) = state.id_relics[RelicName::PenNib as usize]
        && state.entities[id].relic_counter == 9
    {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::PenNib,
                stacks: 1,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Preserved Insect
    if has_relic(&state.id_relics, RelicName::PreservedInsect)
        && matches!(
            get_active_room_kind(&state.id_rooms, state.location, &state.entities),
            Some(RoomKind::CombatElite)
        )
    {
        for id in state.id_monsters.iter().flatten().copied() {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::HealthSet {
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

    // Du-Vu Doll
    if has_relic(&state.id_relics, RelicName::DuVuDoll) {
        let num_curses = state
            .id_deck
            .iter()
            .filter(|&&id| state.entities[id].card_kind == CardKind::Curse)
            .count();

        if num_curses > 0 {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Strength,
                    stacks: num_curses as i16,
                },
                id_source: None,
                target: Target::Direct(Some(state.id_character)),
            });
        }
    }

    // Sling of Courage: Elite fights open with 2 Strength
    if has_relic(&state.id_relics, RelicName::SlingOfCourage)
        && matches!(
            get_active_room_kind(&state.id_rooms, state.location, &state.entities),
            Some(RoomKind::CombatElite)
        )
    {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: 2,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }
}
