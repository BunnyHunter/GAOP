use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct TenacityOfTheMillelithEffect {
    pub rate: f64,
}

impl TenacityOfTheMillelithEffect {
    pub fn new(config: &ArtifactEffectConfig) -> TenacityOfTheMillelithEffect {
        TenacityOfTheMillelithEffect {
            rate: config.config_tenacity_of_the_millelith.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for TenacityOfTheMillelithEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.add_hp_percentage("Tenacity of the Millelith 2 Piece Effect", 0.2);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.add_atk_percentage("Tenacity Of The Millelith 4 Piece Effect ", self.rate * 0.2);
        attribute.set_value_by(AttributeName::ShieldStrength, "Tenacity Of The Millelith 4 Piece Effect ", self.rate * 0.3);
        attribute.set_value_by(AttributeName::ATKBonusForOther, "Tenacity Of The Millelith 4 Piece Effect ", self.rate * 0.2);
    }
}

pub struct TenacityOfTheMillelith;

impl ArtifactTrait for TenacityOfTheMillelith {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(TenacityOfTheMillelithEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::TenacityOfTheMillelith,
        name_mona: "tenacityOfTheMillelith",
        chs: "Tenacity of the Millelith",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (4, 5),
        effect1: None,
        effect2: Some("HP +20%."),
        effect3: None,
        effect4: Some("When an Elemental Skill hits an opponent, the ATK of all nearby party members is increased by 20% and their Shield Strength is increased by 30% for 3s. This effect can be triggered once every 0.5s. This effect can still be triggered even when the character who is using this artifact set is not on the field. "),
        effect5: None
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
