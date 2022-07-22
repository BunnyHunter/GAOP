use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::Element;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct ArchaicPetraEffect {
    pub element: Element,
    pub rate: f64,
}

impl ArchaicPetraEffect {
    pub fn new(config: &ArtifactEffectConfig) -> ArchaicPetraEffect {
        ArchaicPetraEffect {
            element: config.config_archaic_petra.element,
            rate: config.config_archaic_petra.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for ArchaicPetraEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusGeo, "Archaic Petra 2 Piece Effect", 0.15);
    }

    fn effect4(&self, attribute: &mut T) {
        let attribute_name = AttributeName::bonus_name_by_element(self.element);

        attribute.set_value_by(attribute_name, "Archaic Petra 4 Piece Effect ", self.rate * 0.35)
    }
}

pub struct ArchaicPetra;

impl ArtifactTrait for ArchaicPetra {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ArchaicPetraEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::ArchaicPetra,
        name_mona: "archaicPetra",
        chs: "Archaic Petra",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (4, 5),
        effect1: None,
        effect2: Some("Gain a 15% Geo DMG Bonus."),
        effect3: None,
        effect4: Some("Upon obtaining an Elemental Shard created through a Crystallize Reaction, all party members gain 35% DMG Bonus for that particular element for 10s. Only one form of Elemental DMG Bonus can be gained in this manner at any one time."),
        effect5: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "element",
            title: "Element",
            config: ItemConfigType::Element4 { default: Element::Electro }
        },
        ItemConfig {
            name: "rate",
            title: "Aplication Ratio",
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);
}
