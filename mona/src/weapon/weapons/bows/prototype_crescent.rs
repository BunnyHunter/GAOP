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

pub struct PrototypeCrescentEffect {
    rate: f64
}

impl PrototypeCrescentEffect {
    pub fn new(config: &WeaponConfig) -> PrototypeCrescentEffect {
        match *config {
            WeaponConfig::PrototypeCrescent { rate } => PrototypeCrescentEffect {
                rate
            },
            _ => PrototypeCrescentEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for PrototypeCrescentEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.09 + 0.27) * self.rate;
        attribute.add_atk_percentage("Prototype Crescent Passive Equivalent", value);
    }
}

pub struct PrototypeCrescent;

impl WeaponTrait for PrototypeCrescent {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::PrototypeCrescent,
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Unreturning: Charged Attack hits on weak points increase Movement SPD by 10% and ATK by 36/45/56/63/72% for 10s."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Prototype Crescent"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01,
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(PrototypeCrescentEffect::new(config)))
    }
}
