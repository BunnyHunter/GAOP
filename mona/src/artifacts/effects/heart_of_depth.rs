use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct HeartOfDepthEffect {
    pub rate: f64,
}

impl HeartOfDepthEffect {
    pub fn new(config: &ArtifactEffectConfig) -> HeartOfDepthEffect {
        HeartOfDepthEffect {
            rate: config.config_heart_of_depth.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for HeartOfDepthEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusHydro, "Heart of Depth 2 Piece Effect", 0.15);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "Heart of Depth 4 Piece Effect", self.rate * 0.3);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "Heart of Depth 4 Piece Effect", self.rate * 0.3);
    }
}

pub struct HeartOfDepth;

impl ArtifactTrait for HeartOfDepth {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(HeartOfDepthEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::HeartOfDepth,
        name_mona: "heartOfDepth",
        chs: "Heart of Depth ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (4, 5),
        effect1: None,
        effect2: Some("Hydro DMG Bonus +15%. "),
        effect3: None,
        effect4: Some("After using Elemental Skill, increases Normal Attack and Charged Attack DMG by 30% for 15s. "),
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
