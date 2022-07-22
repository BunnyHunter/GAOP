use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::{ArtifactEffect, ArtifactEffectNone};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;

pub struct PrayersToSpringtime;

impl ArtifactTrait for PrayersToSpringtime {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ArtifactEffectNone)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::PrayersToSpringtime,
        name_mona: "prayersToSpringtime",
        chs: "Prayers To Springtime",
        flower: None,
        feather: None,
        sand: None,
        goblet: None,
        head: Some("Circlet"),
        star: (3, 4),
        effect1: Some("Affected by Cryo for 40% less time. "),
        effect2: None,
        effect3: None,
        effect4: None,
        effect5: None,
    };
}
