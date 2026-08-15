use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;

pub static FINESSE: Entity = make_entity_card(
    CardName::Finesse,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Uncommon,
    0,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 2 },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::CardDraw { count: 1 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static FINESSE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut effects = FINESSE.card_effects;
        effects[0].kind = EffectKind::BlockGain { amount: 4 }; // +2 block
        effects
    },
    ..FINESSE
};
