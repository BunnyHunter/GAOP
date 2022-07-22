use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct BloodstainedChivalryEffect {
    pub rate: f64,
}

impl BloodstainedChivalryEffect {
    pub fn new(config: &ArtifactEffectConfig) -> BloodstainedChivalryEffect {
        BloodstainedChivalryEffect {
            rate: config.config_bloodstained_chivalry.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for BloodstainedChivalryEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusPhysical, "Bloodstained Chivalry 2 Piece Effect", 0.25);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusChargedAttack, "Bloodstained Chivalry 4 Piece Effect", self.rate * 0.5);
    }
}

pub struct BloodstainedChivalry;

impl ArtifactTrait for BloodstainedChivalry {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(BloodstainedChivalryEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::BloodstainedChivalry,
        name_mona: "bloodstainedChivalry",
        chs: "Bloodstained Chivalry ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (4, 5),
        effect1: None,
        effect2: Some("Physical DMG +25%."),
        effect3: None,
        effect4: Some("After defeating an opponent, increases Charged Attack DMG by 50%, and reduces its Stamina cost to 0 for 10s."),
        effect5: None,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "Effect Application Ratio",
            config: ItemConfig::RATE01_TYPE
        }
    ]);
}
