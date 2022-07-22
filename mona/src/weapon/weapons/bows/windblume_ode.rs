use crate::attribute::{Attribute, AttributeCommon};
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

pub struct WindblumeOdeEffect {
    rate: f64
}

impl WindblumeOdeEffect {
    pub fn new(config: &WeaponConfig) -> WindblumeOdeEffect {
        match *config {
            WeaponConfig::WindblumeOde { rate } => WindblumeOdeEffect {
                rate
            },
            _ => WindblumeOdeEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for WindblumeOdeEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let atk_bonus = (data.refine as f64 * 0.04 + 0.12) * self.rate;
        attribute.add_atk_percentage("Windblume Ode Passive Equivalent", atk_bonus);
    }
}

pub struct WindblumeOde;

impl WeaponTrait for WindblumeOde {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::WindblumeOde,
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM36),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Windblume Wish: After using Elemental Skill, receive a boon from the ancient wish of the Windblume, increasing ATK by 16/20/24/28/32% for 6s."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Windblume Ode"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(WindblumeOdeEffect::new(config)))
    }
}
