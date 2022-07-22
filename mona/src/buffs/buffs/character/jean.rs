use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffJeanC4;

impl<A: Attribute> Buff<A> for BuffJeanC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusAnemo, "BUFF: Jean - Lands of Dandelion", 0.4);
    }
}

impl BuffMeta for BuffJeanC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::JeanC4,
        chs: "Jean - Lands of Dandelion ",
        image: BuffImage::Avatar(CharacterName::Jean),
        genre: BuffGenre::Character,
        description: Some("Jean's 4th Constellation: Within the Field created by Dandelion Breeze, all opponents have their Anemo RES decreased by 40%."),
        from: BuffFrom::Character(CharacterName::Jean),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffJeanC4)
    }
}