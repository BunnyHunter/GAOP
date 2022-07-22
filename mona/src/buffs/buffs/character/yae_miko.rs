use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::enemies::Enemy;

pub struct BuffYaeMikoC4;

impl<A: Attribute> Buff<A> for BuffYaeMikoC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusElectro, "BUFF: Yae Miko - Sakura Channeling ", 0.2);
    }
}

impl BuffMeta for BuffYaeMikoC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::YaeMikoC4,
        chs: "Yae Miko - Sakura Channeling ",
        image: BuffImage::Avatar(CharacterName::YaeMiko),
        genre: BuffGenre::Character,
        description: Some("Yae Miko's 4th Constellation: When Sesshou Sakura lightning hits opponents, the Electro DMG Bonus of all nearby party members is increased by 20% for 5s."),
        from: BuffFrom::Character(CharacterName::YaeMiko)
    };

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffYaeMikoC4)
    }
}