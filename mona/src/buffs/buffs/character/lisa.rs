use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffLisaTalent2;

impl<A: Attribute> Buff<A> for BuffLisaTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::DefMinus, "BUFF: Lisa - Static Electricity Field ", 0.15);
    }
}

impl BuffMeta for BuffLisaTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::LisaTalent2,
        chs: "Lisa - Static Electricity Field ",
        image: BuffImage::Avatar(CharacterName::Lisa),
        genre: BuffGenre::Character,
        description: Some("Lisa's 2nd Ascesion Talent: Opponents hit by Lightning Rose have their DEF decreased by 15% for 10s."),
        from: BuffFrom::Character(CharacterName::Lisa),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffLisaTalent2)
    }
}
