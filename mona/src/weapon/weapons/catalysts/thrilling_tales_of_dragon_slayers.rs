use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct ThrillingTalesOfDragonSlayers;

impl WeaponTrait for ThrillingTalesOfDragonSlayers {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::ThrillingTalesOfDragonSlayers,
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::HP77),
        weapon_base: WeaponBaseATKFamily::ATK401,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Heritage: When switching characters, the new character taking the field has their ATK increased by 24/30/36/42/48% for 10s."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Thrilling Tales of Dragon Slayers",
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
