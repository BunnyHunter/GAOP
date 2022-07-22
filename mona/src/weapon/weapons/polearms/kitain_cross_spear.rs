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

pub struct KitainCrossSpearEffect;

impl KitainCrossSpearEffect {
    pub fn new() -> KitainCrossSpearEffect {
        KitainCrossSpearEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for KitainCrossSpearEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.015 + 0.045;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "Kitain Cross Spear Passive", value);
    }
}

pub struct KitainCrossSpear;

impl WeaponTrait for KitainCrossSpear {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::KitainCrossSpear,
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM24),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Samurai Conduct: Increases Elemental Skill DMG by 6/7.5/9/10.5/12%. After Elemental Skill hits an opponent, lose 3 Energy but regenerate 3/3.5/4/4.5/5 Energy every 2s for the next 6s. This effect can occur once every 10s. Can be triggered even when the character is not on the field."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Kitain Cross Spear"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(KitainCrossSpearEffect::new()))
    }
}
