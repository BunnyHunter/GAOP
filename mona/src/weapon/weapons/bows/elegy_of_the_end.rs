use crate::attribute::{Attribute, AttributeName, AttributeCommon};
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

pub struct ElegyOfTheEndEffect {
    rate: f64
}

impl ElegyOfTheEndEffect {
    pub fn new(config: &WeaponConfig) -> ElegyOfTheEndEffect {
        match *config {
            WeaponConfig::ElegyOfTheEnd { rate } => ElegyOfTheEndEffect {
                rate
            },
            _ => ElegyOfTheEndEffect {
                rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for ElegyOfTheEndEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let em_bonus = refine * 15.0 + 45.0 + (refine * 25.0 + 75.0) * self.rate;
        attribute.set_value_by(AttributeName::ElementalMastery, "Millennial Movement: Farewell Song Passive Equivalent", em_bonus);
        let atk_bonus = (refine * 0.05 + 0.15) * self.rate;
        attribute.add_atk_percentage("Millennial Movement: Farewell Song Passive Equivalent", atk_bonus);
    }
}

pub struct ElegyOfTheEnd;

impl WeaponTrait for ElegyOfTheEnd {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::ElegyOfTheEnd,
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge120),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("The Parting Realm：A part of the 'Millennial Movement' that wanders amidst the winds. Increases Elemental Mastery by 60/75/90/105/120. When the Elemental Skills or Elemental Bursts of the character wielding this weapon hit opponents, that character gains a Sigil of Remembrance. This effect can be triggered once every 0.2s and can be triggered even if said character is not on the field. When you possess 4 Sigils of Remembrance, all of them will be consumed and all nearby party members will obtain the 'Millennial Movement: Farewell Song' effect for 12s. 'Millennial Movement: Farewell Song' increases Elemental Mastery by 100/125/150/175/200 and increases ATK by 20/25/30/35/40%. Once this effect is triggered, you will not gain Sigils of Remembrance for 20s. Of the many effects of the 'Millennial Movement' buffs of the same type will not stack. "),
        #[cfg(not(target_family = "wasm"))]
        chs: "Elegy for the End"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01,
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(ElegyOfTheEndEffect::new(config)))
    }
}
