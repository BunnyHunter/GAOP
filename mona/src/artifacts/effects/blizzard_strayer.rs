use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BlizzardStrayerEffect {
    pub crit_bonus: f64,
}

impl BlizzardStrayerEffect {
    pub fn new(config: &ArtifactEffectConfig) -> BlizzardStrayerEffect {
        BlizzardStrayerEffect {
            crit_bonus: config.config_blizzard_strayer.critical_bonus
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for BlizzardStrayerEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusCryo, "Blizzard Strayer 2 Piece Effect", 0.15);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::CriticalAttacking, "Blizzard Strayer 2 Piece Effect", self.crit_bonus);
    }
}

pub struct BlizzardStrayer;

impl ArtifactTrait for BlizzardStrayer {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(BlizzardStrayerEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::BlizzardStrayer,
        name_mona: "blizzardStrayer",
        chs: "Blizzard Strayer",
        flower: Some("Flower of Life"),
        feather: Some("Plume of Death"),
        sand: Some("Sands of Eon"),
        goblet: Some("Goblet"),
        head: Some("Circlet"),
        star: (4, 5),
        effect1: None,
        effect2: Some("Cryo DMG Bonus +15%."),
        effect3: None,
        effect4: Some("When a character attacks an enemy affected by Cryo, their CRIT Rate is increased by 20%. If the enemy is Frozen, CRIT Rate is increased by an additional 20%."),
        effect5: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "critical_bonus",
            title: "Equivalent Crit Chance",
            config: ItemConfigType::Float { min: 0.0, max: 0.4, default: 0.0 }
        }
    ]);
}
