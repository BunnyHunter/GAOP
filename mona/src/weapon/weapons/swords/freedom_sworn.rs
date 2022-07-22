use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct FreedomSwornEffect {
    rate: f64,
}

impl FreedomSwornEffect {
    pub fn new(config: &WeaponConfig) -> FreedomSwornEffect {
        match *config {
            WeaponConfig::FreedomSworn { rate } => FreedomSwornEffect {
                rate,
            },
            _ => FreedomSwornEffect {
                rate: 0.0,
            },
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for FreedomSwornEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine;

        attribute.set_value_by(AttributeName::BonusBase, "Freedom-Sworn ", refine as f64 * 0.025 + 0.075);
        let dmg_bonus = (refine as f64 * 0.04 + 0.12) * self.rate;
        let atk_bonus = (refine as f64 * 0.05 + 0.15) * self.rate;
        attribute.set_value_by(AttributeName::BonusNormalAttack, "Freedom-Sworn Passive Equivalent", dmg_bonus);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "Freedom-Sworn Passive Equivalent", dmg_bonus);
        attribute.set_value_by(AttributeName::BonusPlungingAttack, "Freedom-Sworn Passive Equivalent", dmg_bonus);
        attribute.add_atk_percentage("Freedom-Sworn Passive Equivalent", atk_bonus);
    }
}

pub struct FreedomSworn;

impl WeaponTrait for FreedomSworn {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FreedomSworn,
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM43),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Revolutionary Chorale: A part of the 'Millennial Movement' that wanders amidst the winds. Increases DMG by 10/12.5/15/17.5/20%. When the character wielding this weapon triggers Elemental Reactions, they gain a Sigil of Rebellion. This effect can be triggered once every 0.5s and can be triggered even if said character is not on the field. When you possess 2 Sigils of Rebellion, all of them will be consumed and all nearby party members will obtain 'Millennial Movement: Song of Resistance' for 12s. 'Millennial Movement: Song of Resistance' increases Normal, Charged, and Plunging Attack DMG by 16/20/24/28/32% and increases ATK by 20/25/30/35/40%. Once this effect is triggered, you will not gain Sigils of Rebellion for 20s. Of the many effects of the 'Millennial Movement,' buffs of the same type will not stack. "),
        #[cfg(not(target_family = "wasm"))]
        chs: "Freedom-Sworn"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "Passive Application Ratio",
            config: ItemConfigType::Float {
                min: 0.0,
                max: 1.0,
                default: 0.0
            }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(FreedomSwornEffect::new(config)))
    }
}
