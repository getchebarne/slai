mod a_thousand_cuts;
mod accuracy;
mod acrobatics;
mod adrenaline;
mod after_image;
mod alchemize;
mod all_out_attack;
mod apotheosis;
mod apparition;
mod ascenders_bane;
mod backflip;
mod backstab;
mod bandage_up;
mod bane;
mod bite;
mod blade_dance;
mod blind;
mod blur;
mod bouncing_flask;
mod bullet_time;
mod burn;
mod burst;
mod calculated_gamble;
mod caltrops;
mod catalyst;
mod choke;
mod chrysalis;
mod cloak_and_dagger;
mod concentrate;
mod corpse_explosion;
mod crippling_poison;
mod curse_of_the_bell;
mod dagger_spray;
mod dagger_throw;
mod dark_shackles;
mod dash;
mod dazed;
mod deadly_poison;
mod decay;
mod deep_breath;
mod defend;
mod deflect;
mod die_die_die;
mod discovery;
mod distraction;
mod dodge_and_roll;
mod doppelganger;
mod doubt;
mod dramatic_entrance;
mod endless_agony;
mod enlightenment;
mod envenom;
mod escape_plan;
mod eviscerate;
mod expertise;
mod finesse;
mod finisher;
mod flash_of_steel;
mod flechettes;
mod flying_knee;
mod footwork;
mod forethought;
mod glass_knife;
mod good_instincts;
mod grand_finale;
mod hand_of_greed;
mod heel_hook;
mod impatience;
mod infinite_blades;
mod injury;
mod jack_of_all_trades;
mod jax;
mod leg_sweep;
mod madness;
mod magnetism;
mod malaise;
mod master_of_strategy;
mod masterful_stab;
mod mayhem;
mod metamorphosis;
mod mind_blast;
mod necronomicurse;
mod neutralize;
mod nightmare;
mod normality;
mod noxious_fumes;
mod outmaneuver;
mod pain;
mod panacea;
mod panache;
mod panic_button;
mod parasite;
mod phantasmal_killer;
mod piercing_wail;
mod poisoned_stab;
mod predator;
mod prepared;
mod purity;
mod quick_slash;
mod reflex;
mod regret;
mod riddle_with_holes;
mod ritual_dagger;
mod sadistic_nature;
mod secret_technique;
mod secret_weapon;
mod setup;
mod shame;
mod shiv;
mod skewer;
mod slice;
mod slimed;
mod sneaky_strike;
mod storm_of_steel;
mod strike;
mod sucker_punch;
mod survivor;
mod swift_strike;
mod tactician;
mod terror;
mod the_bomb;
mod thinking_ahead;
mod tools_of_the_trade;
mod transmutation;
mod trip;
mod unload;
mod violence;
mod well_laid_plans;
mod wound;
mod wraith_form;
mod writhe;

use crate::consts::MAX_EFFECTS_PER_CARD;
use crate::effect::EFFECT_ZERO;
use crate::effect::Effect;
use crate::entity::CardCostKind;
use crate::entity::ENTITY_ZERO;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::entity::PlayRestriction;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::utils::card_name_never_obtainable;
use crate::utils::shuffle;
use strum::EnumCount;

// Totality relies on the len == COUNT and no-duplicate asserts below
const fn build_card_by_name() -> [&'static CardTemplate; CardName::COUNT] {
    let mut buf = [ALL_CARDS[0]; CardName::COUNT];
    let mut idx = 0;
    while idx < ALL_CARDS.len() {
        buf[ALL_CARDS[idx].name as usize] = ALL_CARDS[idx];
        idx += 1;
    }
    buf
}

