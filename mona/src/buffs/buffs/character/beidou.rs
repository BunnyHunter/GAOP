use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;

pub struct BuffBeidouC6;

impl<A: Attribute> Buff<A> for BuffBeidouC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusElectro, "BUFF：Beidou - Bane of Evil ", 0.15);
    }
}

impl BuffMeta for BuffBeidouC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::BeidouC6,
        chs: "Beidou - Bane of Evil ",
        image: BuffImage::Avatar(CharacterName::Beidou),
        genre: BuffGenre::Character,
        description: Some("Beidou's 6th Constellation: During the duration of Stormbreaker, the Electro RES of surrounding opponents is decreased by 15%."),
        from: BuffFrom::Character(CharacterName::Beidou),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffBeidouC6)
    }
}
