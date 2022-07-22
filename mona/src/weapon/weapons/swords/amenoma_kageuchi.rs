use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::{WeaponType};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct AmenomaKageuchi;

impl WeaponTrait for AmenomaKageuchi {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::AmenomaKageuchi,
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK120),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("Iwakura Sucession: After casting an Elemental Skill, gain 1 Succession Seed. This effect can be<br>triggered once every 5s. The Succession Seed lasts for 30s. Up to 3 Succession Seeds may exist<br>simultaneously. After using an Elemental Burst, all Succession Seeds are consumed and after 2s, the<br>regenerates 6/7.5/9/10.5/12 Energy for each seed consumed."),
        #[cfg(not(target_family = "wasm"))]
        chs: "Amenoma Kageuchi"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
