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

pub struct WolfsGravestoneEffect {
    rate: f64,
}

impl WolfsGravestoneEffect {
    pub fn new(config: &WeaponConfig) -> WolfsGravestoneEffect {
        match *config {
            WeaponConfig::WolfsGravestone { rate } => WolfsGravestoneEffect {
                rate
            },
            _ => WolfsGravestoneEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for WolfsGravestoneEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let value1 = refine * 0.05 + 0.15;
        let value2 = (refine * 0.1 + 0.3) * self.rate;
        attribute.add_atk_percentage("Wolfs Gravestone Passive Equivalent", value1 + value2);
    }
}

pub struct WolfsGravestone;

impl WeaponTrait for WolfsGravestone {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::WolfsGravestone,
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK108),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Wolfish Tracker：Increases ATK by 20/25/30/35/40%. On hit, attacks against opponents with less than 30% HP increase all party members’ ATK by 40/50/60/70/80% for 12s. Can only occur once every 30s."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Wolfs Gravestone"
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
        Some(Box::new(WolfsGravestoneEffect::new(config)))
    }
}
