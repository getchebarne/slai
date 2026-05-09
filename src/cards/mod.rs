mod a_thousand_cuts;
mod accuracy;
mod acrobatics;
mod adrenaline;
mod after_image;
mod all_out_attack;
mod backflip;
mod backstab;
mod bane;
mod blade_dance;
mod blur;
mod bouncing_flask;
mod bullet_time;
mod burn;
mod burst;
mod calculated_gamble;
mod caltrops;
mod catalyst;
mod choke;
mod cloak_and_dagger;
mod concentrate;
mod corpse_explosion;
mod crippling_poison;
mod dagger_spray;
mod dagger_throw;
mod dash;
mod dazed;
mod deadly_poison;
mod defend;
mod deflect;
mod die_die_die;
mod distraction;
mod dodge_and_roll;
mod doppelganger;
mod endless_agony;
mod envenom;
mod escape_plan;
mod eviscerate;
mod expertise;
mod finisher;
mod flechettes;
mod flying_knee;
mod footwork;
mod glass_knife;
mod grand_finale;
mod heel_hook;
mod infinite_blades;
mod leg_sweep;
mod malaise;
mod masterful_stab;
mod neutralize;
mod nightmare;
mod noxious_fumes;
mod outmaneuver;
mod phantasmal_killer;
mod piercing_wail;
mod poisoned_stab;
mod predator;
mod prepared;
mod quick_slash;
mod reflex;
mod riddle_with_holes;
mod setup;
mod shiv;
mod skewer;
mod slice;
mod slimed;
mod sneaky_strike;
mod storm_of_steel;
mod strike;
mod sucker_punch;
mod survivor;
mod tactician;
mod terror;
mod tools_of_the_trade;
mod unload;
mod well_laid_plans;
mod wraith_form;

use crate::entity::Entity;
use crate::types::{CardKind, CardName, CardRarity};
use strum::EnumCount;

