use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct ShimenawasReminiscenceEffect {
    pub rate: f64,
}

impl ShimenawasReminiscenceEffect {
    pub fn new(config: &ArtifactEffectConfig) -> ShimenawasReminiscenceEffect {
        ShimenawasReminiscenceEffect {
            rate: config.config_shimenawas_reminiscence.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for ShimenawasReminiscenceEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_atk_percentage("Shimenawa's Reminiscence 2 Piece Effect", 0.18);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "Shimenawa's Reminiscence 4 Piece Effect", self.rate * 0.5);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "Shimenawa's Reminiscence 4 Piece Effect", self.rate * 0.5);
        attribute.set_value_by(AttributeName::BonusPlungingAttack, "Shimenawa's Reminiscence 4 Piece Effect", self.rate * 0.5);
    }
}

pub struct ShimenawasReminiscence;

impl ArtifactTrait for ShimenawasReminiscence {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ShimenawasReminiscenceEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::ShimenawasReminiscence,
        name_mona: "shimenawaReminiscence",
        chs: "Shimenawa's Reminiscence ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (4, 5),
        effect1: None,
        effect2: Some("ATK +18%. "),
        effect3: None,
        effect4: Some("When casting an Elemental Skill, if the character has 15 or more Energy, they lose 15 Energy and Normal/Charged/Plunging Attack DMG is increased by 50% for 10s. "),
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
