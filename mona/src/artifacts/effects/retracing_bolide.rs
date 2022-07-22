use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct RetracingBolideEffect {
    pub rate: f64,
}

impl RetracingBolideEffect {
    pub fn new(config: &ArtifactEffectConfig) -> RetracingBolideEffect {
        RetracingBolideEffect {
            rate: config.config_retracing_bolide.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for RetracingBolideEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::ShieldStrength, "Retracing Bolide 2 Piece Effect", 0.35);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "Retracing Bolide 4 Piece Effect", self.rate * 0.4);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "Retracing Bolide 4 Piece Effect", self.rate * 0.4);
    }
}

pub struct RetracingBolide;

impl ArtifactTrait for RetracingBolide {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(RetracingBolideEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::RetracingBolide,
        name_mona: "retracingBolide",
        chs: "Retracing Bolide ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (4, 5),
        effect1: None,
        effect2: Some("Increases Shield Strength by 35%. "),
        effect3: None,
        effect4: Some("While protected by a shield, gain an additional 40% Normal and Charged Attack DMG."),
        effect5: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "Shield Uptime",
            config: ItemConfig::RATE01_TYPE
        }
    ]);
}