pub fn get_card(name: CardName, upgraded: bool) -> Entity {
    match (name, upgraded) {
        (CardName::AThousandCuts, false) => a_thousand_cuts::A_THOUSAND_CUTS,
        (CardName::AThousandCuts, true) => a_thousand_cuts::A_THOUSAND_CUTS_PLUS,
        (CardName::Accuracy, false) => accuracy::ACCURACY,
        (CardName::Accuracy, true) => accuracy::ACCURACY_PLUS,
        (CardName::Acrobatics, false) => acrobatics::ACROBATICS,
        (CardName::Acrobatics, true) => acrobatics::ACROBATICS_PLUS,
        (CardName::Adrenaline, false) => adrenaline::ADRENALINE,
        (CardName::Adrenaline, true) => adrenaline::ADRENALINE_PLUS,
        (CardName::AfterImage, false) => after_image::AFTER_IMAGE,
        (CardName::AfterImage, true) => after_image::AFTER_IMAGE_PLUS,
        (CardName::AllOutAttack, false) => all_out_attack::ALL_OUT_ATTACK,
        (CardName::AllOutAttack, true) => all_out_attack::ALL_OUT_ATTACK_PLUS,
        (CardName::Backflip, false) => backflip::BACKFLIP,
        (CardName::Backflip, true) => backflip::BACKFLIP_PLUS,
        (CardName::Backstab, false) => backstab::BACKSTAB,
        (CardName::Backstab, true) => backstab::BACKSTAB_PLUS,
        (CardName::Bane, false) => bane::BANE,
        (CardName::Bane, true) => bane::BANE_PLUS,
        (CardName::BladeDance, false) => blade_dance::BLADE_DANCE,
        (CardName::BladeDance, true) => blade_dance::BLADE_DANCE_PLUS,
        (CardName::Blur, false) => blur::BLUR,
        (CardName::Blur, true) => blur::BLUR_PLUS,
        (CardName::BouncingFlask, false) => bouncing_flask::BOUNCING_FLASK,
        (CardName::BouncingFlask, true) => bouncing_flask::BOUNCING_FLASK_PLUS,
        (CardName::BulletTime, false) => bullet_time::BULLET_TIME,
        (CardName::BulletTime, true) => bullet_time::BULLET_TIME_PLUS,
        (CardName::Burn, false) => burn::BURN,
        (CardName::Burn, true) => burn::BURN_UPGRADED,
        (CardName::Burst, false) => burst::BURST,
        (CardName::Burst, true) => burst::BURST_PLUS,
        (CardName::CalculatedGamble, false) => calculated_gamble::CALCULATED_GAMBLE,
        (CardName::CalculatedGamble, true) => calculated_gamble::CALCULATED_GAMBLE_PLUS,
        (CardName::Caltrops, false) => caltrops::CALTROPS,
        (CardName::Caltrops, true) => caltrops::CALTROPS_PLUS,
        (CardName::Catalyst, false) => catalyst::CATALYST,
        (CardName::Catalyst, true) => catalyst::CATALYST_PLUS,
        (CardName::Choke, false) => choke::CHOKE,
        (CardName::Choke, true) => choke::CHOKE_PLUS,
        (CardName::CloakAndDagger, false) => cloak_and_dagger::CLOAK_AND_DAGGER,
        (CardName::CloakAndDagger, true) => cloak_and_dagger::CLOAK_AND_DAGGER_PLUS,
        (CardName::Concentrate, false) => concentrate::CONCENTRATE,
        (CardName::Concentrate, true) => concentrate::CONCENTRATE_PLUS,
        (CardName::CorpseExplosion, false) => corpse_explosion::CORPSE_EXPLOSION,
        (CardName::CorpseExplosion, true) => corpse_explosion::CORPSE_EXPLOSION_PLUS,
        (CardName::CripplingPoison, false) => crippling_poison::CRIPPLING_POISON,
        (CardName::CripplingPoison, true) => crippling_poison::CRIPPLING_POISON_PLUS,
        (CardName::DaggerSpray, false) => dagger_spray::DAGGER_SPRAY,
        (CardName::DaggerSpray, true) => dagger_spray::DAGGER_SPRAY_PLUS,
        (CardName::DaggerThrow, false) => dagger_throw::DAGGER_THROW,
        (CardName::DaggerThrow, true) => dagger_throw::DAGGER_THROW_PLUS,
        (CardName::Dash, false) => dash::DASH,
        (CardName::Dash, true) => dash::DASH_PLUS,
        (CardName::Dazed, _) => dazed::DAZED, // No upgraded variant
        (CardName::DeadlyPoison, false) => deadly_poison::DEADLY_POISON,
        (CardName::DeadlyPoison, true) => deadly_poison::DEADLY_POISON_PLUS,
        (CardName::Defend, false) => defend::DEFEND,
        (CardName::Defend, true) => defend::DEFEND_PLUS,
        (CardName::Deflect, false) => deflect::DEFLECT,
        (CardName::Deflect, true) => deflect::DEFLECT_PLUS,
        (CardName::DieDieDie, false) => die_die_die::DIE_DIE_DIE,
        (CardName::DieDieDie, true) => die_die_die::DIE_DIE_DIE_PLUS,
        (CardName::Distraction, false) => distraction::DISTRACTION,
        (CardName::Distraction, true) => distraction::DISTRACTION_PLUS,
        (CardName::DodgeAndRoll, false) => dodge_and_roll::DODGE_AND_ROLL,
        (CardName::DodgeAndRoll, true) => dodge_and_roll::DODGE_AND_ROLL_PLUS,
        (CardName::Doppelganger, false) => doppelganger::DOPPELGANGER,
        (CardName::Doppelganger, true) => doppelganger::DOPPELGANGER_PLUS,
        (CardName::EndlessAgony, false) => endless_agony::ENDLESS_AGONY,
        (CardName::EndlessAgony, true) => endless_agony::ENDLESS_AGONY_PLUS,
        (CardName::Envenom, false) => envenom::ENVENOM,
        (CardName::Envenom, true) => envenom::ENVENOM_PLUS,
        (CardName::EscapePlan, false) => escape_plan::ESCAPE_PLAN,
        (CardName::EscapePlan, true) => escape_plan::ESCAPE_PLAN_PLUS,
        (CardName::Eviscerate, false) => eviscerate::EVISCERATE,
        (CardName::Eviscerate, true) => eviscerate::EVISCERATE_PLUS,
        (CardName::Expertise, false) => expertise::EXPERTISE,
        (CardName::Expertise, true) => expertise::EXPERTISE_PLUS,
        (CardName::Finisher, false) => finisher::FINISHER,
        (CardName::Finisher, true) => finisher::FINISHER_PLUS,
        (CardName::Flechettes, false) => flechettes::FLECHETTES,
        (CardName::Flechettes, true) => flechettes::FLECHETTES_PLUS,
        (CardName::FlyingKnee, false) => flying_knee::FLYING_KNEE,
        (CardName::FlyingKnee, true) => flying_knee::FLYING_KNEE_PLUS,
        (CardName::Footwork, false) => footwork::FOOTWORK,
        (CardName::Footwork, true) => footwork::FOOTWORK_PLUS,
        (CardName::GlassKnife, false) => glass_knife::GLASS_KNIFE,
        (CardName::GlassKnife, true) => glass_knife::GLASS_KNIFE_PLUS,
        (CardName::GrandFinale, false) => grand_finale::GRAND_FINALE,
        (CardName::GrandFinale, true) => grand_finale::GRAND_FINALE_PLUS,
        (CardName::HeelHook, false) => heel_hook::HEEL_HOOK,
        (CardName::HeelHook, true) => heel_hook::HEEL_HOOK_PLUS,
        (CardName::InfiniteBlades, false) => infinite_blades::INFINITE_BLADES,
        (CardName::InfiniteBlades, true) => infinite_blades::INFINITE_BLADES_PLUS,
        (CardName::LegSweep, false) => leg_sweep::LEG_SWEEP,
        (CardName::LegSweep, true) => leg_sweep::LEG_SWEEP_PLUS,
        (CardName::Malaise, false) => malaise::MALAISE,
        (CardName::Malaise, true) => malaise::MALAISE_PLUS,
        (CardName::MasterfulStab, false) => masterful_stab::MASTERFUL_STAB,
        (CardName::MasterfulStab, true) => masterful_stab::MASTERFUL_STAB_PLUS,
        (CardName::Neutralize, false) => neutralize::NEUTRALIZE,
        (CardName::Neutralize, true) => neutralize::NEUTRALIZE_PLUS,
        (CardName::Nightmare, false) => nightmare::NIGHTMARE,
        (CardName::Nightmare, true) => nightmare::NIGHTMARE_PLUS,
        (CardName::NoxiousFumes, false) => noxious_fumes::NOXIOUS_FUMES,
        (CardName::NoxiousFumes, true) => noxious_fumes::NOXIOUS_FUMES_PLUS,
        (CardName::Outmaneuver, false) => outmaneuver::OUTMANEUVER,
        (CardName::Outmaneuver, true) => outmaneuver::OUTMANEUVER_PLUS,
        (CardName::PhantasmalKiller, false) => phantasmal_killer::PHANTASMAL_KILLER,
        (CardName::PhantasmalKiller, true) => phantasmal_killer::PHANTASMAL_KILLER_PLUS,
        (CardName::PiercingWail, false) => piercing_wail::PIERCING_WAIL,
        (CardName::PiercingWail, true) => piercing_wail::PIERCING_WAIL_PLUS,
        (CardName::PoisonedStab, false) => poisoned_stab::POISONED_STAB,
        (CardName::PoisonedStab, true) => poisoned_stab::POISONED_STAB_PLUS,
        (CardName::Predator, false) => predator::PREDATOR,
        (CardName::Predator, true) => predator::PREDATOR_PLUS,
        (CardName::Prepared, false) => prepared::PREPARED,
        (CardName::Prepared, true) => prepared::PREPARED_PLUS,
        (CardName::QuickSlash, false) => quick_slash::QUICK_SLASH,
        (CardName::QuickSlash, true) => quick_slash::QUICK_SLASH_PLUS,
        (CardName::Reflex, false) => reflex::REFLEX,
        (CardName::Reflex, true) => reflex::REFLEX_PLUS,
        (CardName::RiddleWithHoles, false) => riddle_with_holes::RIDDLE_WITH_HOLES,
        (CardName::RiddleWithHoles, true) => riddle_with_holes::RIDDLE_WITH_HOLES_PLUS,
        (CardName::Setup, false) => setup::SETUP,
        (CardName::Setup, true) => setup::SETUP_PLUS,
        (CardName::Shiv, false) => shiv::SHIV,
        (CardName::Shiv, true) => shiv::SHIV_PLUS,
        (CardName::Skewer, false) => skewer::SKEWER,
        (CardName::Skewer, true) => skewer::SKEWER_PLUS,
        (CardName::Slice, false) => slice::SLICE,
        (CardName::Slice, true) => slice::SLICE_PLUS,
        (CardName::Slimed, _) => slimed::SLIMED, // No upgraded variant
        (CardName::SneakyStrike, false) => sneaky_strike::SNEAKY_STRIKE,
        (CardName::SneakyStrike, true) => sneaky_strike::SNEAKY_STRIKE_PLUS,
        (CardName::StormOfSteel, false) => storm_of_steel::STORM_OF_STEEL,
        (CardName::StormOfSteel, true) => storm_of_steel::STORM_OF_STEEL_PLUS,
        (CardName::Strike, false) => strike::STRIKE,
        (CardName::Strike, true) => strike::STRIKE_PLUS,
        (CardName::SuckerPunch, false) => sucker_punch::SUCKER_PUNCH,
        (CardName::SuckerPunch, true) => sucker_punch::SUCKER_PUNCH_PLUS,
        (CardName::Survivor, false) => survivor::SURVIVOR,
        (CardName::Survivor, true) => survivor::SURVIVOR_PLUS,
        (CardName::Tactician, false) => tactician::TACTICIAN,
        (CardName::Tactician, true) => tactician::TACTICIAN_PLUS,
        (CardName::Terror, false) => terror::TERROR,
        (CardName::Terror, true) => terror::TERROR_PLUS,
        (CardName::ToolsOfTheTrade, false) => tools_of_the_trade::TOOLS_OF_THE_TRADE,
        (CardName::ToolsOfTheTrade, true) => tools_of_the_trade::TOOLS_OF_THE_TRADE_PLUS,
        (CardName::Unload, false) => unload::UNLOAD,
        (CardName::Unload, true) => unload::UNLOAD_PLUS,
        (CardName::WellLaidPlans, false) => well_laid_plans::WELL_LAID_PLANS,
        (CardName::WellLaidPlans, true) => well_laid_plans::WELL_LAID_PLANS_PLUS,
        (CardName::WraithForm, false) => wraith_form::WRAITH_FORM,
        (CardName::WraithForm, true) => wraith_form::WRAITH_FORM_PLUS,
    }
}

