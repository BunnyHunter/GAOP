use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct InstructorEffect {
    pub rate: f64,
}

impl InstructorEffect {
    pub fn new(config: &ArtifactEffectConfig) -> InstructorEffect {
        InstructorEffect {
            rate: config.config_instructor.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for InstructorEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::ElementalMastery, "Instructor 2 Piece Effect", 80.0);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::ElementalMastery, "Instructor 4 Piece Effect ", self.rate * 120.0);
    }
}

pub struct Instructor;

impl ArtifactTrait for Instructor {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(InstructorEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::Instructor,
        name_mona: "instructor",
        chs: "Instructor ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (3, 4),
        effect1: None,
        effect2: Some("Increases Elemental Mastery by 80. "),
        effect3: None,
        effect4: Some("Upon triggering an Elemental Reaction, increases all party members' Elemental Mastery by 120 for 8s. "),
        effect5: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "Effect Application Ratio",
            config: ItemConfig::RATE01_TYPE
        }
    ]);
}
