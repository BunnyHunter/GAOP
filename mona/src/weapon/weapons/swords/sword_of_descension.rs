use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct SwordOfDescension;

impl WeaponTrait for SwordOfDescension {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SwordOfDescension,
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK77),
        weapon_base: WeaponBaseATKFamily::ATK440,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Descension: This weapon's effect is only applied on the following platform(s):<br>'PlayStation Network'<br>Hitting enemies with Normal or Charged Attacks grants a 50% chance to deal 200% ATK as DMG in a small AoE. This effect can only occur once every 10s.<br>Additionally, if the Traveler equips the Sword of Descension, their ATK is increased by 66."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Sword of Descension"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
