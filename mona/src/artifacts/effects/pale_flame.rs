use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct PaleFlameEffect {
    pub level: f64,
    pub full_rate: f64,
}

impl PaleFlameEffect {
    pub fn new(config: &ArtifactEffectConfig) -> PaleFlameEffect {
        PaleFlameEffect {
            level: config.config_pale_flame.avg_level,
            full_rate: config.config_pale_flame.full_rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for PaleFlameEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusPhysical, "Pale Flame 2 Piece Effect", 0.25);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.add_atk_percentage("Pale Flame 4 Piece Effect", 0.09 * self.level);
        attribute.set_value_by(AttributeName::BonusPhysical, "Pale Flame 4 Piece Effect", 0.25 * self.full_rate);
    }
}

pub struct PaleFlame;

impl ArtifactTrait for PaleFlame {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(PaleFlameEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::PaleFlame,
        name_mona: "paleFlame",
        chs: "Pale Flame ",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (4, 5),
        effect1: None,
        effect2: Some("Physical DMG +25%. "),
        effect3: None,
        effect4: Some("When an Elemental Skill hits an opponent, ATK is increased by 9% for 7s. This effect stacks up to 2 times and can be triggered once every 0.3s. Once 2 stacks are reached, the 2-set effect is increased by 100%. "),
        effect5: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "avg_level",
            title: "Average Effect Stacks",
            config: ItemConfigType::Float { min: 0.0, max: 2.0, default: 0.0 }
        },
        ItemConfig {
            name: "full_rate",
            title: "Full Effect Stacks",
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        }
    ]);
}
