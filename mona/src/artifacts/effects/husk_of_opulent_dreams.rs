use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct HuskOfOpulentDreamsEffect {
    pub level: f64,
}

impl HuskOfOpulentDreamsEffect {
    pub fn new(config: &ArtifactEffectConfig) -> HuskOfOpulentDreamsEffect {
        HuskOfOpulentDreamsEffect {
            level: config.config_husk_of_opulent_dreams.level,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for HuskOfOpulentDreamsEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_def_percentage("Husk of Opulent Dreams 2 Piece Effect", 0.3);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.add_def_percentage("Husk of Opulent Dreams 4 Piece Effect", 0.06 * self.level);
        attribute.set_value_by(AttributeName::BonusGeo, "Husk of Opulent Dreams 4 Piece Effect", self.level * 0.06);
    }
}

pub struct HuskOfOpulentDreams;

impl ArtifactTrait for HuskOfOpulentDreams {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(HuskOfOpulentDreamsEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::HuskOfOpulentDreams,
        name_mona: "huskOfOpulentDreams",
        chs: "Husk of Opulent Dreams ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (4, 5),
        effect1: None,
        effect2: Some("DEF +30%. "),
        effect3: None,
        effect4: Some("A character equipped with this Artifact set will obtain the Curiosity effect in the following conditions: When on the field, the character gains 1 stack after hitting an opponent with a Geo attack, triggering a maximum of once every 0.3s. When off the field, the character gains 1 stack every 3s. Curiosity can stack up to 4 times, each providing 6% DEF and a 6% Geo DMG Bonus. When 6 seconds pass without gaining a Curiosity stack, 1 stack is lost. "),
        effect5: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "level",
            title: "Stacks",
            config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 0.0 }
        }
    ]);
}
