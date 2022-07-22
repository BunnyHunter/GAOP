use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::enemies::Enemy;

pub struct BuffAlbedoTalent2;

impl<A: Attribute> Buff<A> for BuffAlbedoTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "BUFF: Albedo - Homuncular Nature ", 125.0);
    }
}

impl BuffMeta for BuffAlbedoTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::AlbedoTalent2,
        chs: "Albedo - Homuncular Nature",
        image: BuffImage::Avatar(CharacterName::Albedo),
        genre: BuffGenre::Character,
        description: Some("Albedo's 2nd Ascension Talent:  When Using Rite of Progeniture Tectonic Tide increases the Elemental Mastery of nearby party members by 125 for 10s."),
        from: BuffFrom::Character(CharacterName::Albedo),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffAlbedoTalent2)
    }
}
