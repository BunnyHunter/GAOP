use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;

pub struct WanderersTroupeEffect {
    pub weapon_type: WeaponType
}

impl WanderersTroupeEffect {
    pub fn new(weapon_type: WeaponType) -> WanderersTroupeEffect {
        WanderersTroupeEffect {
            weapon_type,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for WanderersTroupeEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::ElementalMastery, "Wanderer's Troupe 2 Piece Effect", 80.0);
    }

    fn effect4(&self, attribute: &mut T) {
        match self.weapon_type {
            WeaponType::Catalyst | WeaponType::Bow => {
                attribute.set_value_by(AttributeName::BonusChargedAttack, "Wanderer's Troupe 4 Piece Effect", 0.35);
            },
            _ => (),
        }
    }
}

pub struct WanderersTroupe;

impl ArtifactTrait for WanderersTroupe {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(WanderersTroupeEffect::new(character_common_data.static_data.weapon_type))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::WanderersTroupe,
        name_mona: "wandererTroupe",
        chs: "Wanderer's Troupe",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (4, 5),
        effect1: None,
        effect2: Some("Elemental Mastery +80. "),
        effect3: None,
        effect4: Some("Increases Charged Attack DMG by 35% if the character uses a Catalyst or Bow. "),
        effect5: None
    };
}