static CARD_BY_NAME: [&'static CardTemplate; CardName::COUNT] = build_card_by_name();

pub fn get_card(name: CardName, upgraded: bool) -> Entity {
    instance_card_from_template(card_template(name, upgraded))
}

pub fn card_template(name: CardName, upgraded: bool) -> &'static CardTemplate {
    if !upgraded {
        return CARD_BY_NAME[name as usize];
    }
    // Named, not `_`, so a new CardName missing its _PLUS arm stays a compile error
    match name {
        CardName::AThousandCuts => &a_thousand_cuts::A_THOUSAND_CUTS_PLUS,
        CardName::Accuracy => &accuracy::ACCURACY_PLUS,
        CardName::Acrobatics => &acrobatics::ACROBATICS_PLUS,
        CardName::Adrenaline => &adrenaline::ADRENALINE_PLUS,
        CardName::AfterImage => &after_image::AFTER_IMAGE_PLUS,
        CardName::Alchemize => &alchemize::ALCHEMIZE_PLUS,
        CardName::AllOutAttack => &all_out_attack::ALL_OUT_ATTACK_PLUS,
        CardName::Backflip => &backflip::BACKFLIP_PLUS,
        CardName::Backstab => &backstab::BACKSTAB_PLUS,
        CardName::BandageUp => &bandage_up::BANDAGE_UP_PLUS,
        CardName::Bane => &bane::BANE_PLUS,
        CardName::BladeDance => &blade_dance::BLADE_DANCE_PLUS,
        CardName::Blind => &blind::BLIND_PLUS,
        CardName::Blur => &blur::BLUR_PLUS,
        CardName::BouncingFlask => &bouncing_flask::BOUNCING_FLASK_PLUS,
        CardName::BulletTime => &bullet_time::BULLET_TIME_PLUS,
        CardName::Burn => &burn::BURN_UPGRADED,
        CardName::Burst => &burst::BURST_PLUS,
        CardName::CalculatedGamble => &calculated_gamble::CALCULATED_GAMBLE_PLUS,
        CardName::Caltrops => &caltrops::CALTROPS_PLUS,
        CardName::Catalyst => &catalyst::CATALYST_PLUS,
        CardName::Choke => &choke::CHOKE_PLUS,
        CardName::CloakAndDagger => &cloak_and_dagger::CLOAK_AND_DAGGER_PLUS,
        CardName::Concentrate => &concentrate::CONCENTRATE_PLUS,
        CardName::CorpseExplosion => &corpse_explosion::CORPSE_EXPLOSION_PLUS,
        CardName::CripplingPoison => &crippling_poison::CRIPPLING_POISON_PLUS,
        CardName::DaggerSpray => &dagger_spray::DAGGER_SPRAY_PLUS,
        CardName::DaggerThrow => &dagger_throw::DAGGER_THROW_PLUS,
        CardName::Dash => &dash::DASH_PLUS,
        CardName::DeadlyPoison => &deadly_poison::DEADLY_POISON_PLUS,
        CardName::DeepBreath => &deep_breath::DEEP_BREATH_PLUS,
        CardName::Defend => &defend::DEFEND_PLUS,
        CardName::Deflect => &deflect::DEFLECT_PLUS,
        CardName::DieDieDie => &die_die_die::DIE_DIE_DIE_PLUS,
        CardName::Distraction => &distraction::DISTRACTION_PLUS,
        CardName::DodgeAndRoll => &dodge_and_roll::DODGE_AND_ROLL_PLUS,
        CardName::Doppelganger => &doppelganger::DOPPELGANGER_PLUS,
        CardName::EndlessAgony => &endless_agony::ENDLESS_AGONY_PLUS,
        CardName::Envenom => &envenom::ENVENOM_PLUS,
        CardName::EscapePlan => &escape_plan::ESCAPE_PLAN_PLUS,
        CardName::Eviscerate => &eviscerate::EVISCERATE_PLUS,
        CardName::Expertise => &expertise::EXPERTISE_PLUS,
        CardName::Finesse => &finesse::FINESSE_PLUS,
        CardName::Finisher => &finisher::FINISHER_PLUS,
        CardName::FlashOfSteel => &flash_of_steel::FLASH_OF_STEEL_PLUS,
        CardName::Flechettes => &flechettes::FLECHETTES_PLUS,
        CardName::FlyingKnee => &flying_knee::FLYING_KNEE_PLUS,
        CardName::Footwork => &footwork::FOOTWORK_PLUS,
        CardName::GlassKnife => &glass_knife::GLASS_KNIFE_PLUS,
        CardName::GoodInstincts => &good_instincts::GOOD_INSTINCTS_PLUS,
        CardName::GrandFinale => &grand_finale::GRAND_FINALE_PLUS,
        CardName::HeelHook => &heel_hook::HEEL_HOOK_PLUS,
        CardName::InfiniteBlades => &infinite_blades::INFINITE_BLADES_PLUS,
        CardName::LegSweep => &leg_sweep::LEG_SWEEP_PLUS,
        CardName::Malaise => &malaise::MALAISE_PLUS,
        CardName::MasterOfStrategy => &master_of_strategy::MASTER_OF_STRATEGY_PLUS,
        CardName::MasterfulStab => &masterful_stab::MASTERFUL_STAB_PLUS,
        CardName::MindBlast => &mind_blast::MIND_BLAST_PLUS,
        CardName::Neutralize => &neutralize::NEUTRALIZE_PLUS,
        CardName::Nightmare => &nightmare::NIGHTMARE_PLUS,
        CardName::NoxiousFumes => &noxious_fumes::NOXIOUS_FUMES_PLUS,
        CardName::Outmaneuver => &outmaneuver::OUTMANEUVER_PLUS,
        CardName::PhantasmalKiller => &phantasmal_killer::PHANTASMAL_KILLER_PLUS,
        CardName::PiercingWail => &piercing_wail::PIERCING_WAIL_PLUS,
        CardName::PoisonedStab => &poisoned_stab::POISONED_STAB_PLUS,
        CardName::Predator => &predator::PREDATOR_PLUS,
        CardName::Prepared => &prepared::PREPARED_PLUS,
        CardName::QuickSlash => &quick_slash::QUICK_SLASH_PLUS,
        CardName::Reflex => &reflex::REFLEX_PLUS,
        CardName::RiddleWithHoles => &riddle_with_holes::RIDDLE_WITH_HOLES_PLUS,
        CardName::Setup => &setup::SETUP_PLUS,
        CardName::Shiv => &shiv::SHIV_PLUS,
        CardName::Skewer => &skewer::SKEWER_PLUS,
        CardName::Slice => &slice::SLICE_PLUS,
        CardName::SneakyStrike => &sneaky_strike::SNEAKY_STRIKE_PLUS,
        CardName::StormOfSteel => &storm_of_steel::STORM_OF_STEEL_PLUS,
        CardName::Strike => &strike::STRIKE_PLUS,
        CardName::SuckerPunch => &sucker_punch::SUCKER_PUNCH_PLUS,
        CardName::Survivor => &survivor::SURVIVOR_PLUS,
        CardName::SwiftStrike => &swift_strike::SWIFT_STRIKE_PLUS,
        CardName::Tactician => &tactician::TACTICIAN_PLUS,
        CardName::Terror => &terror::TERROR_PLUS,
        CardName::ToolsOfTheTrade => &tools_of_the_trade::TOOLS_OF_THE_TRADE_PLUS,
        CardName::Unload => &unload::UNLOAD_PLUS,
        CardName::WellLaidPlans => &well_laid_plans::WELL_LAID_PLANS_PLUS,
        CardName::WraithForm => &wraith_form::WRAITH_FORM_PLUS,
        CardName::Apparition => &apparition::APPARITION_PLUS,
        CardName::Bite => &bite::BITE_PLUS,
        CardName::DarkShackles => &dark_shackles::DARK_SHACKLES_PLUS,
        CardName::DramaticEntrance => &dramatic_entrance::DRAMATIC_ENTRANCE_PLUS,
        CardName::Jax => &jax::JAX_PLUS,
        CardName::Panacea => &panacea::PANACEA_PLUS,
        CardName::Trip => &trip::TRIP_PLUS,
        CardName::Apotheosis => &apotheosis::APOTHEOSIS_PLUS,
        CardName::Chrysalis => &chrysalis::CHRYSALIS_PLUS,
        CardName::Discovery => &discovery::DISCOVERY_PLUS,
        CardName::Enlightenment => &enlightenment::ENLIGHTENMENT_PLUS,
        CardName::HandOfGreed => &hand_of_greed::HAND_OF_GREED_PLUS,
        CardName::Impatience => &impatience::IMPATIENCE_PLUS,
        CardName::JackOfAllTrades => &jack_of_all_trades::JACK_OF_ALL_TRADES_PLUS,
        CardName::Madness => &madness::MADNESS_PLUS,
        CardName::Magnetism => &magnetism::MAGNETISM_PLUS,
        CardName::Metamorphosis => &metamorphosis::METAMORPHOSIS_PLUS,
        CardName::Panache => &panache::PANACHE_PLUS,
        CardName::PanicButton => &panic_button::PANIC_BUTTON_PLUS,
        CardName::SadisticNature => &sadistic_nature::SADISTIC_NATURE_PLUS,
        CardName::ThinkingAhead => &thinking_ahead::THINKING_AHEAD_PLUS,
        CardName::Transmutation => &transmutation::TRANSMUTATION_PLUS,
        CardName::Forethought => &forethought::FORETHOUGHT_PLUS,
        CardName::Mayhem => &mayhem::MAYHEM_PLUS,
        CardName::Purity => &purity::PURITY_PLUS,
        CardName::SecretTechnique => &secret_technique::SECRET_TECHNIQUE_PLUS,
        CardName::SecretWeapon => &secret_weapon::SECRET_WEAPON_PLUS,
        CardName::TheBomb => &the_bomb::THE_BOMB_PLUS,
        CardName::Violence => &violence::VIOLENCE_PLUS,
        CardName::RitualDagger => &ritual_dagger::RITUAL_DAGGER_PLUS,
        CardName::Dazed
        | CardName::Wound
        | CardName::Slimed
        | CardName::AscendersBane
        | CardName::CurseOfTheBell
        | CardName::Regret
        | CardName::Pain
        | CardName::Doubt
        | CardName::Decay
        | CardName::Injury
        | CardName::Shame
        | CardName::Writhe
        | CardName::Parasite
        | CardName::Normality
        | CardName::Necronomicurse => CARD_BY_NAME[name as usize],
    }
}

