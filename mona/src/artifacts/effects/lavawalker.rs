use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct LavawalkerEffect {
    pub rate: f64,
}

impl LavawalkerEffect {
    pub fn new(config: &ArtifactEffectConfig) -> LavawalkerEffect {
        LavawalkerEffect {
            rate: config.config_lavawalker.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for LavawalkerEffect {
    // fn effect2(&self, attribute: &mut AttributeGraph) {
    //     attribute.add_value(AttributeName::CriticalBase, "战狂2", 0.12);
    // }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusBase, "Lavawalker 4 Piece Effect", self.rate * 0.35);
    }
}

pub struct Lavawalker;

impl ArtifactTrait for Lavawalker {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(LavawalkerEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::Lavawalker,
        name_mona: "lavaWalker",
        chs: "Lavawalker ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (4, 5),
        effect1: None,
        effect2: Some("Pyro RES increased by 40%. "),
        effect3: None,
        effect4: Some("Increases DMG against enemies that are Burning or affected by Pyro by 35%. "),
        effect5: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "Pyro Uptime",
            config: ItemConfig::RATE01_TYPE
        }
    ]);
}
