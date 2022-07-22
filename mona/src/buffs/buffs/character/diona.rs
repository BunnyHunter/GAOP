use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::enemies::Enemy;

pub struct BuffDionaC6G50;

impl<A: Attribute> Buff<A> for BuffDionaC6G50 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "BUFF: Diona - Cat's Tail Closing Time", 200.0);
    }
}

impl BuffMeta for BuffDionaC6G50 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::DionaC6G50,
        chs: "Diona - Cat's Tail Closing Time ",
        image: BuffImage::Avatar(CharacterName::Diona),
        genre: BuffGenre::Character,
        description: Some("Diona's 6th Constellation: Characters within Signature Mix's radius will gain the following effects based on their HP amounts:<br>Increases Incoming Healing Bonus by 30% when HP falls below or is equal to 50%.<br>Elemental Mastery increased by 200 when HP is above 50%."),
        from: BuffFrom::Character(CharacterName::Diona)
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffDionaC6G50)
    }
}
