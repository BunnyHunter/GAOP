use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct FavoniusLance;

impl WeaponTrait for FavoniusLance {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FavoniusLance,
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge67),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Wind Companion: CRIT hits have a 60/70/80/90/100% chance to generate a small amount of Elemental Particles, which will regenerate 6 Energy for the character.<br>Can only occur once every 12/10.5/9/7.5/6 s."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Favonius Lance"
    };

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