// Single source of truth for "what cards exist". Compile-time checks below
// guarantee this list contains every CardName variant exactly once.
pub const ALL_CARDS: &[CardName] = &[
    CardName::AThousandCuts,
    CardName::Accuracy,
    CardName::Acrobatics,
    CardName::Adrenaline,
    CardName::AfterImage,
    CardName::AllOutAttack,
    CardName::Backflip,
    CardName::Backstab,
    CardName::Bane,
    CardName::BladeDance,
    CardName::Blur,
    CardName::BouncingFlask,
    CardName::BulletTime,
    CardName::Burn,
    CardName::Burst,
    CardName::CalculatedGamble,
    CardName::Caltrops,
    CardName::Catalyst,
    CardName::Choke,
    CardName::CloakAndDagger,
    CardName::Concentrate,
    CardName::CorpseExplosion,
    CardName::CripplingPoison,
    CardName::DaggerSpray,
    CardName::DaggerThrow,
    CardName::Dash,
    CardName::Dazed,
    CardName::DeadlyPoison,
    CardName::Defend,
    CardName::Deflect,
    CardName::DieDieDie,
    CardName::Distraction,
    CardName::DodgeAndRoll,
    CardName::Doppelganger,
    CardName::EndlessAgony,
    CardName::Envenom,
    CardName::EscapePlan,
    CardName::Eviscerate,
    CardName::Expertise,
    CardName::Finisher,
    CardName::Flechettes,
    CardName::FlyingKnee,
    CardName::Footwork,
    CardName::GlassKnife,
    CardName::GrandFinale,
    CardName::HeelHook,
    CardName::InfiniteBlades,
    CardName::LegSweep,
    CardName::Malaise,
    CardName::MasterfulStab,
    CardName::Neutralize,
    CardName::Nightmare,
    CardName::NoxiousFumes,
    CardName::Outmaneuver,
    CardName::PhantasmalKiller,
    CardName::PiercingWail,
    CardName::PoisonedStab,
    CardName::Predator,
    CardName::Prepared,
    CardName::QuickSlash,
    CardName::Reflex,
    CardName::RiddleWithHoles,
    CardName::Setup,
    CardName::Shiv,
    CardName::Skewer,
    CardName::Slice,
    CardName::Slimed,
    CardName::SneakyStrike,
    CardName::StormOfSteel,
    CardName::Strike,
    CardName::SuckerPunch,
    CardName::Survivor,
    CardName::Tactician,
    CardName::Terror,
    CardName::ToolsOfTheTrade,
    CardName::Unload,
    CardName::WellLaidPlans,
    CardName::WraithForm,
];

