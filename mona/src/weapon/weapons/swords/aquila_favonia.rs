use crate::attribute::{Attribute, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct AquilaFavoniaEffect;

impl AquilaFavoniaEffect {
    pub fn new() -> AquilaFavoniaEffect {
        AquilaFavoniaEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for AquilaFavoniaEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.05 + 0.15;
        attribute.add_atk_percentage("Aquila Favonia Passive", value);
    }
}

pub struct AquilaFavonia;

impl WeaponTrait for AquilaFavonia {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::AquilaFavonia,
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::PhysicalBonus90),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        chs: "Aquila Favonia",
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Falcon of The West’s Resistance: ATK is increased by 20/25/30/35/40%. Triggers on taking DMG: the soul of the Falcon of the West awakens, holding the banner of the resistance aloft, regenerating HP equal to 100/115/130/145/160% of ATK and dealing 200/230/260/290/320% of ATK as DMG to surrounding opponents. This effect can only occur once every 15s.")
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(AquilaFavoniaEffect::new()))
    }
}
