use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct RedhornStonethresherEffect;

impl RedhornStonethresherEffect {
    pub fn new() -> RedhornStonethresherEffect {
        RedhornStonethresherEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for RedhornStonethresherEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let def_bonus = refine * 0.07 + 0.21;
        attribute.add_def_percentage("Redhorn Stonethresher Passive", def_bonus);
        let bonus = refine * 0.1 + 0.3;
        attribute.set_value_by(AttributeName::DEFRatioNormalAttack, "Redhorn Stonethresher Passive", bonus);
        attribute.set_value_by(AttributeName::DEFRatioChargedAttack, "Redhorn Stonethresher Passive", bonus);
    }
}

pub struct RedhornStonethresher;

impl WeaponTrait for RedhornStonethresher {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::RedhornStonethresher,
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage192),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Gokadaiou Otoginabanashi: DEF is increased by 28/35/42/49/56%. Normal and Charged Attack DMG is increased by 40/50/60/70/80% of DEF."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Redhorn Stonethresher"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(RedhornStonethresherEffect::new()))
    }
}
