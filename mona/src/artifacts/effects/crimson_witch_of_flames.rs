use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct CrimsonWitchOfFlamesEffect {
    pub level: f64,
}

impl CrimsonWitchOfFlamesEffect {
    pub fn new(config: &ArtifactEffectConfig) -> CrimsonWitchOfFlamesEffect {
        CrimsonWitchOfFlamesEffect {
            level: config.config_crimson_witch_of_flames.level.min(3.0),
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for CrimsonWitchOfFlamesEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusPyro, "Crimson Witch of Flames 2 Piece Effect", 0.15);
    }

    fn effect4(&self, attribute: &mut T) {
        let key = "Crimson Witch of Flames 4 Piece Effect";
        attribute.set_value_by(AttributeName::EnhanceOverload, key, 0.4);
        attribute.set_value_by(AttributeName::EnhanceBurning, key, 0.4);
        attribute.set_value_by(AttributeName::EnhanceVaporize, key, 0.15);
        attribute.set_value_by(AttributeName::EnhanceMelt, key, 0.15);
        attribute.set_value_by(AttributeName::BonusPyro, key, self.level * 0.075);
    }
}

pub struct CrimsonWitchOfFlames;

impl ArtifactTrait for CrimsonWitchOfFlames {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(CrimsonWitchOfFlamesEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::CrimsonWitchOfFlames,
        name_mona: "crimsonWitch",
        chs: "Crimson Witch of Flames ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (4, 5),
        effect1: None,
        effect2: Some("Gain a 15% Pyro DMG Bonus. "),
        effect3: None,
        effect4: Some("Increases Overloaded and Burning DMG by 40%. Increases Vaporize and Melt DMG by 15%. Using an Elemental Skill increases 2-Piece Set effects by 50% for 10s. Max 3 stacks. "),
        effect5: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "level",
            title: "Stacks",
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 0.0 }
        }
    ]);
}
