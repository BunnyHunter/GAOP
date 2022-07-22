use crate::attribute::{Attribute, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffAratakiIttoC4;

impl<A: Attribute> Buff<A> for BuffAratakiIttoC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_atk_percentage("BUFF: Arataki Itto - Jailhouse Bread and Butter ", 0.2);
        attribute.add_def_percentage("BUFF: Arataki Itto - Jailhouse Bread and Butter ", 0.2);
    }
}

impl BuffMeta for BuffAratakiIttoC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::AratakiIttoC4,
        chs: "Arataki Itto - Jailhouse Bread and Butter ",
        image: BuffImage::Avatar(CharacterName::AratakiItto),
        genre: BuffGenre::Character,
        description: Some("Arataki Itto's 4th Constellation: When the Raging Oni King state caused by Royal Descent: Behold, Itto the Evil! ends, all nearby party members gain 20% DEF and 20% ATK for 10s."),
        from: BuffFrom::Character(CharacterName::AratakiItto),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffAratakiIttoC4)
    }
}
