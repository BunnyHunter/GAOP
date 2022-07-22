use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct PrototypeAmber;

impl WeaponTrait for PrototypeAmber {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::PrototypeAmber,
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::HP90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Golden Plating: Using an Elemental Burst regenerates 4/4.5/5/5.5/6 Energy every 2s for 6s. All party members will regenerate 4/4.5/5/5.5/6% HP every 2s for this duration."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Prototype Amber"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
