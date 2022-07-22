use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct LuxuriousSeaLordEffect;

impl LuxuriousSeaLordEffect {
    pub fn new() -> LuxuriousSeaLordEffect {
        LuxuriousSeaLordEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for LuxuriousSeaLordEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.03 + 0.09;
        attribute.set_value_by(AttributeName::BonusElementalBurst, "Luxurious Sea Lord Passive", value);
    }
}

pub struct LuxuriousSeaLord;

impl WeaponTrait for LuxuriousSeaLord {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::LuxuriousSeaLord,
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK120),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Oceanic Victory: Increases Elemental Burst DMG by 12/15/18/21/24% when Elemental Burst hits opponents, there is a 100% chance of summoning a titanic tuna that charges and deals 100/125/150/175/200% ATK as AoE DMG. This effect can occur once every 15s."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Luxurious Sea Lord"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(LuxuriousSeaLordEffect::new()))
    }
}
