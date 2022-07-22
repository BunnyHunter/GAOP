use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffZhongliShield;

impl<A: Attribute> Buff<A> for BuffZhongliShield {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusBase, "BUFF: Zhongli - Dominus Lapidis ", 0.2);
    }
}

impl BuffMeta for BuffZhongliShield {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ZhongliShield,
        chs: "Zhongli - Dominus Lapidis ",
        image: BuffImage::Avatar(CharacterName::Zhongli),
        genre: BuffGenre::Character,
        description: Some("Zhongli's Elemental Skill: Characters protected by the Jade Shield will decrease the Elemental RES and Physical RES of opponents in a small AoE by 20%. This effect cannot be stacked."),
        from: BuffFrom::Character(CharacterName::Zhongli),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffZhongliShield)
    }
}
