use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;

pub struct ResolutionOfSojournerEffect;

impl<T: Attribute> ArtifactEffect<T> for ResolutionOfSojournerEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_atk_percentage("Resolution of Sojourner 2 Piece Effect", 0.18);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::CriticalChargedAttack, "Resolution of Sojourner 4 Piece Effect", 0.3);
    }
}

pub struct ResolutionOfSojourner;

impl ArtifactTrait for ResolutionOfSojourner {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ResolutionOfSojournerEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::ResolutionOfSojourner,
        name_mona: "resolutionOfSojourner",
        chs: "Resolution of Sojourner ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (3, 4),
        effect1: None,
        effect2: Some("ATK +18%. "),
        effect3: None,
        effect4: Some("Increases Charged Attack CRIT Rate by 30%. "),
        effect5: None
    };
}