pub const ALL_CARDS: &[&'static CardTemplate] = &[
    &a_thousand_cuts::A_THOUSAND_CUTS,
    &accuracy::ACCURACY,
    &acrobatics::ACROBATICS,
    &adrenaline::ADRENALINE,
    &after_image::AFTER_IMAGE,
    &alchemize::ALCHEMIZE,
    &all_out_attack::ALL_OUT_ATTACK,
    &backflip::BACKFLIP,
    &backstab::BACKSTAB,
    &bandage_up::BANDAGE_UP,
    &bane::BANE,
    &blade_dance::BLADE_DANCE,
    &blind::BLIND,
    &blur::BLUR,
    &bouncing_flask::BOUNCING_FLASK,
    &bullet_time::BULLET_TIME,
    &burn::BURN,
    &burst::BURST,
    &calculated_gamble::CALCULATED_GAMBLE,
    &caltrops::CALTROPS,
    &catalyst::CATALYST,
    &choke::CHOKE,
    &cloak_and_dagger::CLOAK_AND_DAGGER,
    &concentrate::CONCENTRATE,
    &corpse_explosion::CORPSE_EXPLOSION,
    &crippling_poison::CRIPPLING_POISON,
    &dagger_spray::DAGGER_SPRAY,
    &dagger_throw::DAGGER_THROW,
    &dash::DASH,
    &dazed::DAZED,
    &deadly_poison::DEADLY_POISON,
    &deep_breath::DEEP_BREATH,
    &defend::DEFEND,
    &deflect::DEFLECT,
    &die_die_die::DIE_DIE_DIE,
    &distraction::DISTRACTION,
    &dodge_and_roll::DODGE_AND_ROLL,
    &doppelganger::DOPPELGANGER,
    &endless_agony::ENDLESS_AGONY,
    &envenom::ENVENOM,
    &escape_plan::ESCAPE_PLAN,
    &eviscerate::EVISCERATE,
    &expertise::EXPERTISE,
    &finesse::FINESSE,
    &finisher::FINISHER,
    &flash_of_steel::FLASH_OF_STEEL,
    &flechettes::FLECHETTES,
    &flying_knee::FLYING_KNEE,
    &footwork::FOOTWORK,
    &glass_knife::GLASS_KNIFE,
    &good_instincts::GOOD_INSTINCTS,
    &grand_finale::GRAND_FINALE,
    &heel_hook::HEEL_HOOK,
    &infinite_blades::INFINITE_BLADES,
    &leg_sweep::LEG_SWEEP,
    &malaise::MALAISE,
    &master_of_strategy::MASTER_OF_STRATEGY,
    &masterful_stab::MASTERFUL_STAB,
    &mind_blast::MIND_BLAST,
    &neutralize::NEUTRALIZE,
    &nightmare::NIGHTMARE,
    &noxious_fumes::NOXIOUS_FUMES,
    &outmaneuver::OUTMANEUVER,
    &phantasmal_killer::PHANTASMAL_KILLER,
    &piercing_wail::PIERCING_WAIL,
    &poisoned_stab::POISONED_STAB,
    &predator::PREDATOR,
    &prepared::PREPARED,
    &quick_slash::QUICK_SLASH,
    &reflex::REFLEX,
    &riddle_with_holes::RIDDLE_WITH_HOLES,
    &setup::SETUP,
    &shiv::SHIV,
    &skewer::SKEWER,
    &slice::SLICE,
    &slimed::SLIMED,
    &sneaky_strike::SNEAKY_STRIKE,
    &storm_of_steel::STORM_OF_STEEL,
    &strike::STRIKE,
    &sucker_punch::SUCKER_PUNCH,
    &survivor::SURVIVOR,
    &swift_strike::SWIFT_STRIKE,
    &tactician::TACTICIAN,
    &terror::TERROR,
    &tools_of_the_trade::TOOLS_OF_THE_TRADE,
    &unload::UNLOAD,
    &well_laid_plans::WELL_LAID_PLANS,
    &wraith_form::WRAITH_FORM,
    &ascenders_bane::ASCENDERS_BANE,
    &regret::REGRET,
    &pain::PAIN,
    &doubt::DOUBT,
    &decay::DECAY,
    &injury::INJURY,
    &shame::SHAME,
    &writhe::WRITHE,
    &parasite::PARASITE,
    &normality::NORMALITY,
    &apparition::APPARITION,
    &bite::BITE,
    &dark_shackles::DARK_SHACKLES,
    &dramatic_entrance::DRAMATIC_ENTRANCE,
    &jax::JAX,
    &panacea::PANACEA,
    &trip::TRIP,
    &apotheosis::APOTHEOSIS,
    &chrysalis::CHRYSALIS,
    &discovery::DISCOVERY,
    &enlightenment::ENLIGHTENMENT,
    &hand_of_greed::HAND_OF_GREED,
    &impatience::IMPATIENCE,
    &jack_of_all_trades::JACK_OF_ALL_TRADES,
    &madness::MADNESS,
    &magnetism::MAGNETISM,
    &metamorphosis::METAMORPHOSIS,
    &panache::PANACHE,
    &panic_button::PANIC_BUTTON,
    &sadistic_nature::SADISTIC_NATURE,
    &thinking_ahead::THINKING_AHEAD,
    &transmutation::TRANSMUTATION,
    &forethought::FORETHOUGHT,
    &mayhem::MAYHEM,
    &purity::PURITY,
    &secret_technique::SECRET_TECHNIQUE,
    &secret_weapon::SECRET_WEAPON,
    &the_bomb::THE_BOMB,
    &violence::VIOLENCE,
    &curse_of_the_bell::CURSE_OF_THE_BELL,
    &wound::WOUND,
    &ritual_dagger::RITUAL_DAGGER,
    &necronomicurse::NECRONOMICURSE,
];
// Assert all Cards are included without duplicates
const _: () = assert!(ALL_CARDS.len() == CardName::COUNT);
const _: () = {
    let mut seen = [false; CardName::COUNT];
    let mut idx_all = 0;
    while idx_all < ALL_CARDS.len() {
        let idx_card = ALL_CARDS[idx_all].name as usize;
        assert!(!seen[idx_card], "ALL_CARDS contains a duplicate CardName");
        seen[idx_card] = true;
        idx_all += 1;
    }
};

