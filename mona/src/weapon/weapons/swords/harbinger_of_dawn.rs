use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct HarbingerOfDawnEffect {
    pub rate: f64,
}

impl HarbingerOfDawnEffect {
    pub fn new(rate: f64) -> Self {
        Self { rate }
    }
}

impl<A: Attribute> WeaponEffect<A> for HarbingerOfDawnEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = (refine * 0.035 + 0.105) * self.rate;
        attribute.set_value_by(AttributeName::CriticalBase, "Harbinger of Dawn Passive Equivalent", value);
    }
}

pub struct HarbingerOfDawn;

impl WeaponTrait for HarbingerOfDawn {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::HarbingerOfDawn,
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage102),
        weapon_base: WeaponBaseATKFamily::ATK401,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Vigorous: When HP is above 90%, increases CRIT Rate by 14/18/22/24/28%."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Harbinger of Dawn"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "Harbinger of Dawn Passive Application Rate",
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::HarbingerOfDawn { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(HarbingerOfDawnEffect::new(rate)))
    }
}
