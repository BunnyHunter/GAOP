use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::super::effect::ArtifactEffect;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;

pub struct EmblemOfSeveredFateEffect;

impl<T: Attribute> ArtifactEffect<T> for EmblemOfSeveredFateEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::Recharge, "Emblem of Severed Fate 2 Piece Effect", 0.2);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.add_edge1(
            AttributeName::Recharge,
            AttributeName::BonusElementalBurst,
            Box::new(|x, _| (x * 0.25).min(0.75)),
            Box::new(|grad, x, _| (if x < 3.0 { grad * 0.25 } else { 0.0 }, 0.0)),
            "Emblem of Severed Fate 4 Piece Effect"
        );
    }
}

pub struct EmblemOfSeveredFate;

impl ArtifactTrait for EmblemOfSeveredFate {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(EmblemOfSeveredFateEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::EmblemOfSeveredFate,
        name_mona: "emblemOfSeveredFate",
        chs: "Emblem of Severed Fate ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (4, 5),
        effect1: None,
        effect2: Some("Energy Recharge +20%. "),
        effect3: None,
        effect4: Some("Increases Elemental Burst DMG by 25% of Energy Recharge. A maximum of 75% bonus DMG can be obtained in this way. "),
        effect5: None
    };
}
