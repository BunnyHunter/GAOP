use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct MitternachtsWaltzEffect {
    rate1: f64,
    rate2: f64
}

impl MitternachtsWaltzEffect {
    pub fn new(config: &WeaponConfig) -> MitternachtsWaltzEffect {
        match *config {
            WeaponConfig::MitternachtsWaltz { rate1, rate2 } => MitternachtsWaltzEffect {
                rate1,
                rate2
            },
            _ => MitternachtsWaltzEffect {
                rate1: 0.0,
                rate2: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for MitternachtsWaltzEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.05 + 0.15;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "Mitternachts Waltz Passive Equivalent", value * self.rate1);
        attribute.set_value_by(AttributeName::BonusNormalAttack, "Mitternachts Waltz Passive Equivalent", value * self.rate2);
    }
}

pub struct MitternachtsWaltz;

impl WeaponTrait for MitternachtsWaltz {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::MitternachtsWaltz,
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::PhysicalBonus113),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Evernight Duet: Normal Attack hits on opponents increase Elemental Skill DMG by 20/25/30/35/40% for 5s. Elemental Skill hits on opponents increase Normal Attack DMG by 20/25/30/35/40% for 5s."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Mitternachts Waltz"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate1",
            title: "Effect 1 Rate",
            config: ItemConfig::RATE01_TYPE
        },
        ItemConfig {
            name: "rate2",
            title: "Effect 2 Rate",
            config: ItemConfig::RATE01_TYPE
        },
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(MitternachtsWaltzEffect::new(config)))
    }
}
