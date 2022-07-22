use crate::artifacts::ArtifactSetName;
use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::Element;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffViridescentVenerer4 {
    pub element: Element
}

impl<A: Attribute> Buff<A> for BuffViridescentVenerer4 {
    fn change_attribute(&self, attribute: &mut A) {
        let name = AttributeName::res_minus_name_by_element(self.element);
        attribute.set_value_by(name, "BUFF: Viridescent Venerer 4 Piece Effect", 0.4);
    }
}

impl BuffMeta for BuffViridescentVenerer4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ViridescentVenerer4,
        chs: "Viridescent Venerer 4 Piece Effect",
        image: BuffImage::Artifact(ArtifactSetName::ViridescentVenerer),
        genre: BuffGenre::Artifact,
        description: Some("Increases Swirl DMG by 60%. Decreases opponent's Elemental RES to the element infused in the Swirl by 40% for 10s."),
        from: BuffFrom::Artifact(ArtifactSetName::ViridescentVenerer),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "element",
            title: "Swirl Element",
            config: ItemConfigType::Element4 { default: Element::Electro }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let element = match *b {
            BuffConfig::ViridescentVenerer4 { element } => element,
            _ => Element::Electro
        };

        Box::new(BuffViridescentVenerer4 {
            element
        })
    }
}
