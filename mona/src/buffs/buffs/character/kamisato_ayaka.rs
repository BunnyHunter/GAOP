use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffKamisatoAyakaC4;

impl<A: Attribute> Buff<A> for BuffKamisatoAyakaC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::DefMinus, "BUFF: Kamisato Ayaka - Ebb and Flow ", 0.3);
    }
}

impl BuffMeta for BuffKamisatoAyakaC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::KamisatoAyakaC4,
        chs: "Kamisato Ayaka - Ebb and Flow ",
        image: BuffImage::Avatar(CharacterName::KamisatoAyaka),
        genre: BuffGenre::Character,
        description: Some("Kamisato Ayaka's 4th Constellation: Opponents damaged by Kamisato Art: Soumetsu's Frostflake Seki no To will have their DEF decreased by 30% for 6s."),
        from: BuffFrom::Character(CharacterName::KamisatoAyaka),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffKamisatoAyakaC4)
    }
}
