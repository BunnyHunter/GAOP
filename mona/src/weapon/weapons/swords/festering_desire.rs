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

pub struct FesteringDesireEffect;

impl FesteringDesireEffect {
    pub fn new() -> FesteringDesireEffect {
        FesteringDesireEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for FesteringDesireEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let bonus1 = refine * 0.015 + 0.045;
        attribute.set_value_by(AttributeName::CriticalElementalSkill, "Festering Desire Passive", bonus1);
        let bonus2 = refine * 0.04 + 0.12;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "Festering Desire Passive", bonus2);
    }
}

pub struct FesteringDesire;

impl WeaponTrait for FesteringDesire {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FesteringDesire,
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge100),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Undying Admiration: Increases Elemental Skill DMG by 16/20/24/28/32%, and Elemental Skill CRIT Rate by 6/7,5/9/10,5/12%."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Festering Desire"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(FesteringDesireEffect::new()))
    }
}
