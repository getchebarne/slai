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

pub static OUTMANEUVER: Entity = make_entity_card(
    CardName::Outmaneuver,
    CardKind::Skill,
    CardColor::Green,
    CardRarity::Common,
    1,
    CardCostKind::Fixed,
    false,
    false,
    false,
    false,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::NextTurnEnergy,
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
pub static OUTMANEUVER_PLUS: Entity = Entity {
    card_upgraded: true,
    card_effects: {
        let mut effects = OUTMANEUVER.card_effects;
        effects[0].kind = EffectKind::ModifierGain {
            kind: ModifierKind::NextTurnEnergy,
            stacks: 3, // +1 next-turn-energy
        };
        effects
    },
    ..OUTMANEUVER
};
