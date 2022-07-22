use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BerserkerEffect {
    pub rate: f64,
}

impl BerserkerEffect {
    pub fn new(config: &ArtifactEffectConfig) -> BerserkerEffect {
        BerserkerEffect {
            rate: config.config_berserker.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for BerserkerEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::CriticalBase, "Berserker 2 Piece Effect", 0.12);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::CriticalBase, "Berserker 4 Piece Effect", self.rate * 0.24);
    }
}

pub struct Berserker;

impl ArtifactTrait for Berserker {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(BerserkerEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::Berserker,
        name_mona: "berserker",
        chs: "Berserker",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (3, 4),
        effect1: None,
        effect2: Some("CRIT Rate +12%."),
        effect3: None,
        effect4: Some("When HP is below 70%, CRIT Rate increases by an additional 24%."),
        effect5: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "Effect Application Ratio",
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);
}
