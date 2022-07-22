use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;

pub struct DefendersWillEffect;

impl<T: Attribute> ArtifactEffect<T> for DefendersWillEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_def_percentage("Defender's Will 2 Piece Effect", 0.3);
    }

    // fn effect4(&self, attribute: &mut AttributeGraph) {
    //     attribute.add_value(AttributeName::CriticalBase, "战狂4", self.rate * 0.24);
    // }
}

pub struct DefendersWill;

impl ArtifactTrait for DefendersWill {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(DefendersWillEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::DefendersWill,
        name_mona: "defenderWill",
        chs: "Defender's Will ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (0, 0),
        effect1: None,
        effect2: Some("Base DEF +30%. "),
        effect3: None,
        effect4: Some("Increases Elemental RES by 30% for each element present in the party. "),
        effect5: None
    };
}
