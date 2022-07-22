use crate::artifacts::ArtifactSetName;
use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffInstructor4;

impl<A: Attribute> Buff<A> for BuffInstructor4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "BUFF: Instructor 4 Piece Effect ", 120.0);
    }
}

impl BuffMeta for BuffInstructor4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::Instructor4,
        chs: "Instructor 4 Piece Effect ",
        image: BuffImage::Artifact(ArtifactSetName::Instructor),
        genre: BuffGenre::Artifact,
        description: Some("Upon triggering an Elemental Reaction, increases all party members' Elemental Mastery by 120 for 8s."),
        from: BuffFrom::Artifact(ArtifactSetName::Instructor),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffInstructor4)
    }
}
