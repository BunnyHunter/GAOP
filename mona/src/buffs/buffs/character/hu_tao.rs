use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffHuTaoTalent1;

impl<A: Attribute> Buff<A> for BuffHuTaoTalent1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::CriticalBase, "BUFF: Hu Tao - Flutter By ", 0.12);
    }
}

impl BuffMeta for BuffHuTaoTalent1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::HuTaoTalent1,
        chs: "Hu Tao - Flutter By ",
        image: BuffImage::Avatar(CharacterName::HuTao),
        genre: BuffGenre::Character,
        description: Some("Hu Tao's 1st Ascension Talent: When a Paramita Papilio state activated by Guide to Afterlife ends, all allies in the party (excluding Hu Tao herself) will have their CRIT Rate increased by 12% for 8s."),
        from: BuffFrom::Character(CharacterName::HuTao),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffHuTaoTalent1)
    }
}