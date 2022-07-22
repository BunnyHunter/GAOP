use crate::attribute::{Attribute, AttributeCommon, AttributeName};
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

pub struct StaffOfHomaEffect {
    be50_rate: f64
}

impl StaffOfHomaEffect {
    pub fn new(config: &WeaponConfig) -> StaffOfHomaEffect {
        match *config {
            WeaponConfig::StaffOfHoma { be50_rate } => StaffOfHomaEffect {
                be50_rate
            },
            _ => StaffOfHomaEffect {
                be50_rate: 0.0,
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for StaffOfHomaEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let hp_bonus = refine * 0.05 + 0.15;
        attribute.add_hp_percentage("Staff of Homa Passive", hp_bonus);
        let atk_bonus_ratio = refine * 0.002 + 0.006 + (refine * 0.002 + 0.008) * self.be50_rate;
        attribute.add_edge1(
            AttributeName::HP,
            AttributeName::ATKFixed,
            Box::new(move |x, _| x * atk_bonus_ratio),
            Box::new(move |grad, _x1, _x2| (grad * atk_bonus_ratio, 0.0)),
            "Staff of Homa Passive Equivalent"
        );
    }
}

pub struct StaffOfHoma;

impl WeaponTrait for StaffOfHoma {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::StaffOfHoma,
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage144),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Reckless Cinnabar: HP increased by 20%. Additionally, provides an ATK Bonus based on 0.8/1/1.2/1.4/1.6% of the wielder’s Max HP. When the wielder’s HP is less than 50%, this ATK Bonus is increased by an additional 1/1.2/1.4/1.6/1.8% of Max HP."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Staff of Homa"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "be50_rate",
            title: "HP Below 50% Uptime",
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(StaffOfHomaEffect::new(config)))
    }
}
