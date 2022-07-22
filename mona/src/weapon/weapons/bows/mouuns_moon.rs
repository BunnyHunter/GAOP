use crate::attribute::{Attribute, AttributeName};
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

pub struct MouunsMoonEffect {
    energy: usize
}

impl MouunsMoonEffect {
    pub fn new(config: &WeaponConfig) -> MouunsMoonEffect {
        match *config {
            WeaponConfig::MouunsMoon { energy } => MouunsMoonEffect {
                energy
            },
            _ => MouunsMoonEffect {
                energy: 0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for MouunsMoonEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let value = ((refine * 0.0003 + 0.0009) * self.energy as f64).min(refine * 0.1 + 0.3);
        attribute.set_value_by(AttributeName::BonusElementalBurst, "Mouun’s Moon Passive", value);
    }
}

pub struct MouunsMoon;

impl WeaponTrait for MouunsMoon {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::MouunsMoon,
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK60),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Watatsumi Wavewalker: For every point of the entire party’s combined maximum Energy, the Elemental Burst DMG of the character equipping this weapon is increased by 0.12/0.15/0.18/0.21/0.24%. A maximum of 40/50/60/70/80% increased Elemental Burst DMG can be achieved this way."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Mouun’s Moon"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "energy",
            title: "Total Energy Limit of the Team",
            config: ItemConfigType::Int { min: 40, max: 400, default: 40 }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(MouunsMoonEffect::new(config)))
    }
}
