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

pub struct SkywardAtlasEffect;

impl SkywardAtlasEffect {
    pub fn new() -> SkywardAtlasEffect {
        SkywardAtlasEffect {}
    }
}

impl<T: Attribute> WeaponEffect<T> for SkywardAtlasEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = data.refine as f64 * 0.03 + 0.09;
        attribute.add_elemental_bonus("Skyward Atlas Passive", value);
    }
}

pub struct SkywardAtlas;

impl WeaponTrait for SkywardAtlas {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SkywardAtlas,
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK72),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Wandering Clouds: Increases Elemental DMG Bonus by 12/15/18/21/24% . Normal Attack hits have a 50% chance to earn the favor of the clouds. which actively seek out nearby opponents to attack for 15s, dealing 160/200/240/280/320% ATK DMG.<br>Can only occur once every 30s."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Skyward Atlas"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(SkywardAtlasEffect::new()))
    }
}
