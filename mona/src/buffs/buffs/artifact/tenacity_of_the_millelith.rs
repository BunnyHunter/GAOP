use crate::artifacts::ArtifactSetName;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffTenacityOfTheMillelith4;

impl<A: Attribute> Buff<A> for BuffTenacityOfTheMillelith4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ShieldStrength, "BUFF: Tenacity Of The Millelith 4 Piece Effect ", 0.3);
        attribute.add_atk_percentage("BUFF: Tenacity Of The Millelith 4 Piece Effect ", 0.2);
    }
}

impl BuffMeta for BuffTenacityOfTheMillelith4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::TenacityOfTheMillelith4,
        chs: "Tenacity Of The Millelith 4 Piece Effect ",
        image: BuffImage::Artifact(ArtifactSetName::TenacityOfTheMillelith),
        genre: BuffGenre::Artifact,
        description: Some("When an Elemental Skill hits an opponent, the ATK of all nearby party members is increased by 20% and their Shield Strength is increased by 30% for 3s. This effect can be triggered once every 0.5s. This effect can still be triggered even when the character who is using this artifact set is not on the field."),
        from: BuffFrom::Artifact(ArtifactSetName::TenacityOfTheMillelith),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffTenacityOfTheMillelith4)
    }
}