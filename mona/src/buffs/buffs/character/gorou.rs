use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::characters::Gorou;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffGorouE1 {
    pub skill2: usize,
}

impl<A: Attribute> Buff<A> for BuffGorouE1 {
    fn change_attribute(&self, attribute: &mut A) {
        let bonus = Gorou::SKILL.elemental_skill_def_bonus[self.skill2 - 1];
        attribute.set_value_by(AttributeName::DEFFixed, "BUFF: Gorou - Inuzaka All-Round Defense ", bonus);
    }
}

impl BuffMeta for BuffGorouE1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::GorouE1,
        chs: "Gorou - Inuzaka All-Round Defense ",
        image: BuffImage::Avatar(CharacterName::Gorou),
        genre: BuffGenre::Character,
        description: Some("Gorou Elemental Skill: 1 Geo character: Adds 'Standing Firm' - DEF Bonus."),
        from: BuffFrom::Character(CharacterName::Gorou),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "skill2",
            title: "Gorou Elemental Skill Level",
            config: ItemConfigType::Int { min: 1, max: 15, default: 10 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let skill2 = match *b {
            BuffConfig::GorouE1 { skill2 } => skill2,
            _ => 1
        };

        Box::new(BuffGorouE1 {
            skill2
        })
    }
}

pub struct BuffGorouE3;

impl<A: Attribute> Buff<A> for BuffGorouE3 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusGeo, "BUFF: Gorou - Inuzaka All-Round Defense -3 ", 0.15);
    }
}

impl BuffMeta for BuffGorouE3 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::GorouE3,
        chs: "Gorou - Inuzaka All-Round Defense -3 ",
        image: BuffImage::Avatar(CharacterName::Gorou),
        genre: BuffGenre::Character,
        description: Some("Gorou's Elemental Skill: 3 Geo characters: Adds 'Crunch' - Geo DMG Bonus."),
        from: BuffFrom::Character(CharacterName::Gorou),
    };

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffGorouE3)
    }
}

pub struct BuffGorouTalent1;

impl<A: Attribute> Buff<A> for BuffGorouTalent1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_def_percentage("BUFF: 五郎天赋「不畏Anemo雨」", 0.25);
    }
}

impl BuffMeta for BuffGorouTalent1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::GorouTalent1,
        chs: "Gorou - Heedless of the Wind and Weather ",
        image: BuffImage::Avatar(CharacterName::Gorou),
        genre: BuffGenre::Character,
        description: Some("Gorou's 1st Ascension Talent: After using Juuga: Forward Unto Victory, all nearby party members' DEF is increased by 25% for 12s."),
        from: BuffFrom::Character(CharacterName::Gorou),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffGorouTalent1)
    }
}

pub struct BuffGorouC6 {
    pub level: usize
}

impl<A: Attribute> Buff<A> for BuffGorouC6 {
    fn change_attribute(&self, attribute: &mut A) {
        let value = match self.level {
            1 => 0.1,
            2 => 0.2,
            _ => 0.4
        };

        attribute.set_value_by(AttributeName::CriticalDamageGeo, "BUFF：五郎六命「犬勇•忠如山」", value);
    }
}

impl BuffMeta for BuffGorouC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::GorouC6,
        chs: "Gorou - Valiant Hound: Mountainous Fealty ",
        image: BuffImage::Avatar(CharacterName::Gorou),
        genre: BuffGenre::Character,
        description: Some("Gorou's 5th Constellation: For 12s after using Inuzaka All-Round Defense or Juuga: Forward Unto Victory, increases the CRIT DMG of all nearby party members' Geo DMG based on the buff level of the skill's field at the time of use: 'Standing Firm':+10%, 'Impregnable': +20%, 'Crunch': +40%"),
        from: BuffFrom::Character(CharacterName::Gorou),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "level",
            title: "Field Level",
            config: ItemConfigType::Int { min: 1, max: 3, default: 1 },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let level = match *b {
            BuffConfig::GorouC6 { level } => level,
            _ => 1
        };
        Box::new(BuffGorouC6 {
            level
        })
    }
}
