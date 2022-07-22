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

pub struct PolarStarEffect {
    stack: usize
}

impl PolarStarEffect {
    pub fn new(config: &WeaponConfig) -> PolarStarEffect {
        match *config {
            WeaponConfig::PolarStar { stack } => PolarStarEffect {
                stack
            },
            _ => PolarStarEffect {
                stack: 0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for PolarStarEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let bonus1 = refine * 0.03 + 0.09;
        attribute.set_value_by(AttributeName::BonusElementalSkill, "Polar Start Passive", bonus1);
        attribute.set_value_by(AttributeName::BonusElementalBurst, "Polar Start Passive", bonus1);

        let atk_bonus = if self.stack == 1 {
            refine * 0.025 + 0.075
        } else if self.stack == 2 {
            refine * 0.05 + 0.15
        } else if self.stack == 3 {
            refine * 0.075 + 0.225
        } else if self.stack == 4 {
            refine * 0.12 + 0.36
        } else {
            0.0
        };

        attribute.add_atk_percentage("Polar Passive Equivalent", atk_bonus);
    }
}

pub struct PolarStar;

impl WeaponTrait for PolarStar {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::PolarStar,
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate72),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Elemental Skill and Elemental Burst DMG increased by 12%. After a Normal Attack, Charged Attack, Elemental Skill or Elemental Burst hits an opponent, 1 stack of Ashen Nightstar will be gained for 12s. When 1/2/3/4 stacks of Ashen Nightstar are present, ATK is increased by 10/20/30/48%. The stack of Ashen Nightstar created by the Normal Attack, Charged Attack, Elemental Skill or Elemental Burst will be counted independently of the others."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Polar Star"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: "Polar Star Passive Stacks",
            config: ItemConfigType::Int { min: 0, max: 4, default: 0 }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(PolarStarEffect::new(config)))
    }
}
