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

pub struct TheBlackSwordEffect;

impl TheBlackSwordEffect {
    pub fn new() -> TheBlackSwordEffect {
        TheBlackSwordEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for TheBlackSwordEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let bonus = data.refine as f64 * 0.05 + 0.15;
        attribute.set_value_by(AttributeName::BonusNormalAttack, "The Black Sword Pssive", bonus);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "The Black Sword Pssive", bonus);
    }
}

pub struct TheBlackSword;

impl WeaponTrait for TheBlackSword {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TheBlackSword,
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate60),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Justice: Increases DMG dealt by Normal and Charged Attacks by 20/25/30/35/40%. Additionally, regenerates 60/70/80/90/100% of ATK as HP when Normal and Charged Attacks score a CRIT Hit. This effect can occur once every 5s."),
        #[cfg(not(target_family = "wasm"))]
        chs: "The Black Sword"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(TheBlackSwordEffect::new()))
    }
}
