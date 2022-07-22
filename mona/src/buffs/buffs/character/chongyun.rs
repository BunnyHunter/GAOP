use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffChongyunTalent2;

impl<A: Attribute> Buff<A> for BuffChongyunTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusCryo, "Chongyun - Rimechaser Blade ", 0.1);
    }
}

impl BuffMeta for BuffChongyunTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ChongyunTalent2,
        chs: "Chongyun - Rimechaser Blade ",
        image: BuffImage::Avatar(CharacterName::Chongyun),
        genre: BuffGenre::Character,
        description: Some("Chongyun's 2nd Ascension Talent: When the field created by Spirit Blade: Chonghua's Layered Frost disappears, another spirit blade will be summoned to strike nearby opponents, dealing 100% of Chonghua's Layered Frost's Skill DMG as AoE Cryo DMG.<br>Opponents hit by this blade will have their Cryo RES decreased by 10% for 8s."),
        from: BuffFrom::Character(CharacterName::Chongyun),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffChongyunTalent2)
    }
}
