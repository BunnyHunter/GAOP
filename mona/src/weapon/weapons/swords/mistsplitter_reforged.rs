use crate::attribute::{AttributeName, Attribute};
use crate::common::{Element, WeaponType};
use super::super::super::weapon_effect::WeaponEffect;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::character::Character;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::WeaponName;
use super::super::super::weapon_config::WeaponConfig;

pub struct MistsplitterReforgedEffect {
    pub level: i32,
    pub element: Element,
}

impl<T: Attribute> WeaponEffect<T> for MistsplitterReforgedEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value1 = data.refine as f64 * 0.03 + 0.09;
        let key = "Mistsplitter Reforged Passive";
        attribute.set_value_by(AttributeName::BonusElectro, key, value1);
        attribute.set_value_by(AttributeName::BonusHydro, key, value1);
        attribute.set_value_by(AttributeName::BonusAnemo, key, value1);
        attribute.set_value_by(AttributeName::BonusPyro, key, value1);
        attribute.set_value_by(AttributeName::BonusCryo, key, value1);
        attribute.set_value_by(AttributeName::BonusDendro, key, value1);
        attribute.set_value_by(AttributeName::BonusGeo, key, value1);

        let value2 = if self.level == 1 {
            0.02 * data.refine as f64 + 0.06
        } else if self.level == 2 {
            0.04 * data.refine as f64 + 0.12
        } else if self.level == 3 {
            0.07 * data.refine as f64 + 0.21
        } else {
            0.0
        };

        let attribute_name = AttributeName::bonus_name_by_element(self.element);
        attribute.set_value_by(attribute_name, key, value2);
    }
}

impl MistsplitterReforgedEffect {
    pub fn new(config: &WeaponConfig, element: Element) -> MistsplitterReforgedEffect {
        let level = match *config {
            WeaponConfig::MistsplitterReforged { emblem_level } => emblem_level,
            _ => 0
        };

        MistsplitterReforgedEffect {
            element,
            level: level as i32
        }
    }
}

pub struct MistsplitterReforged;

impl WeaponTrait for MistsplitterReforged {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::MistsplitterReforged,
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage96),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Mistsplitter’s Edge: Gain a 12/15/18/21/24% Elemental DMG Bonus for every element and receive the might of Mistsplitter’s Seal. At stack levels 1/2/3, Mistsplitter’s Seal provides a 8/16/28–10/20/35–12/24/42–14/28/49–16/32/56% Elemental DMG Bonus for the character’s Elemental Type. The character will obtain 1 stack of Mistsplitter’s Seal in each of the following scenarios: Normal Attack deals Elemental DMG (stack lasts 5s), casting Elemental Burst (stacks lasts 10s); Energy is less than 100% (stack disappears when Energy is full). Each stack’s duration is calculated independently."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Mistsplitter Reforged"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "emblem_level",
            title: "Mistsplitter Reforged Passive Stacks",
            config: ItemConfigType::Int {
                min: 0,
                max: 3,
                default: 3
            }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(MistsplitterReforgedEffect::new(config, character.static_data.element)))
    }
}
