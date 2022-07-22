use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffResonancePyro2;

impl<A: Attribute> Buff<A> for BuffResonancePyro2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_atk_percentage("BUFF: Pyro Elemental Resonance - Fervent Flames", 0.25);
    }
}

impl BuffMeta for BuffResonancePyro2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ResonancePyro2,
        chs: "Pyro Elemental Resonance - Fervent Flames",
        image: BuffImage::Misc("pyro"),
        genre: BuffGenre::Resonance,
        description: Some("Affected by Cryo for 40% less time. Increases ATK by 25%."),
        from: BuffFrom::Resonance,
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffResonancePyro2)
    }
}


pub struct BuffResonanceCryo2 {
    pub rate: f64
}

impl<A: Attribute> Buff<A> for BuffResonanceCryo2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::CriticalBase, "Cryo Elemental Resonance - Shattering Ice ", self.rate * 0.15);
    }
}

impl BuffMeta for BuffResonanceCryo2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ResonanceCryo2,
        chs: "Cryo Elemental Resonance - Shattering Ice ",
        image: BuffImage::Misc("cryo"),
        genre: BuffGenre::Resonance,
        description: Some("Affected by Electro for 40% less time. Increases CRIT Rate against enemies that are Frozen or affected by Cryo by 15%."),
        from: BuffFrom::Resonance,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "Application Ratio",
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let rate = match *b {
            BuffConfig::ResonanceCryo2 { rate } => rate,
            _ => 0.0
        };

        Box::new(BuffResonanceCryo2 {
            rate
        })
    }
}


pub struct BuffResonanceGeo2 {
    pub rate1: f64,
    pub rate2: f64
}

impl<A: Attribute> Buff<A> for BuffResonanceGeo2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ShieldStrength, "Geo Elemental Resonance - Enduring Rock ", 0.15);
        attribute.set_value_by(AttributeName::BonusBase, "Geo Elemental Resonance - Enduring Rock ", self.rate1 * 0.15);
        attribute.set_value_by(AttributeName::ResMinusGeo, "Geo Elemental Resonance - Enduring Rock ", self.rate2 * 0.2);
    }
}

impl BuffMeta for BuffResonanceGeo2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ResonanceGeo2,
        chs: "Geo Elemental Resonance - Enduring Rock ",
        image: BuffImage::Misc("geo"),
        genre: BuffGenre::Resonance,
        description: Some("Increases shield strength by 15%. Additionally, characters protected by a shield will have the following special characteristics: DMG dealt increased by 15%, dealing DMG to enemies will decrease their Geo RES by 20% for 15s."),
        from: BuffFrom::Common,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate1",
            title: "1st Effect Proportion",
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        },
        ItemConfig {
            name: "rate2",
            title: "2st Effect Proportion",
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (rate1, rate2) = match *b {
            BuffConfig::ResonanceGeo2 { rate1, rate2 } => (rate1, rate2),
            _ => (0.0, 0.0)
        };

        Box::new(BuffResonanceGeo2 {
            rate1, rate2
        })
    }
}
