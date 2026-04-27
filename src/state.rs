use rand::rngs::SmallRng;
use std::collections::VecDeque;

use crate::consts::{MAP_HEIGHT, MAP_WIDTH, MAX_MONSTERS};
use crate::effect::Effect;
use crate::entity::Entity;
use crate::types::*;

#[derive(Debug, Clone, Copy)]
pub struct Energy {
    pub current: u8,
    pub max: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    Start,
    Overworld { y: usize, x: usize },
    BossRoom,
}

// GameState: the single source of truth
#[derive(Debug, Clone)]
pub struct GameState {
    pub ascension: u8,
    pub rng: SmallRng,

    // Finite-state-machine-like to track in which "screen" the game is in
    // e.g. combat, awaiting card discard selection, room selection, etc.
    pub phase: Phase,

    // Effect queue
    pub effect_queue: VecDeque<Effect>,

    // Location
    pub location: Location,

    // Energy
    pub energy: Energy,

    // Entities
    pub entities: Vec<Entity>,

    // Rooms
    pub id_rooms: [[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],

    // Entities / Character
    pub id_character: usize,

    // Entities / Monsters
    pub id_monsters: [usize; MAX_MONSTERS],
    pub monster_count: u8,

    // Entities / Monsters / Target
    pub id_card_target: Option<usize>,

    // Entities / Card / Deck
    pub id_deck: Vec<usize>,

    // Entities / Card / Combat piles
    pub id_pile_draw: Vec<usize>,
    pub id_hand: Vec<usize>,
    pub id_pile_discard: Vec<usize>,
    pub id_pile_exhaust: Vec<usize>,

    // Entities / Card / Combat rewards
    pub id_card_rewards: Vec<usize>,

    // Per-draw bookkeeping: id of the most recently drawn card (set inside
    // `process_effect_card_draw` on every successful draw, regardless of
    // whether the card landed in hand or went to discard due to hand cap).
    // Consumed by post-draw inspection effects like EscapePlanCheck.
    pub last_drawn_card: Option<usize>,

    // Per-play bookkeeping: id of the card currently being played (set at the
    // top of `process_effect_card_play`). Used by self-referential effects
    // like GlassKnifeDecay that need to mutate the played card's own state.
    pub last_played_card: Option<usize>,

    // Per-turn counters, reset at the start of `process_effect_turn_end_character`.
    // Used by SneakyStrike, Finisher, and Tier-5 Eviscerate.
    pub cards_discarded_this_turn: u8,
    pub attacks_played_this_turn: u8,

    // Per-combat counter: number of damage events the character has taken
    // (incremented by 1 per HealthLoss with amount > 0; NOT scaled by HP
    // amount). Reset to 0 at combat_start. Read by MasterfulStab's
    // GrowsOnDamageInstanceTaken cost variant.
    pub instances_of_damage_taken_this_combat: u8,

    // Nightmare-pending: snapshot copies of the picked card(s), spawned into
    // hand at the next character TurnStart (post-draw). Cleared on combat
    // end. Holds full Entity snapshots so per-instance state (GlassKnife
    // damage decay, free-to-play flag, etc.) is preserved on the spawned
    // copies — matches StS `makeStatEquivalentCopy` semantics.
    pub cards_nightmare: Vec<Entity>,
}
