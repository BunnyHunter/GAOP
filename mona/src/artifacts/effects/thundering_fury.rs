use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;

pub struct ThunderingFuryEffect;

impl<T: Attribute> ArtifactEffect<T> for ThunderingFuryEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusElectro, "Thundering Fury 2 Piece Effect", 0.15);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::EnhanceOverload, "Thundering Fury 4 Piece Effect", 0.4);
        attribute.set_value_by(AttributeName::EnhanceElectroCharged, "Thundering Fury 4 Piece Effect", 0.4);
        attribute.set_value_by(AttributeName::EnhanceSuperconduct, "Thundering Fury 4 Piece Effect", 0.4);
    }
}

pub struct ThunderingFury;

impl ArtifactTrait for ThunderingFury {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ThunderingFuryEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::ThunderingFury,
        name_mona: "thunderingFury",
        chs: "Thundering Fury ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (4, 5),
        effect1: None,
        effect2: Some("Gain a 15% Electro DMG Bonus. "),
        effect3: None,
        effect4: Some("Increases damage caused by Overloaded, Electro-Charged, and Superconduct DMG by 40%. Triggering such effects decreases Elemental Skill CD by 1s. Can only occur once every 0.8s. "),
        effect5: None
    };
}
