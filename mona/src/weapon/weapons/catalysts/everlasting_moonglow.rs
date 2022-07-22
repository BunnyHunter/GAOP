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

pub struct EverlastingMoonglowEffect;

impl EverlastingMoonglowEffect {
    pub fn new() -> EverlastingMoonglowEffect {
        EverlastingMoonglowEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for EverlastingMoonglowEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        attribute.set_value_by(AttributeName::HealingBonus, "Everlasting Moonglow Passive", refine * 0.025 + 0.075);
        attribute.set_value_by(AttributeName::HPRatioNormalAttack, "Everlasting Moonglow Passive", refine * 0.005 + 0.005);
    }
}

pub struct EverlastingMoonglow;

impl WeaponTrait for EverlastingMoonglow {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::EverlastingMoonglow,
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::HP108),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Byakuya Kougetsu: Healing Bonus increased by 10/12.5/15/17.5/20%, Normal Attack DMG is increased by 1/1.5/2/2.5/3% of Max HP of the character equipping this weapon. For 12s after using an Elemental Burst, Normal Attacks that hit opponents will restore 0.6 Energy. Energy can be restored this way once every 0.1s."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Everlasting Moonglow"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(EverlastingMoonglowEffect::new()))
    }
}
