use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct BlackcliffSlasherEffect {
    stack: f64
}

impl BlackcliffSlasherEffect {
    pub fn new(config: &WeaponConfig) -> BlackcliffSlasherEffect {
        match *config {
            WeaponConfig::BlackcliffSlasher { stack } => BlackcliffSlasherEffect {
                stack
            },
            _ => BlackcliffSlasherEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for BlackcliffSlasherEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.03 + 0.09) * self.stack;
        attribute.add_atk_percentage("Blackcliff Slasher Passive Equivalent", value);
    }
}

pub struct BlackcliffSlasher;

impl WeaponTrait for BlackcliffSlasher {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::BlackcliffSlasher,
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage120),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Press The Advantage: After defeating an opponent, ATK is increased by 12/15/18/21/24% for 30s. This effect has a maximum of 3 stacks, and the duration of each stack is independent of the others."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Blackcliff Slasher"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: "Blackcliff Slasher Passive Equivalent Stacks",
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 0.0 }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(BlackcliffSlasherEffect::new(config)))
    }
}
