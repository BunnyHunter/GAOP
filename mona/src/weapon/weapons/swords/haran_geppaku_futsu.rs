use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct HaranGeppakuFutsuEffect {
    pub stack: f64
}

impl<A: Attribute> WeaponEffect<A> for HaranGeppakuFutsuEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let bonus1 = refine * 0.03 + 0.09;
        attribute.add_elemental_bonus("Haran Geppaku Futsu Passive ", bonus1);

        let bonus2 = refine * 0.05 + 0.15;
        attribute.set_value_by(AttributeName::BonusNormalAttack, "Haran Geppaku Futsu Passive Equivalent", bonus2);
    }
}

pub struct HaranGeppakuFutsu;

impl WeaponTrait for HaranGeppakuFutsu {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::HaranGeppakuFutsu,
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate72),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Honed Flow: Obtain 12/15/18/21/24% All Elemental DMG Bonus. When other nearby party members use Elemental Skills, the character equipping this weapon will gain 1 Wavespike stack. Max 2 stacks.<br>This effect can be triggered once every 0.3s. When the character equipping this weapon uses an Elemental Skill, all stacks of Wavespike will be consumed to gain Rippling Upheaval. Each stack of Wavespike consumed will increase Normal Attack DMG by 20/25/30/35/40% for 8s."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Haran Geppaku Futsu"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: "Haran Geppaku Futsu Passive Stacks",
            config: ItemConfigType::Float { min: 0.0, max: 2.0, default: 2.0 }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let stack = match *config {
            WeaponConfig::HaranGeppakuFutsu { stack } => stack,
            _ => 0.0
        };

        Some(Box::new(HaranGeppakuFutsuEffect {
            stack
        }))
    }
}