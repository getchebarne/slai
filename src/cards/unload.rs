use crate::cards::CardTemplate;
use crate::cards::make_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static UNLOAD: CardTemplate = make_card_template(
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
pub static UNLOAD_PLUS: CardTemplate = CardTemplate {
    upgraded: true,
    effects: {
        let mut effects = UNLOAD.effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 18,
            lifesteal: false,
        }; // +4 damage
        effects
    },
    ..UNLOAD
};
