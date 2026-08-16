use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::modifier::ModifierKind;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static NOXIOUS_FUMES: Entity = make_entity_card(
    CardName::NoxiousFumes,
    CardKind::Power,
    CardColor::Green,
    CardRarity::Uncommon,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::NoxiousFumes,
            stacks: 2,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static NOXIOUS_FUMES_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut effects = NOXIOUS_FUMES.card_effects;
        effects[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::NoxiousFumes,
            stacks: 3, // +1 poison
        };
        effects
    },
    ..NOXIOUS_FUMES
};
