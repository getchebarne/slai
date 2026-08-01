use crate::consts::DISCOVER_PICK_COUNT;
use crate::consts::MAX_SIZE_DECK;
use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::relics::RELIC_COUNTERS_PER_COMBAT;
use crate::relics::RELIC_COUNTERS_PER_TURN;
use crate::relics::iter_owned_relics;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::DeltaSign;
use crate::types::Energy;
use crate::types::Mode;
use crate::types::MonsterKind;
use crate::types::RelicName;
use crate::utils::has_relic;
use crate::utils::push_entity;
use crate::utils::shuffle;

pub fn process_effect_combat_start(
    state: &mut GameState,
    event_gold: Option<Amount>,
    event_relic: Option<RelicName>,
    event_relic_roll: bool,
) {
    let Mode::Combat {
        id_pile_draw,
        id_monsters,
        id_picked_monster,
        energy,
        this_turn_cards_played,
        this_turn_panache,
        bomb_countdown,
        this_combat_damage_instances_taken,
        this_combat_escaped,
        event_gold: combat_event_gold,
        event_relic: combat_event_relic,
        event_relic_roll: combat_event_relic_roll,
        ..
    } = &mut state.mode
    else {
        unreachable!("process_effect_combat_start outside Combat mode")
    };

    // Stamp the event fight's reward parameters (None for ordinary fights)
    *combat_event_gold = event_gold;
    *combat_event_relic = event_relic;
    *combat_event_relic_roll = event_relic_roll;

    // Elite fights are identified by the monsters, not the room: Dead Adventurer's
    // elite returns inside an event room
    let is_elite_fight = id_monsters
        .iter()
        .flatten()
        .any(|&id| state.entities[id].monster_kind == MonsterKind::Elite);
    let is_boss_fight = id_monsters
        .iter()
        .flatten()
        .any(|&id| state.entities[id].monster_kind == MonsterKind::Boss);

    // Base 3, +1 per owned energy relic; Slaver's Collar counts only in elite and boss fights
    let mut energy_max = 3;
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
    if has_relic(&state.id_relics, RelicName::SlaversCollar) && (is_elite_fight || is_boss_fight) {
        energy_max += 1;
    }

    // Energy starts empty; the turn-1 refill fills
    *energy = Energy {
        energy_current: 0,
        energy_max,
    };

    *this_combat_damage_instances_taken = 0;
    *this_combat_escaped = false;
    *this_turn_cards_played = 0;
    *this_turn_panache = 0;
    *bomb_countdown = 0;

    // Combat can end mid-turn, skipping the turn-end reset
    for &name in RELIC_COUNTERS_PER_TURN
        .iter()
        .chain(RELIC_COUNTERS_PER_COMBAT)
    {
        if let Some(id) = state.id_relics[name as usize] {
            state.entities[id].relic_counter = 0;
        }
    }

    // Innate and bottled cards sit on top of the draw pile, ahead of the shuffled rest
    let mut other_ids: [usize; MAX_SIZE_DECK] = [0; MAX_SIZE_DECK];
    let mut other_n: usize = 0;
    let mut innate_ids: [usize; MAX_SIZE_DECK] = [0; MAX_SIZE_DECK];
    let mut innate_n: usize = 0;

    for i in 0..state.id_deck.len() {
        let id_card_src = state.id_deck[i];
        let card = state.entities[id_card_src];
        let id_card = push_entity(&mut state.entities, card);
        if card.card_innate || card.card_bottled {
            innate_ids[innate_n] = id_card;
            innate_n += 1;
        } else {
            other_ids[other_n] = id_card;
            other_n += 1;
        }
    }

    shuffle(&mut other_ids[..other_n], &mut state.rng);

    id_pile_draw.clear();
    for &id in &other_ids[..other_n] {
        id_pile_draw.push(id);
    }
    for &id in &innate_ids[..innate_n] {
        id_pile_draw.push(id);
    }

    *id_picked_monster = None;

    // Monster MoveUpdates already queued at MonsterSpawn; queue character TurnStart
    state.effect_queue.push_front(Effect {
        kind: EffectKind::TurnStart,
        id_source: None,
        target: Target::Direct(Some(state.id_character)),
    });

    // Toolbox: choose 1 of 3 colorless cards (Bandage Up mirrors the source's
    // healing-tag exclusion) at printed cost, before the opening draw. Inline
    // because relic_effects_on_combat_start drains post-draw
    if has_relic(&state.id_relics, RelicName::Toolbox) {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardDiscoverPick { cost_zero: None },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Discover,
                filter: CandidateFilter::Any,
                selection_kind: SelectionKind::Input { count: 1 },
            },
        });
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardDiscoverRoll {
                kind: None,
                color: CardColor::Colorless,
                exclude: &[CardName::BandageUp],
                count: DISCOVER_PICK_COUNT,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }

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
            kind: EffectKind::EnergyDelta {
                sign: DeltaSign::Gain,
                amount: 2,
            },
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
    if has_relic(&state.id_relics, RelicName::PreservedInsect) && is_elite_fight {
        for id in id_monsters.iter().flatten().copied() {
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

    // Girya: combats open with Strength equal to lifts
    if let Some(id) = state.id_relics[RelicName::Girya as usize]
        && state.entities[id].relic_counter > 0
    {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: state.entities[id].relic_counter,
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
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
    if has_relic(&state.id_relics, RelicName::SlingOfCourage) && is_elite_fight {
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
