use crate::attribute::{Attribute, AttributeName};
use crate::character::{CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct PredatorEffect {
    stack: f64,
    is_aloy: bool
}

impl PredatorEffect {
    pub fn new(config: &WeaponConfig, common_data: &CharacterCommonData) -> PredatorEffect {
        let is_aloy = common_data.name == CharacterName::Aloy;
        match *config {
            WeaponConfig::Predator { stack } => PredatorEffect {
                stack,
                is_aloy
            },
            _ => PredatorEffect {
                stack: 0.0,
                is_aloy
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for PredatorEffect {
    fn apply(&self, _data: &WeaponCommonData, attribute: &mut T) {
        let value = 0.1 + self.stack;
        attribute.set_value_by(AttributeName::BonusNormalAttack, "Predator Bow Passive Equivalent", value);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "Predator Bow Passive Equivalent", value);
        if self.is_aloy {
            attribute.set_value_by(AttributeName::ATKFixed, "Predator Bow Passive", 66.0);
        }
    }
}

pub struct Predator;

impl WeaponTrait for Predator {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::Predator,
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Strong Strike: *Effective for players on “Playstation Network” only.<br>Dealing Cryo DMG to opponents increases this character’s Normal and Charged Attack DMG by 10% for 6s. This effect can have a maximum of 2 stacks. Additionally, when Aloy equips Predator, ATK is increased by 66."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Predator Bow"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK02
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(PredatorEffect::new(config, character)))
    }
}
