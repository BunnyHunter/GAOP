use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct VermillionHereafterEffect {
    pub rate_q: f64,
    pub stack: f64,
}

impl<A: Attribute> ArtifactEffect<A> for VermillionHereafterEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.add_atk_percentage("Vermillion Hereafter 2 Piece Effect", 0.18);
    }

    fn effect4(&self, attribute: &mut A) {
        let bonus = self.rate_q * (0.08 + 0.1 * self.stack);
        attribute.add_atk_percentage("Vermillion Hereafter 4 Piece Effect", bonus);
    }
}

pub struct VermillionHereafter;

impl ArtifactTrait for VermillionHereafter {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        let rate_q = config.config_vermillion_hereafter.rate_q;
        let stack = config.config_vermillion_hereafter.stack;

        Box::new(VermillionHereafterEffect {
            rate_q, stack
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::VermillionHereafter,
        name_mona: "VermillionHereafter",
        chs: "Vermillion Hereafter ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (4, 5),
        effect1: None,
        effect2: Some("ATK +18%. "),
        effect3: None,
        effect4: Some("After using an Elemental Burst, this character will gain the Nascent Light effect, increasing their ATK by 8% for 16s. When the character's HP decreases, their ATK will further increase by 10%. This further increase can occur this way a maximum of 4 times. This effect can be triggered once every 0.8s. Nascent Light will be dispelled when the character leaves the field. If an Elemental Burst is used again during the duration of Nascent Light, the original Nascent Light will be dispelled. "),
        effect5: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate_q",
            title: "Elemental Burst Rate",
            config: ItemConfig::RATE01_TYPE
        },
        ItemConfig {
            name: "stack",
            title: "Average Number of Stacks",
            config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 0.0 }
        }
    ]);
}