const fn card_rarity(name: CardName) -> CardRarity {
    match name {
        CardName::AThousandCuts => CardRarity::Rare,
        CardName::Accuracy => CardRarity::Uncommon,
        CardName::Acrobatics => CardRarity::Common,
        CardName::Adrenaline => CardRarity::Rare,
        CardName::AfterImage => CardRarity::Rare,
        CardName::AllOutAttack => CardRarity::Uncommon,
        CardName::Backflip => CardRarity::Common,
        CardName::Backstab => CardRarity::Uncommon,
        CardName::Bane => CardRarity::Common,
        CardName::BladeDance => CardRarity::Common,
        CardName::Blur => CardRarity::Uncommon,
        CardName::BouncingFlask => CardRarity::Uncommon,
        CardName::BulletTime => CardRarity::Rare,
        CardName::Burn => CardRarity::Common,
        CardName::Burst => CardRarity::Rare,
        CardName::CalculatedGamble => CardRarity::Uncommon,
        CardName::Caltrops => CardRarity::Uncommon,
        CardName::Catalyst => CardRarity::Uncommon,
        CardName::Choke => CardRarity::Uncommon,
        CardName::CloakAndDagger => CardRarity::Common,
        CardName::Concentrate => CardRarity::Uncommon,
        CardName::CorpseExplosion => CardRarity::Rare,
        CardName::CripplingPoison => CardRarity::Uncommon,
        CardName::DaggerSpray => CardRarity::Common,
        CardName::DaggerThrow => CardRarity::Common,
        CardName::Dash => CardRarity::Uncommon,
        CardName::Dazed => CardRarity::Special,
        CardName::DeadlyPoison => CardRarity::Common,
        CardName::Defend => CardRarity::Basic,
        CardName::Deflect => CardRarity::Common,
        CardName::DieDieDie => CardRarity::Rare,
        CardName::Distraction => CardRarity::Uncommon,
        CardName::DodgeAndRoll => CardRarity::Common,
        CardName::Doppelganger => CardRarity::Rare,
        CardName::EndlessAgony => CardRarity::Uncommon,
        CardName::Envenom => CardRarity::Rare,
        CardName::EscapePlan => CardRarity::Uncommon,
        CardName::Eviscerate => CardRarity::Uncommon,
        CardName::Expertise => CardRarity::Uncommon,
        CardName::Finisher => CardRarity::Uncommon,
        CardName::Flechettes => CardRarity::Uncommon,
        CardName::FlyingKnee => CardRarity::Common,
        CardName::Footwork => CardRarity::Uncommon,
        CardName::GlassKnife => CardRarity::Rare,
        CardName::GrandFinale => CardRarity::Rare,
        CardName::HeelHook => CardRarity::Uncommon,
        CardName::InfiniteBlades => CardRarity::Uncommon,
        CardName::LegSweep => CardRarity::Uncommon,
        CardName::Malaise => CardRarity::Rare,
        CardName::MasterfulStab => CardRarity::Uncommon,
        CardName::Neutralize => CardRarity::Basic,
        CardName::Nightmare => CardRarity::Rare,
        CardName::NoxiousFumes => CardRarity::Uncommon,
        CardName::Outmaneuver => CardRarity::Common,
        CardName::PhantasmalKiller => CardRarity::Rare,
        CardName::PiercingWail => CardRarity::Common,
        CardName::PoisonedStab => CardRarity::Common,
        CardName::Predator => CardRarity::Uncommon,
        CardName::Prepared => CardRarity::Common,
        CardName::QuickSlash => CardRarity::Common,
        CardName::Reflex => CardRarity::Uncommon,
        CardName::RiddleWithHoles => CardRarity::Uncommon,
        CardName::Setup => CardRarity::Uncommon,
        CardName::Shiv => CardRarity::Special,
        CardName::Skewer => CardRarity::Uncommon,
        CardName::Slice => CardRarity::Common,
        CardName::Slimed => CardRarity::Special,
        CardName::SneakyStrike => CardRarity::Common,
        CardName::StormOfSteel => CardRarity::Rare,
        CardName::Strike => CardRarity::Basic,
        CardName::SuckerPunch => CardRarity::Common,
        CardName::Survivor => CardRarity::Basic,
        CardName::Tactician => CardRarity::Uncommon,
        CardName::Terror => CardRarity::Uncommon,
        CardName::ToolsOfTheTrade => CardRarity::Rare,
        CardName::Unload => CardRarity::Rare,
        CardName::WellLaidPlans => CardRarity::Uncommon,
        CardName::WraithForm => CardRarity::Rare,
    }
}

