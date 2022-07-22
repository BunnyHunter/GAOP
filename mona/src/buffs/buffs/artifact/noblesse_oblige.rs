use crate::artifacts::ArtifactSetName;
use crate::attribute::{Attribute, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffNoblesseOblige4;

impl<A: Attribute> Buff<A> for BuffNoblesseOblige4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_atk_percentage("BUFF: Noblesse Oblige 4 Piece Effect ", 0.2);
    }
}

impl BuffMeta for BuffNoblesseOblige4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::NoblesseOblige4,
        chs: "Noblesse Oblige 4 Piece Effect ",
        image: BuffImage::Artifact(ArtifactSetName::NoblesseOblige),
        genre: BuffGenre::Artifact,
        description: Some("Using an Elemental Burst increases all party members' ATK by 20% for 12s. This effect cannot stack."),
        from: BuffFrom::Artifact(ArtifactSetName::NoblesseOblige),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffNoblesseOblige4)
    }
}
