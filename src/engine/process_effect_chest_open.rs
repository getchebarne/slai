use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RewardSource;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardColor;
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::Frame;
use crate::types::RelicName;
use crate::utils::frame_top_mut;
use crate::utils::has_relic;

pub fn process_effect_chest_open(state: &mut GameState) {
    let Frame::Chest {
        chest_kind,
        chest_opened,
    } = frame_top_mut(&mut state.frame_stack)
    else {
        unreachable!("ChestOpen outside the Chest frame")
    };
    let chest_kind = *chest_kind;
    *chest_opened = true;

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