const fn card_kind(name: CardName) -> CardKind {
    match name {
        CardName::AThousandCuts => CardKind::Power,
        CardName::Accuracy => CardKind::Power,
        CardName::Acrobatics => CardKind::Skill,
        CardName::Adrenaline => CardKind::Skill,
        CardName::AfterImage => CardKind::Power,
        CardName::AllOutAttack => CardKind::Attack,
        CardName::Backflip => CardKind::Skill,
        CardName::Backstab => CardKind::Attack,
        CardName::Bane => CardKind::Attack,
        CardName::BladeDance => CardKind::Skill,
        CardName::Blur => CardKind::Skill,
        CardName::BouncingFlask => CardKind::Skill,
        CardName::BulletTime => CardKind::Skill,
        CardName::Burn => CardKind::Status,
        CardName::Burst => CardKind::Skill,
        CardName::CalculatedGamble => CardKind::Skill,
        CardName::Caltrops => CardKind::Power,
        CardName::Catalyst => CardKind::Skill,
        CardName::Choke => CardKind::Attack,
        CardName::CloakAndDagger => CardKind::Skill,
        CardName::Concentrate => CardKind::Skill,
        CardName::CorpseExplosion => CardKind::Skill,
        CardName::CripplingPoison => CardKind::Skill,
        CardName::DaggerSpray => CardKind::Attack,
        CardName::DaggerThrow => CardKind::Attack,
        CardName::Dash => CardKind::Attack,
        CardName::Dazed => CardKind::Status,
        CardName::DeadlyPoison => CardKind::Skill,
        CardName::Defend => CardKind::Skill,
        CardName::Deflect => CardKind::Skill,
        CardName::DieDieDie => CardKind::Attack,
        CardName::Distraction => CardKind::Skill,
        CardName::DodgeAndRoll => CardKind::Skill,
        CardName::Doppelganger => CardKind::Skill,
        CardName::EndlessAgony => CardKind::Attack,
        CardName::Envenom => CardKind::Power,
        CardName::EscapePlan => CardKind::Skill,
        CardName::Eviscerate => CardKind::Attack,
        CardName::Expertise => CardKind::Skill,
        CardName::Finisher => CardKind::Attack,
        CardName::Flechettes => CardKind::Attack,
        CardName::FlyingKnee => CardKind::Attack,
        CardName::Footwork => CardKind::Power,
        CardName::GlassKnife => CardKind::Attack,
        CardName::GrandFinale => CardKind::Attack,
        CardName::HeelHook => CardKind::Attack,
        CardName::InfiniteBlades => CardKind::Power,
        CardName::LegSweep => CardKind::Skill,
        CardName::Malaise => CardKind::Skill,
        CardName::MasterfulStab => CardKind::Attack,
        CardName::Neutralize => CardKind::Attack,
        CardName::Nightmare => CardKind::Skill,
        CardName::NoxiousFumes => CardKind::Power,
        CardName::Outmaneuver => CardKind::Skill,
        CardName::PhantasmalKiller => CardKind::Skill,
        CardName::PiercingWail => CardKind::Skill,
        CardName::PoisonedStab => CardKind::Attack,
        CardName::Predator => CardKind::Attack,
        CardName::Prepared => CardKind::Skill,
        CardName::QuickSlash => CardKind::Attack,
        CardName::Reflex => CardKind::Skill,
        CardName::RiddleWithHoles => CardKind::Attack,
        CardName::Setup => CardKind::Skill,
        CardName::Shiv => CardKind::Attack,
        CardName::Skewer => CardKind::Attack,
        CardName::Slice => CardKind::Attack,
        CardName::Slimed => CardKind::Status,
        CardName::SneakyStrike => CardKind::Attack,
        CardName::StormOfSteel => CardKind::Skill,
        CardName::Strike => CardKind::Attack,
        CardName::SuckerPunch => CardKind::Attack,
        CardName::Survivor => CardKind::Skill,
        CardName::Tactician => CardKind::Skill,
        CardName::Terror => CardKind::Skill,
        CardName::ToolsOfTheTrade => CardKind::Power,
        CardName::Unload => CardKind::Attack,
        CardName::WellLaidPlans => CardKind::Power,
        CardName::WraithForm => CardKind::Power,
    }
}

