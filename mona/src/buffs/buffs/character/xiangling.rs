use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffXianglingTalent2;

impl<A: Attribute> Buff<A> for BuffXianglingTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_atk_percentage("BUFF: Xiangling - Beware, It's Super Hot!", 0.1);
    }
}

impl BuffMeta for BuffXianglingTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::XianglingTalent2,
        chs: "Xiangling - Beware, It's Super Hot! ",
        image: BuffImage::Avatar(CharacterName::Xiangling),
        genre: BuffGenre::Character,
        description: Some("Xiangling's 2nd Ascension Talent: When Guoba Attack's effects end, Guoba leaves a chili pepper on the spot where it disappeared. Picking up a chili pepper increases ATK by 10% for 10s."),
        from: BuffFrom::Character(CharacterName::Xiangling),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffXianglingTalent2)
    }
}


pub struct BuffXianglingC1;

impl<A: Attribute> Buff<A> for BuffXianglingC1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusPyro, "BUFF: Xiangling - Crispy Outside, Tender Inside ", 0.15);
    }
}

impl BuffMeta for BuffXianglingC1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::XianglingC1,
        chs: "Xiangling - Crispy Outside, Tender Inside ",
        image: BuffImage::Avatar(CharacterName::Xiangling),
        genre: BuffGenre::Character,
        description: Some("Xiangling's 1st Constellation: Opponents hit by Guoba's attacks have their Pyro RES reduced by 15% for 6s."),
        from: BuffFrom::Character(CharacterName::Xiangling),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffXianglingC1)
    }
}


pub struct BuffXianglingC6;

impl<A: Attribute> Buff<A> for BuffXianglingC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusPyro, "BUFF: Xiangling - Condensed Pyronado", 0.15);
    }
}

impl BuffMeta for BuffXianglingC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::XianglingC6,
        chs: "Xiangling - Condensed Pyronado ",
        image: BuffImage::Avatar(CharacterName::Xiangling),
        genre: BuffGenre::Character,
        description: Some("Xiangling's 6th Constellation: For the duration of Pyronado, all party members receive a 15% Pyro DMG Bonus. "),
        from: BuffFrom::Character(CharacterName::Xiangling),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffXianglingC6)
    }
}