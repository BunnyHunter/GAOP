use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffXinyanC4;

impl<A: Attribute> Buff<A> for BuffXinyanC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusPhysical, "BUFF: Xinyan - Wildfire Rhythm ", 0.15);
    }
}

impl BuffMeta for BuffXinyanC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::XinyanC4,
        chs: "Xinyan - Wildfire Rhythm ",
        image: BuffImage::Avatar(CharacterName::Xinyan),
        genre: BuffGenre::Character,
        description: Some("Xinyan's 4th Constellation: Sweeping Fervor's swing DMG decreases opponent's Physical RES by 15% for 12s."),
        from: BuffFrom::Character(CharacterName::Xinyan),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffXinyanC4)
    }
}


pub struct BuffXinyanTalent2;

impl<A: Attribute> Buff<A> for BuffXinyanTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusPhysical, "BUFF: Xinyan - '...Now That's Rock 'N' Roll!' ", 0.15);
    }
}

impl BuffMeta for BuffXinyanTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::XinyanTalent2,
        chs: "Xinyan - '...Now That's Rock 'N' Roll!'",
        image: BuffImage::Avatar(CharacterName::Xinyan),
        genre: BuffGenre::Character,
        description: Some("Xinyan's 2nd Ascesion Talnet: Characters shielded by Sweeping Fervor deal 15% increased Physical DMG."),
        from: BuffFrom::Character(CharacterName::Xinyan),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffXinyanTalent2)
    }
}
