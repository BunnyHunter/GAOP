use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::character::Character;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;

pub struct GladiatorsFinaleEffect {
    weapon_type: WeaponType,
}

impl GladiatorsFinaleEffect {
    pub fn new(weapon_type: WeaponType) -> GladiatorsFinaleEffect {
        GladiatorsFinaleEffect {
            weapon_type
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for GladiatorsFinaleEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_atk_percentage("Gladiator's Finale 2 Piece Effect", 0.18);
    }

    fn effect4(&self, attribute: &mut T) {
        match self.weapon_type {
            WeaponType::Sword | WeaponType::Claymore | WeaponType::Polearm => {
                attribute.set_value_by(AttributeName::BonusNormalAttack, "Gladiator's Finale 4 Piece Effect", 0.35);
            },
            _ => (),
        };
    }
}

pub struct GladiatorsFinale;

impl ArtifactTrait for GladiatorsFinale {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(GladiatorsFinaleEffect::new(character_common_data.static_data.weapon_type))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::GladiatorsFinale,
        name_mona: "gladiatorFinale",
        chs: "Gladiator's Finale " ,
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (4, 5),
        effect1: None,
        effect2: Some("ATK +18%. "),
        effect3: None,
        effect4: Some("If the wielder of this artifact set uses a Sword, Claymore or Polearm, increases their Normal Attack DMG by 35%. "),
        effect5: None
    };
}
