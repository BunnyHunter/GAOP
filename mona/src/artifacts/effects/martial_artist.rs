use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct MartialArtistEffect {
    pub rate: f64,
}

impl MartialArtistEffect {
    pub fn new(config: &ArtifactEffectConfig) -> MartialArtistEffect {
        MartialArtistEffect {
            rate: config.config_martial_artist.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for MartialArtistEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "Martial Artist 2 Piece Effect", 0.15);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "Martial Artist 2 Piece Effect", 0.15);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "Martial Artist 4 Piece Effect", self.rate * 0.25);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "Martial Artist 4 Piece Effect", self.rate * 0.25);
    }
}

pub struct MartialArtist;

impl ArtifactTrait for MartialArtist {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(MartialArtistEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::MartialArtist,
        name_mona: "martialArtist",
        chs: "Martial Artist ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (3, 4),
        effect1: None,
        effect2: Some("Increases Normal Attack and Charged Attack DMG by 15%. "),
        effect3: None,
        effect4: Some("After using Elemental Skill, increases Normal Attack and Charged Attack DMG by 25% for 8s. "),
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