const fn card_rarity_eq(lhs: CardRarity, rhs: CardRarity) -> bool {
    lhs as u8 == rhs as u8
}

const fn card_color_eq(lhs: CardColor, rhs: CardColor) -> bool {
    lhs as u8 == rhs as u8
}

// Reward pools: rewardable kind, minus the two Curse cards that are Neow/event-only
const fn in_pool(card: &CardTemplate, rarity: CardRarity, color: CardColor) -> bool {
    card_rarity_eq(card.rarity, rarity)
        && card_color_eq(card.color, color)
        && matches!(
            card.kind,
            CardKind::Attack | CardKind::Skill | CardKind::Power | CardKind::Curse
        )
        && !card_name_never_obtainable(card.name)
}

const fn count_pool(rarity: CardRarity, color: CardColor) -> usize {
    let mut count = 0;
    let mut idx = 0;
    while idx < ALL_CARDS.len() {
        if in_pool(ALL_CARDS[idx], rarity, color) {
            count += 1;
        }
        idx += 1;
    }
    count
}

const fn build_pool<const N: usize>(rarity: CardRarity, color: CardColor) -> [CardName; N] {
    let mut buf = [CardName::Strike; N];
    let mut idx_pool = 0;
    let mut idx_all = 0;
    while idx_all < ALL_CARDS.len() {
        if in_pool(ALL_CARDS[idx_all], rarity, color) {
            buf[idx_pool] = ALL_CARDS[idx_all].name;
            idx_pool += 1;
        }
        idx_all += 1;
    }
    buf
}

