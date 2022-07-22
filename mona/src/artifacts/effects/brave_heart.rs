use serde::__private::de::IdentifierDeserializer;
use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct BraveHeartEffect {
    pub rate: f64,
}

impl BraveHeartEffect {
    pub fn new(config: &ArtifactEffectConfig) -> BraveHeartEffect {
        BraveHeartEffect {
            rate: config.config_brave_heart.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for BraveHeartEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_atk_percentage("Brave Heart 2 Piece Effect", 0.18);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusBase, "Brave Heart 4 Piece Effect", self.rate * 0.3);
    }
}

pub struct BraveHeart;

impl ArtifactTrait for BraveHeart {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(BraveHeartEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::BraveHeart,
        name_mona: "braveHeart",
        chs: "Brave Heart ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (3, 4),
        effect1: None,
        effect2: Some("ATK +18%. "),
        effect3: None,
        effect4: Some("Increases DMG by 30% against enemies with more than 50% HP. "),
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
