use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardRarity;
use crate::types::DeltaSign;
use crate::types::RelicName;

// Bonfire Spirits: purge the offered card, reward keyed on its rarity
pub fn process_effect_bonfire_offer(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("BonfireOffer requires id_target");
    let id_character = state.id_character;
    let card_rarity = state.entities[id_card].card_rarity;

    // Used by both `CardRarity:Uncommon` and `CardRarity:Rare` arms
    let effect_heal_full = Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Relative {
                numerator: 1,
                denominator: 1,
            },
        },
        id_source: None,
        target: Target::Direct(Some(id_character)),
    };

    // Dispatch according to the Card's rarity
    match card_rarity {
        CardRarity::Curse => state.effect_queue.push_front(Effect {
            kind: EffectKind::RelicGrantSpecific {
                name: RelicName::SpiritPoop,
                fallback_circlet: true,
            },
            id_source: None,
            target: Target::Direct(None),
        }),
        CardRarity::Basic => {}
        CardRarity::Common | CardRarity::Special => state.effect_queue.push_front(Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(5),
            },
            id_source: None,
            target: Target::Direct(Some(id_character)),
        }),
        CardRarity::Uncommon => state.effect_queue.push_front(effect_heal_full),
        CardRarity::Rare => {
            state.effect_queue.push_front(effect_heal_full);
            state.effect_queue.push_front(Effect {
                kind: EffectKind::MaxHealthDelta {
                    sign: DeltaSign::Gain,
                    amount: Amount::Absolute(10),
                },
                id_source: None,
                target: Target::Direct(Some(id_character)),
            });
        }
    }
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardPurge,
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
}
