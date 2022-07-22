use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;

pub struct LuckyDogEffect;

impl<T: Attribute> ArtifactEffect<T> for LuckyDogEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::DEFFixed, "Lucky Dog 2 Piece Effect", 100.0);
    }
}

pub struct LuckyDog;

impl ArtifactTrait for LuckyDog {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(LuckyDogEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::LuckyDog,
        name_mona: "luckyDog",
        chs: "Lucky Dog ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (1, 3),
        effect1: None,
        effect2: Some("DEF increased by 100. "),
        effect3: None,
        effect4: Some("Picking up Mora restores 300 HP. "),
        effect5: None
    };
}
