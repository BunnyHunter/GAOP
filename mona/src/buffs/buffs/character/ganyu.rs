use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffGanyuTalent2;

impl<A: Attribute> Buff<A> for BuffGanyuTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusCryo, "BUFF: Ganyu - Harmony between Heaven and Earth", 0.2);
    }
}

impl BuffMeta for BuffGanyuTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::GanyuTalent2,
        chs: "Ganyu - Harmony between Heaven and Earth ",
        image: BuffImage::Avatar(CharacterName::Ganyu),
        genre: BuffGenre::Character,
        description: Some("Ganyu's 2nd Ascension Talent: Celestial Shower grants a 20% Cryo DMG Bonus to active members in the AoE."),
        from: BuffFrom::Character(CharacterName::Ganyu)
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffGanyuTalent2)
    }
}

pub struct BuffGanyuC1;

impl<A: Attribute> Buff<A> for BuffGanyuC1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusCryo, "BUFF: 甘雨1命「饮露」", 0.15);
    }
}

impl BuffMeta for BuffGanyuC1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::GanyuC1,
        chs: "Ganyu - Dew-Drinker",
        image: BuffImage::Avatar(CharacterName::Ganyu),
        genre: BuffGenre::Character,
        description: Some("Ganyu's 1st Constellation: Taking DMG from a Charge Level 2 Frostflake Arrow or Frostflake Arrow Bloom decreases opponents' Cryo RES by 15% for 6s."),
        from: BuffFrom::Character(CharacterName::Ganyu),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffGanyuC1)
    }
}
