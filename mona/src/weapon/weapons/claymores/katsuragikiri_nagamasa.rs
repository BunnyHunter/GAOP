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

pub struct KatsuragikiriNagamasaEffect;

impl KatsuragikiriNagamasaEffect {
    pub fn new() -> KatsuragikiriNagamasaEffect {
        KatsuragikiriNagamasaEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for KatsuragikiriNagamasaEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.015 + 0.045;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "Katsuragikiri Nagamasa Passive", value);
    }
}

pub struct KatsuragikiriNagamasa;

impl WeaponTrait for KatsuragikiriNagamasa {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::KatsuragikiriNagamasa,
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge100),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Samurai Conduct: Increases Elemental Skill DMG by 6/7.5/9/10.5/12%. After Elemental Skill hits an opponent, lose 3 Energy but regenerate 3/3.5/4/4.5/5 Energy every 2s for the next 6s. This effect can occur once every 10s. Can be triggered even when the character is not on the field."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Katsuragikiri Nagamasa"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(KatsuragikiriNagamasaEffect::new()))
    }
}
