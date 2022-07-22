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

pub struct ThunderingPulseEffect {
    stack: usize
}

impl ThunderingPulseEffect {
    pub fn new(config: &WeaponConfig) -> ThunderingPulseEffect {
        match *config {
            WeaponConfig::ThunderingPulse { stack } => ThunderingPulseEffect {
                stack
            },
            _ => ThunderingPulseEffect {
                stack: 0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for ThunderingPulseEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        attribute.add_atk_percentage("Thundering Pulse Passive", refine * 0.05 + 0.15);
        let bonus = if self.stack == 1 {
            refine * 0.03 + 0.09
        } else if self.stack == 2 {
            refine * 0.06 + 0.18
        } else if self.stack == 3 {
            refine * 0.1 + 0.3
        } else {
            0.0
        };
        attribute.set_value_by(AttributeName::BonusNormalAttack, "Thundering Pulse Passive", bonus);
    }
}

pub struct ThunderingPulse;

impl WeaponTrait for ThunderingPulse {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::ThunderingPulse,
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage144),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Rule by Thunder: Increases ATK by 20/25/30/35/40% and grants the might of the Thunder Emblem. At stack levels 1/2/3, the Thunder Emblem increases Normal Attack DMG by 12/24/40%–15/30/59%–18/36/60%–21/42/70%–24/48/80%. The character will obtain 1 stack of Thunder Emblem in each of the following scenarios: Normal Attack deals DMG (stack lasts 5s), casting Elemental Skill (stacks lasts 10s); Energy is less than 100% (stack disappears when Energy is full). Each stack’s duration is calculated independently."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Thundering Pulse"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: "Thundering Pulse Stacks",
            config: ItemConfigType::Int { min: 0, max: 3, default: 0 }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(ThunderingPulseEffect::new(config)))
    }
}
