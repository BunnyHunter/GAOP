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

pub struct SummitShaperEffect {
    stack: f64,
    shield_rate: f64,
}

impl SummitShaperEffect {
    pub fn new(config: &WeaponConfig) -> SummitShaperEffect {
        match *config {
            WeaponConfig::SummitShaper { stack, shield_rate } => SummitShaperEffect {
                stack, shield_rate
            },
            _ => SummitShaperEffect {
                stack: 0.0,
                shield_rate: 0.0
            },
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for SummitShaperEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine;

        attribute.set_value_by(AttributeName::ShieldStrength, "Summit Shaper Passive ", refine as f64 * 0.05 + 0.15);

        let atk_bonus = (refine as f64 * 0.01 + 0.03) * (1.0 + self.shield_rate) * self.stack;
        attribute.add_atk_percentage("Summit Shaper Passive Equivalent", atk_bonus);
    }
}

pub struct SummitShaper;

impl WeaponTrait for SummitShaper {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SummitShaper,
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK108),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Golden Majesty: Increases Shield Strength by 20/25/30/35/40%. Scoring hits on opponents increases ATK by 4/5/6/7/8% sfor 8s. Max 5 stacks. Can only occur once every 0.3s. While protected by a shield, this ATK increase effect is increased by 100%."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Summit Shaper"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK05,
        ItemConfig {
            name: "shield_rate",
            title: "Shield Uptime",
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(SummitShaperEffect::new(config)))
    }
}
