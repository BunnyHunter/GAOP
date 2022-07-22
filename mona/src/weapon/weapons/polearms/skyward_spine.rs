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

pub struct SkywardSpineEffect;

impl SkywardSpineEffect {
    pub fn new() -> SkywardSpineEffect {
        SkywardSpineEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for SkywardSpineEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        attribute.set_value_by(AttributeName::CriticalBase, "Skyward Spine Passive", data.refine as f64 * 0.02 + 0.06);
        attribute.set_value_by(AttributeName::SpeedNormalAttack, "Skyward Spine Passive", 0.12);
    }
}

pub struct SkywardSpine;

impl WeaponTrait for SkywardSpine {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SkywardSpine,
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge80),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Black Wing: Increases CRIT Rate by 8/10/12/14/16% and increases Normal ATK SPD by 12%. Additionally, Normal and Charged Attacks hits on opponents have a 50% chance to trigger a vaccuum blade that deals 40/55/70/85/100% of ATK as DMG in a small AoE. This effect can occur no more than once every 2s."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Skyward Spine"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(SkywardSpineEffect::new()))
    }
}
