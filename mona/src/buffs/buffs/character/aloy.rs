use crate::attribute::{Attribute, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::enemies::Enemy;

pub struct BuffAloyTalent1;

impl<A: Attribute> Buff<A> for BuffAloyTalent1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_atk_percentage("BUFF: Aloy - Combat Override ", 0.08);
    }
}

impl BuffMeta for BuffAloyTalent1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::AloyTalent1,
        chs: "Aloy - Combat Override ",
        image: BuffImage::Avatar(CharacterName::Aloy),
        genre: BuffGenre::Character,
        description: Some("Aloy's 1st Ascension Talent: When Aloy receives the Coil effect from Frozen Wilds, her ATK is increased by 16%, while nearby party members' ATK is increased by 8%. This effect lasts 10s.%"),
        from: BuffFrom::Character(CharacterName::Aloy),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffAloyTalent1)
    }
}
