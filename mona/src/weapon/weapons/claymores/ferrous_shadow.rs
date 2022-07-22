use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct FerrousShadowEffect {
    pub rate: f64
}

impl FerrousShadowEffect {
    pub fn new(rate: f64) -> Self {
        Self { rate }
    }
}

impl<A: Attribute> WeaponEffect<A> for FerrousShadowEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = (refine * 0.05 + 0.25) * self.rate;
        attribute.set_value_by(AttributeName::BonusChargedAttack, "Ferrous ShadowPassive Equivalent", value);
    }
}

pub struct FerrousShadow;

impl WeaponTrait for FerrousShadow {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FerrousShadow,
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::HP77),
        weapon_base: WeaponBaseATKFamily::ATK401,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Unbending: When HP falls below 70/75/80/85/90%, increases Charged Attack DMG by 30/35/40/45/50%, and Charged Attacks become much harder to interrupt."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Ferrous Shadow"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::FerrousShadow { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(FerrousShadowEffect::new(rate)))
    }
}
