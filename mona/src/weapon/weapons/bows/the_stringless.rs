use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct TheStringlessEffect;

impl TheStringlessEffect {
    pub fn new() -> TheStringlessEffect {
        TheStringlessEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for TheStringlessEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.06 + 0.18;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "The Stringless Passive", value);
        attribute.set_value_by(AttributeName::BonusElementalBurst, "The Stringless Passive", value);
    }
}

pub struct TheStringless;

impl WeaponTrait for TheStringless {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TheStringless,
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM36),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Arowless Song: Increases Elemental Skill and Elemental Burst DMG by 24/30/36/42/48%."),
        #[cfg(not(target_family = "wasm"))]
        chs: "The Stringless"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(TheStringlessEffect::new()))
    }
}
