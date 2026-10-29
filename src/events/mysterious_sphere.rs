use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RelicRoll;
use crate::effect::Target;
use crate::events::EOT_LEAVE;
use crate::events::EventLoot;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::MonsterName;
use crate::types::RelicTier;

const fn spawn_orb_walker(slot: usize) -> Effect {
    Effect {
        kind: EffectKind::MonsterSpawn {
            name: MonsterName::OrbWalker,
            minion: false,
            slot,
        },
        id_source: None,
        target: Target::Direct(None),
    }
}

// Open Sphere starts the fight at once: two Orb Walkers, with gold and a Rare Relic on the line
const OPTION_OPEN_SPHERE: &[Effect] = &[
    spawn_orb_walker(0),
    spawn_orb_walker(1),
    Effect {
        kind: EffectKind::CombatStart { elite: false },
        id_source: None,
        target: Target::Direct(None),
    },
];

// The Sphere's purse: 45-55 gold and a Rare Relic, paid by `fight_loot`
pub const FIGHT_LOOT: EventLoot = EventLoot {
    gold: Some(Amount::Range { min: 45, max: 55 }),
    relics: [Some(RelicRoll::Tier(RelicTier::Rare)), None],
};

static EOTS_BASE: &[EventOptionTemplate] =
    &[make_event_option_template(OPTION_OPEN_SPHERE), EOT_LEAVE];

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}

pub fn option_available(_state: &GameState, _idx: usize) -> bool {
    true
}
