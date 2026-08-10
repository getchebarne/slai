use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::KnowingSkullWish;
use crate::effect::Target;
use crate::events::EventKind;
use crate::events::KNOWING_SKULL_GOLD;
use crate::game::GameState;
use crate::types::CardColor;
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::utils::mode_top_mut;

// Knowing Skull: pay the wish's HP cost, receive, and escalate that cost by one
pub fn process_effect_knowing_skull_ask(state: &mut GameState, wish: KnowingSkullWish) {
    let Mode::Event {
        kind:
            EventKind::KnowingSkull {
                potion_cost,
                gold_cost,
                card_cost,
            },
        ..
    } = mode_top_mut(&mut state.mode_stack)
    else {
        unreachable!("KnowingSkullAsk outside a Knowing Skull event")
    };
    let (cost, reward) = match wish {
        KnowingSkullWish::Potion => (potion_cost, EffectKind::PotionAddRandom { limited: false }),
        KnowingSkullWish::Gold => (
            gold_cost,
            EffectKind::GoldDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(KNOWING_SKULL_GOLD),
            },
        ),
        KnowingSkullWish::Card => (
            card_cost,
            EffectKind::CardAddRandom {
                color: CardColor::Colorless,
                kind: None,
                pile: CardPile::Deck,
                count: 1,
                cost_zero: None,
                upgraded: false,
                rarity: Some(CardRarity::Uncommon),
            },
        ),
    };
    let hp_cost = *cost;
    *cost += 1;

    // Executes in reverse: pay first, then receive
    state.effect_queue.push_front(Effect {
        kind: reward,
        id_source: None,
        target: Target::Direct(None),
    });
    state.effect_queue.push_front(Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(hp_cost as u16),
        },
        id_source: None,
        target: Target::Direct(Some(state.id_character)),
    });
}
