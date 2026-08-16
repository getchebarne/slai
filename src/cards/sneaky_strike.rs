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

pub static SNEAKY_STRIKE: Entity = make_entity_card(
    CardName::SneakyStrike,
    CardKind::Attack,
    CardColor::Green,
    CardRarity::Common,
    2,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 12,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_MONSTER_PICKED,
        },
        Effect {
            kind: EffectKind::SneakyStrikeProc { energy: 2 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static SNEAKY_STRIKE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut effects = SNEAKY_STRIKE.card_effects;
        effects[0].kind = EffectKind::DamagePhysical {
            amount: 16,
            lifesteal: false,
        }; // +4 damage
        effects
    },
    ..SNEAKY_STRIKE
};
