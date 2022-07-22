use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;

pub struct ViridescentVenererEffect;

impl<T: Attribute> ArtifactEffect<T> for ViridescentVenererEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusAnemo, "Viridescent Venerer 2 Piece Effect", 0.15);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::EnhanceSwirlBase, "Viridescent Venerer 4 Piece Effect", 0.6);
    }
}

pub struct ViridescentVenerer;

impl ArtifactTrait for ViridescentVenerer {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ViridescentVenererEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::ViridescentVenerer,
        name_mona: "viridescentVenerer",
        chs: "Viridescent Venerer ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (4, 5),
        effect1: None,
        effect2: Some("Gain a 15% Anemo DMG Bonus. "),
        effect3: None,
        effect4: Some("Increases Swirl DMG by 60%. Decreases opponent's Elemental RES to the element infused in the Swirl by 40% for 10s. "),
        effect5: None
    };
}
