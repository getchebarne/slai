use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static UNLOAD: Entity = make_entity_card(
    CardName::Unload,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Rare,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 14,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
        Effect {
            kind: EffectKind::UnloadDiscard,
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static UNLOAD_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = UNLOAD.card_effects;
        a[0].kind = EffectKind::DamagePhysical {
            amount: 18,
            lifesteal: false,
        }; // +4 damage
        a
    },
    ..UNLOAD
};
