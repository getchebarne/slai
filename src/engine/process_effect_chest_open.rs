use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RewardSource;
use crate::effect::Target;
use crate::game::GameState;
use crate::game::Location;
use crate::map::room_at_mut;
use crate::types::CardColor;
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::Mode;
use crate::types::RelicName;
use crate::utils::has_relic;
use crate::utils::mode_replace;

pub fn process_effect_chest_open(state: &mut GameState) {
    let Location::Overworld { y, x } = state.location else {
        panic!("ChestOpen outside Overworld");
    };

    let room =
        room_at_mut(&state.id_rooms, &mut state.entities, y, x).expect("ChestOpen room missing");
    let chest_kind = room
        .room_chest_kind
        .expect("ChestOpen with no chest_kind on room");

    room.room_chest_opened = true;
    mode_replace(&mut state.mode_stack, Mode::ChestOpened);

    // Cursed Key: opening a chest adds a random Curse to the deck
    if has_relic(&state.id_relics, RelicName::CursedKey) {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CardAddRandom {
                color: CardColor::Curse,
                kind: None,
                pile: CardPile::Deck,
                count: 1,
                cost_zero: None,
                upgraded: false,
                rarity: Some(CardRarity::Curse),
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }

    // N'loth's Hungry Face: the next chest opened is empty (one-shot)
    if let Some(id) = state.id_relics[RelicName::NlothsHungryFace as usize]
        && !state.entities[id].relic_used_up
    {
        state.entities[id].relic_used_up = true;
        return;
    }

    state.effect_queue.push_back(Effect {
        kind: EffectKind::RewardRoll {
            source: RewardSource::Chest { kind: chest_kind },
        },
        id_source: None,
        target: Target::Direct(None),
    });
}
