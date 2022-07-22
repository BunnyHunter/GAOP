use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffRazorC4;

impl<A: Attribute> Buff<A> for BuffRazorC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::DefMinus, "BUFF: Razor - Bite", 0.15);
    }
}

impl BuffMeta for BuffRazorC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::RazorC4,
        chs: "Razor - Bite ",
        image: BuffImage::Avatar(CharacterName::Razor),
        genre: BuffGenre::Character,
        description: Some("Razor's 4th Constellation: When casting Claw and Thunder (Press), opponents hit will have their DEF decreased by 15% for 7s."),
        from: BuffFrom::Character(CharacterName::Razor),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffRazorC4)
    }
}
