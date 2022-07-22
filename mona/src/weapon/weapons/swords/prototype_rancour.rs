use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct PrototypeRancourEffect {
    stack: f64
}

impl PrototypeRancourEffect {
    pub fn new(config: &WeaponConfig) -> PrototypeRancourEffect {
        match *config {
            WeaponConfig::PrototypeRancour { stack } => PrototypeRancourEffect {
                stack,
            },
            _ => PrototypeRancourEffect {
                stack: 0.0,
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for PrototypeRancourEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.01 + 0.03) * self.stack;
        attribute.add_atk_percentage("Prototype Rancour Passive Equivalent", value);
        attribute.add_def_percentage("Prototype Rancour Passive Equivalent", value);
    }
}

pub struct PrototypeRancour;

impl WeaponTrait for PrototypeRancour {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::PrototypeRancour,
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::PhysicalBonus75),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Crushed Stone: On hit, Normal or Charged Attacks increase ATK and DEF by 4/4.5/5/5.5/6% for 6s. Max 4 stacks. Can only occur once every 0.3s."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Prototype Rancour"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: "Prototype Rancour Passive Equivalent Stacks",
            config: ItemConfigType::Float {
                min: 0.0,
                max: 4.0,
                default: 0.0
            }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(PrototypeRancourEffect::new(config)))
    }
}
