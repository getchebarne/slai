use std::collections::VecDeque;

use rand::Rng;

use crate::cards::get_random_cards_of_kind_and_color;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CombatState;

pub fn process_effect_card_discover_select(
    kind: CardKind,
    color: CardColor,
    count: u8,
    entities: &mut Vec<Entity>,
    combat: &mut CombatState,
    effect_queue: &mut VecDeque<Effect>,
    rng: &mut impl Rng,
) {
    let card_picks = get_random_cards_of_kind_and_color(kind, color, count as usize, rng);
    combat.id_pick.clear();
    combat.id_pick.reserve(card_picks.len());
    for card_pick in card_picks {
        let id = entities.len();
        entities.push(card_pick);
        combat.id_pick.push(id);
    }
    effect_queue.push_front(Effect {
        kind: EffectKind::CardDiscoverPick,
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::IdPick,
            selection: SelectionKind::Input { count: 1 },
        },
    });
}
