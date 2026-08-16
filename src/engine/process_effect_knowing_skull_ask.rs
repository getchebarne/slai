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
use crate::types::Focus;
use crate::utils::context_focus;

// Knowing Skull: pay the wish's HP cost, receive, and escalate that cost by one
pub fn process_effect_knowing_skull_ask(state: &mut GameState, wish: KnowingSkullWish) {
    assert!(
        context_focus(state) == Focus::Event,
        "KnowingSkullAsk outside the Event context"
    );
    let EventKind::KnowingSkull {
        potion_cost_hp,
        gold_cost_hp,
        card_cost_hp,
    } = &mut state.event.event_kind
    else {
        unreachable!("KnowingSkullAsk outside a Knowing Skull event")
    };
    let (cost, reward) = match wish {
        KnowingSkullWish::Potion => (
            potion_cost_hp,
            EffectKind::PotionAddRandom { limited: false },
        ),
        KnowingSkullWish::Gold => (
            gold_cost_hp,
            EffectKind::GoldDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(KNOWING_SKULL_GOLD),
            },
        ),
        KnowingSkullWish::Card => (
            card_cost_hp,
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

    // Snapshot cost, increase it after
    let cost_snap = *cost;
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
            amount: Amount::Absolute(cost_snap as u16),
        },
        id_source: None,
        target: Target::Direct(Some(state.id_character)),
    });
}
