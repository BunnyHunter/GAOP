use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct NoblesseObligeEffect {
    pub rate: f64,
}

impl NoblesseObligeEffect {
    pub fn new(config: &ArtifactEffectConfig) -> NoblesseObligeEffect {
        NoblesseObligeEffect {
            rate: config.config_noblesse_oblige.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for NoblesseObligeEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusElementalBurst, "Noblesse Oblige 2 Piece Effect", 0.2);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.add_atk_percentage("Noblesse Oblige 4 Piece Effect", self.rate * 0.2);
        attribute.set_value_by(AttributeName::ATKBonusForOther, "Noblesse Oblige 4 Piece Effect", self.rate * 0.2);
    }
}

pub struct NoblesseOblige;

impl ArtifactTrait for NoblesseOblige {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(NoblesseObligeEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::NoblesseOblige,
        name_mona: "noblesseOblige",
        chs: "Noblesse Oblige",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (4, 5),
        effect1: None,
        effect2: Some("Elemental Burst DMG +20%"),
        effect3: None,
        effect4: Some("Using an Elemental Burst increases all party members' ATK by 20% for 12s. This effect cannot stack."),
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
