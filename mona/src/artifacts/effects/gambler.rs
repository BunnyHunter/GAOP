use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;

pub struct GamblerEffect;

impl<T: Attribute> ArtifactEffect<T> for GamblerEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusElementalSkill, "Gambler 2 Piece Effect", 0.2);
    }
}

pub struct Gambler;

impl ArtifactTrait for Gambler {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(GamblerEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::Gambler,
        name_mona: "gambler",
        chs: "Gambler ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (3, 4),
        effect1: None,
        effect2: Some("Elemental Skill DMG increased by 20%. "),
        effect3: None,
        effect4: Some("Defeating an enemy has 100% chance to remove Elemental Skill CD. Can only occur once every 15s. "),
        effect5: None,
    };
}
