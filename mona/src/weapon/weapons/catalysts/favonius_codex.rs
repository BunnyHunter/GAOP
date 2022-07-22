use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct FavoniusCodex;

impl WeaponTrait for FavoniusCodex {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FavoniusCodex,
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge100),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Wind Companion: CRIT hits have a 60/70/80/90/100% chance to generate a small amount of Elemental Particles, which will regenerate 6 Energy for the character. Can only occur once every 2/10,5/9/7,5/6 s."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Favonius Codex"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