const fn rarity_eq(a: CardRarity, b: CardRarity) -> bool {
    matches!(
        (a, b),
        (CardRarity::Basic, CardRarity::Basic)
            | (CardRarity::Common, CardRarity::Common)
            | (CardRarity::Uncommon, CardRarity::Uncommon)
            | (CardRarity::Rare, CardRarity::Rare)
            | (CardRarity::Special, CardRarity::Special)
            | (CardRarity::Curse, CardRarity::Curse)
    )
}

const fn is_rewardable_kind(kind: CardKind) -> bool {
    matches!(kind, CardKind::Attack | CardKind::Skill | CardKind::Power)
}

const fn in_pool(name: CardName, rarity: CardRarity) -> bool {
    rarity_eq(card_rarity(name), rarity) && is_rewardable_kind(card_kind(name))
}

const fn count_pool(rarity: CardRarity) -> usize {
    let mut n = 0;
    let mut i = 0;
    while i < ALL_CARDS.len() {
        if in_pool(ALL_CARDS[i], rarity) {
            n += 1;
        }
        i += 1;
    }
    n
}

const fn build_pool<const N: usize>(rarity: CardRarity) -> [CardName; N] {
    let mut buf = [CardName::Strike; N];
    let mut idx = 0;
    let mut i = 0;
    while i < ALL_CARDS.len() {
        if in_pool(ALL_CARDS[i], rarity) {
            buf[idx] = ALL_CARDS[i];
            idx += 1;
        }
        i += 1;
    }
    buf
}

