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

pub struct BlackcliffWarbowEffect {
    stack: f64
}

impl BlackcliffWarbowEffect {
    pub fn new(config: &WeaponConfig) -> BlackcliffWarbowEffect {
        match *config {
            WeaponConfig::BlackcliffWarbow { stack } => BlackcliffWarbowEffect {
                stack
            },
            _ => BlackcliffWarbowEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for BlackcliffWarbowEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let atk_bonus = (data.refine as f64 * 0.03 + 0.09) * self.stack;
        attribute.add_atk_percentage("Blackcliff Warbow Passive Equivalent", atk_bonus);
    }
}

pub struct BlackcliffWarbow;

impl WeaponTrait for BlackcliffWarbow {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::BlackcliffWarbow,
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage80),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Press The Advantage: After defeating an opponent, ATK is increased by 12/15/18/21/24% for 30s. This effect has a maximum of 3 stacks, and the duration of each stack is independent of the others."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Blackcliff Warbow "
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK03
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(BlackcliffWarbowEffect::new(config)))
    }
}
