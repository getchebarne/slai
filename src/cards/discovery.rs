use crate::cards::make_entity_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::effect::effect_discover_pick;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::CostScope;

pub static DISCOVERY: Entity = make_entity_card(
    CardName::Discovery,
    CardKind::Skill,
    CardColor::Colorless,
    CardRarity::Uncommon,
    1,
    CardCostKind::Fixed,
    false,
    true,
    false,
    false,
    false,
    &[
        Effect {
            kind: EffectKind::CardDiscoverRoll {
                kind: None,
                color: CardColor::Green,
                exclude: &[],
                count: 3,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        effect_discover_pick(Some(CostScope::Turn), CardPile::Hand),
    ],
    &[],
    &[],
    PlayRestriction::Always,
);
// Upgraded
pub static DISCOVERY_PLUS: Entity = Entity {
    card_upgraded: true,
    card_exhaust: false, // Doesn't exhaust
    ..DISCOVERY
};
