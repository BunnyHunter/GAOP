use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;

pub struct TheExileEffect;

impl<T: Attribute> ArtifactEffect<T> for TheExileEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::Recharge, "The Exile 2 Piece Effect", 0.2);
    }
}

pub struct TheExile;

impl ArtifactTrait for TheExile {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(TheExileEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::TheExile,
        name_mona: "exile",
        chs: "The Exile ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (3, 4),
        effect1: None,
        effect2: Some("Energy Recharge +20%. "),
        effect3: None,
        effect4: Some("Using an Elemental Burst regenerates 2 Energy for other party members every 2s for 6s. This effect cannot stack. "),
        effect5: None
    };
}
