use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::DeltaSign;

pub static ADRENALINE: Entity = make_entity_card(
    CardName::Adrenaline,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Rare,
    0,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::EnergyDelta {
                sign: DeltaSign::Gain,
                amount: 1,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::CardDraw { count: 2 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static ADRENALINE_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut a = ADRENALINE.card_effects;
        a[0].kind = EffectKind::EnergyDelta {
            sign: DeltaSign::Gain,
            amount: 2,
        }; // +1 energy gain
        a
    },
    ..ADRENALINE
};