// Compile-time guarantee: ALL_CARDS contains every CardName exactly once
const _: () = assert!(ALL_CARDS.len() == CardName::COUNT);
const _: () = {
    let mut seen = [false; CardName::COUNT];
    let mut i = 0;
    while i < ALL_CARDS.len() {
        let idx = ALL_CARDS[i] as usize;
        assert!(!seen[idx], "ALL_CARDS contains a duplicate CardName");
        seen[idx] = true;
        i += 1;
    }
};

const COMMON_N: usize = count_pool(CardRarity::Common);
const UNCOMMON_N: usize = count_pool(CardRarity::Uncommon);
const RARE_N: usize = count_pool(CardRarity::Rare);

const REWARD_POOL_COMMON_ARR: [CardName; COMMON_N] = build_pool(CardRarity::Common);
const REWARD_POOL_UNCOMMON_ARR: [CardName; UNCOMMON_N] = build_pool(CardRarity::Uncommon);
const REWARD_POOL_RARE_ARR: [CardName; RARE_N] = build_pool(CardRarity::Rare);

pub const REWARD_POOL_COMMON: &[CardName] = &REWARD_POOL_COMMON_ARR;
pub const REWARD_POOL_UNCOMMON: &[CardName] = &REWARD_POOL_UNCOMMON_ARR;
pub const REWARD_POOL_RARE: &[CardName] = &REWARD_POOL_RARE_ARR;
