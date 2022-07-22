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

pub struct DragonsBaneEffect {
    rate: f64
}

impl DragonsBaneEffect {
    pub fn new(config: &WeaponConfig) -> DragonsBaneEffect {
        match *config {
            WeaponConfig::DragonsBane { rate } => DragonsBaneEffect {
                rate
            },
            _ => DragonsBaneEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for DragonsBaneEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.04 + 0.16) * self.rate;
        attribute.set_value_by(AttributeName::BonusBase, "Dragons Bane Passive Equivalent", value);
    }
}

pub struct DragonsBane;

impl WeaponTrait for DragonsBane {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::DragonsBane,
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM48),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Bane of Flame and Water: Increases DMG against opponents affected by Hydro or Pyro by 20/24/28/32/36%."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Dragons Bane"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: ItemConfig::DEFAULT_RATE_TITLE,
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(DragonsBaneEffect::new(config)))
    }
}
