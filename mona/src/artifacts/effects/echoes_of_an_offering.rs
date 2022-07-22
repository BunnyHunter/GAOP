use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct EchoesOfAnOfferingEffect {
    pub rate: f64,
}

const AVG_TRIGGER: f64 = 1.978911232;

impl<A: Attribute> ArtifactEffect<A> for EchoesOfAnOfferingEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.add_atk_percentage("Echoes Of An Offering 2 Piece Effect", 0.18);
    }

    fn effect4(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ATKRatioNormalAttack, "Echoes Of An Offering 4 Piece Effect", self.rate * 0.7);
    }
}

pub struct EchoesOfAnOffering;

impl ArtifactTrait for EchoesOfAnOffering {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        let rate = config.config_echoes_of_an_offering.rate;
        Box::new(EchoesOfAnOfferingEffect {
            rate
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::EchoesOfAnOffering,
        name_mona: "EchoesOfAnOffering",
        chs: "Echoes Of An Offering ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (4, 5),
        effect1: None,
        effect2: Some(" ATK +18%. "),
        effect3: None,
        effect4: Some("When Normal Attacks hit opponents, there is a 36% chance that it will trigger Valley Rite, which will increase Normal Attack DMG by 70% of ATK. This effect will be dispelled 0.05s after a Normal Attack deals DMG. If a Normal Attack fails to trigger Valley Rite, the odds of it triggering the next time will increase by 20%. This trigger can occur once every 0.2s. "),
        effect5: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "Average Trigger Rate",
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 / AVG_TRIGGER }
        }
    ]); 
}