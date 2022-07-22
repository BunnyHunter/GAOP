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

pub struct WhiteblindEffect {
    stack: f64,
}

impl WhiteblindEffect {
    pub fn new(config: &WeaponConfig) -> WhiteblindEffect {
        match *config {
            WeaponConfig::Whiteblind { stack } => WhiteblindEffect {
                stack
            },
            _ => WhiteblindEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for WhiteblindEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.015 + 0.045) * self.stack;
        attribute.add_atk_percentage("Whiteblind Passive Equivalent", value);
        attribute.add_def_percentage("Whiteblind Passive Equivalent", value);
    }
}

pub struct Whiteblind;

impl WeaponTrait for Whiteblind {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::Whiteblind,
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::DEF113),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Infused Edge: On hit, Normal or Charged Attacks increase ATK and DEF by 6/7.5/9/10.5/12% for 6s. Max 4 stacks. Can only occur once every 0.5s."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Whiteblind"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: "Passive Equivalent Stacks",
            config: ItemConfigType::Float {
                min: 0.0,
                max: 4.0,
                default: 0.0
            }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(WhiteblindEffect::new(config)))
    }
}