// Pool sizes by (rarity, color) — Green
const NUM_COMMON_GREEN: usize = count_pool(CardRarity::Common, CardColor::Green);
const NUM_UNCOMMON_GREEN: usize = count_pool(CardRarity::Uncommon, CardColor::Green);
const NUM_RARE_GREEN: usize = count_pool(CardRarity::Rare, CardColor::Green);

// Colorless
const NUM_UNCOMMON_COLORLESS: usize = count_pool(CardRarity::Uncommon, CardColor::Colorless);
const NUM_RARE_COLORLESS: usize = count_pool(CardRarity::Rare, CardColor::Colorless);

// Curse
const NUM_CURSE: usize = count_pool(CardRarity::Curse, CardColor::Curse);

// Pools by (rarity, color) — Green
pub const POOL_COMMON_GREEN_CARD: &[CardName] =
    &build_pool::<NUM_COMMON_GREEN>(CardRarity::Common, CardColor::Green);
pub const POOL_UNCOMMON_GREEN_CARD: &[CardName] =
    &build_pool::<NUM_UNCOMMON_GREEN>(CardRarity::Uncommon, CardColor::Green);
pub const POOL_RARE_GREEN_CARD: &[CardName] =
    &build_pool::<NUM_RARE_GREEN>(CardRarity::Rare, CardColor::Green);

