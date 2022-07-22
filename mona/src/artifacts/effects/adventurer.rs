use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;

pub struct AdventurerEffect;

impl<T: Attribute> ArtifactEffect<T> for AdventurerEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::HPFixed, "Adventurer 2 Piece Effect", 1000.0);
    }
}

pub struct Adventurer;

impl ArtifactTrait for Adventurer {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(AdventurerEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::Adventurer,
        name_mona: "adventurer",
        chs: "Adventurer",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (1, 3),
        effect1: None,
        effect2: Some("Max HP increased by 1,000."),
        effect3: None,
        effect4: Some("Opening chest regenerates 30% Max HP over 5s."),
        effect5: None,
    };
}