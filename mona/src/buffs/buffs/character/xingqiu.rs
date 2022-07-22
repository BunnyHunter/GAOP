use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffXingqiuC2;

impl<A: Attribute> Buff<A> for BuffXingqiuC2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusHydro, "BUFF: Xingqiu - Rainbow Upon the Azure Sky ", 0.15);
    }
}

impl BuffMeta for BuffXingqiuC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::XingqiuC2,
        chs: "Xingqiu - Rainbow Upon the Azure Sky ",
        image: BuffImage::Avatar(CharacterName::Xingqiu),
        genre: BuffGenre::Character,
        description: Some("Xingqiu's 2nd Constellation: Extends the duration of Guhua Sword: Raincutter by 3s. <br>Decreases the Hydro RES of opponents hit by sword rain attacks by 15% for 4s."),
        from: BuffFrom::Character(CharacterName::Xingqiu),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffXingqiuC2)
    }
}