// Colorless
pub const POOL_UNCOMMON_COLORLESS_CARD: &[CardName] =
    &build_pool::<NUM_UNCOMMON_COLORLESS>(CardRarity::Uncommon, CardColor::Colorless);
pub const POOL_RARE_COLORLESS_CARD: &[CardName] =
    &build_pool::<NUM_RARE_COLORLESS>(CardRarity::Rare, CardColor::Colorless);

// Curse
pub const POOL_CURSE_CARD: &[CardName] =
    &build_pool::<NUM_CURSE>(CardRarity::Curse, CardColor::Curse);

// Pick `count` distinct Card names from the full set, filtered by color and (when given) kind/rarity
pub fn get_random_card_names(
    color: CardColor,
    kind: Option<CardKind>,
    rarity: Option<CardRarity>,
    exclude: &[CardName],
    count: usize,
    rng: &mut impl rand::Rng,
) -> Vec<CardName> {
    let mut pool: Vec<&CardTemplate> = ALL_CARDS
        .iter()
        .copied()
        .filter(|card| card.color == color)
        .filter(|card| kind.is_none_or(|card_kind| card.kind == card_kind))
        .filter(|card| rarity.is_none_or(|card_rarity| card.rarity == card_rarity))
        .filter(|card| !exclude.contains(&card.name))
        .collect();

    shuffle(&mut pool, rng);
    pool.truncate(count);
    pool.iter().map(|card| card.name).collect()
}

