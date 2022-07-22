use crate::attribute::{Attribute, AttributeCommon, AttributeName};
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

pub struct LithicSpearEffect {
    liyue_count: usize
}

impl LithicSpearEffect {
    pub fn new(config: &WeaponConfig) -> LithicSpearEffect {
        match *config {
            WeaponConfig::LithicSpear { liyue_count } => LithicSpearEffect {
                liyue_count
            },
            _ => LithicSpearEffect {
                liyue_count: 0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for LithicSpearEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        attribute.add_atk_percentage("Lithic Spear Passive", (refine * 0.01 + 0.06) * self.liyue_count as f64);
        attribute.set_value_by(AttributeName::CriticalBase, "Lithic Spear Passive", (refine * 0.01 + 0.02) * self.liyue_count as f64);
    }
}

pub struct LithicSpear;

impl WeaponTrait for LithicSpear {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::LithicSpear,
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK60),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Lithic Axiom: Unity: For every character in the party who hails from Liyue, the character who equips this weapon gains a 7/8/9/10/11% ATK increase and a 3/4/5/6/7% CRIT Rate increase. This effect stacks up to 4 times."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Lithic Spear"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "liyue_count",
            title: "Number of Liyue Characters in the Team",
            config: ItemConfigType::Int { min: 0, max: 4, default: 0 }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(LithicSpearEffect::new(config)))
    }
}
