use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffNingguangTalent2;

impl<A: Attribute> Buff<A> for BuffNingguangTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusGeo, "BUFF: Ningguang - Strategic Reserve ", 0.12);
    }
}

impl BuffMeta for BuffNingguangTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::NingguangTalent2,
        chs: "Ningguang - Strategic Reserve ",
        image: BuffImage::Avatar(CharacterName::Ningguang),
        genre: BuffGenre::Character,
        description: Some("Ningguang's 2nd Ascesion Talent: A character that passes through the Jade Screen will gain a 12% Geo DMG Bonus for 10s."),
        from: BuffFrom::Character(CharacterName::Ningguang),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffNingguangTalent2)
    }
}