// Same pick, instanced (base forms)
pub fn get_random_cards(
    color: CardColor,
    kind: Option<CardKind>,
    rarity: Option<CardRarity>,
    exclude: &[CardName],
    count: usize,
    rng: &mut impl rand::Rng,
) -> Vec<Entity> {
    get_random_card_names(color, kind, rarity, exclude, count, rng)
        .into_iter()
        .map(|name| get_card(name, false))
        .collect()
}

#[allow(clippy::too_many_arguments)]
pub struct CardTemplate {
    pub name: CardName,
    pub kind: CardKind,
    pub color: CardColor,
    pub rarity: CardRarity,
    pub cost: u8,
    pub cost_kind: CardCostKind,
    pub upgraded: bool,
    pub exhaust: bool,
    pub ethereal: bool,
    pub innate: bool,
    pub play_restriction: PlayRestriction,
    pub effects: [Effect; MAX_EFFECTS_PER_CARD],
    pub effects_len: u8,
    pub on_discard_effects: &'static [Effect],
    pub effects_on_draw: &'static [Effect],
}

pub const fn make_card_template(
    name: CardName,
    kind: CardKind,
    color: CardColor,
    rarity: CardRarity,
    cost: u8,
    cost_kind: CardCostKind,
    upgraded: bool,
    exhaust: bool,
    ethereal: bool,
    innate: bool,
    effects: &[Effect],
    on_discard_effects: &'static [Effect],
    on_draw_effects: &'static [Effect],
    play_restriction: PlayRestriction,
) -> CardTemplate {
    assert!(
        effects.len() <= MAX_EFFECTS_PER_CARD,
        "card effects exceed MAX_EFFECTS_PER_CARD",
    );
    let mut arr = [EFFECT_ZERO; MAX_EFFECTS_PER_CARD];
    let mut idx = 0;
    while idx < effects.len() {
        arr[idx] = effects[idx];
        idx += 1;
    }
    CardTemplate {
        name,
        kind,
        color,
        rarity,
        cost,
        cost_kind,
        upgraded,
        exhaust,
        ethereal,
        innate,
        play_restriction,
        effects: arr,
        effects_len: effects.len() as u8,
        on_discard_effects,
        effects_on_draw: on_draw_effects,
    }
}

pub const fn instance_card_from_template(template: &CardTemplate) -> Entity {
    Entity {
        kind: EntityKind::Card,
        card_name: template.name,
        card_kind: template.kind,
        card_color: template.color,
        card_rarity: template.rarity,
        card_cost: template.cost,
        card_cost_kind: template.cost_kind,
        card_upgraded: template.upgraded,
        card_exhaust: template.exhaust,
        card_ethereal: template.ethereal,
        card_innate: template.innate,
        card_play_restriction: template.play_restriction,
        card_effects: template.effects,
        card_effects_len: template.effects_len,
        card_on_discard_effects: template.on_discard_effects,
        card_effects_on_draw: template.effects_on_draw,
        ..ENTITY_ZERO
    }
}
