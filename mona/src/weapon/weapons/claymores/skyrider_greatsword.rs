use crate::attribute::{Attribute, AttributeCommon};
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

pub struct SkyriderGreatswordEffect {
    pub stack: f64,
}

impl SkyriderGreatswordEffect {
    pub fn new(stack: f64) -> Self {
        Self { stack }
    }
}

impl<A: Attribute> WeaponEffect<A> for SkyriderGreatswordEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = (refine * 0.01 + 0.05) * self.stack;
        attribute.add_atk_percentage("Skyrider GreatswordPassive Equivalent", value);
    }
}

pub struct SkyriderGreatsword;

impl WeaponTrait for SkyriderGreatsword {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SkyriderGreatsword,
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::PhysicalBonus96),
        weapon_base: WeaponBaseATKFamily::ATK401,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Courage: On hit, Normal or Charged Attacks increase ATK by 6/7/8/9/10% for 6s. Max 4 stacks. Can only occur once every 0.5s."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Skyrider Greatsword"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK04
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let stack = match *config {
            WeaponConfig::SkyriderGreatsword { stack } => stack,
            _ => 0.0
        };

        Some(Box::new(SkyriderGreatswordEffect::new(stack)))
    }
}
