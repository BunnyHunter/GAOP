use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::Element;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffVentiC2 {
    pub levitating: bool
}

impl<A: Attribute> Buff<A> for BuffVentiC2 {
    fn change_attribute(&self, attribute: &mut A) {
        let v = 0.12 + if self.levitating { 0.12 } else { 0.0 };
        attribute.set_value_by(AttributeName::ResMinusAnemo, "BUFF: Venti - Breeze of Reminiscence", v);
        attribute.set_value_by(AttributeName::ResMinusPhysical, "BUFF: Venti - Breeze of Reminiscence", v);
    }
}

impl BuffMeta for BuffVentiC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::VentiC2,
        chs: "Venti - Breeze of Reminiscence ",
        image: BuffImage::Avatar(CharacterName::Venti),
        genre: BuffGenre::Character,
        description: Some("Venti's 2nd Constellation: Skyward Sonnet decreases opponents' Anemo RES and Physical RES by 12% for 10s.<br>Opponents launched by Skyward Sonnet suffer an additional 12% Anemo RES and Physical RES decrease while airborne."),
        from: BuffFrom::Character(CharacterName::Venti),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "levitating",
            title: "Airborne",
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let levitating = match *b {
            BuffConfig::VentiC2 { levitating } => levitating,
            _ => false
        };

        Box::new(BuffVentiC2 {
            levitating
        })
    }
}


pub struct BuffVentiC6 {
    pub element: Element,
    pub is_convert: bool,
}

impl<A: Attribute> Buff<A> for BuffVentiC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusAnemo, "BUFF: Venti - Storm of Defiance ", 0.2);
        if self.is_convert {
            let name = AttributeName::bonus_name_by_element(self.element);
            attribute.set_value_by(name, "BUFF: Venti - Storm of Defiance ", 0.2);
        }
    }
}

impl BuffMeta for BuffVentiC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::VentiC6,
        chs: "Venti - Storm of Defiance ",
        image: BuffImage::Avatar(CharacterName::Venti),
        genre: BuffGenre::Character,
        description: Some("Venti's 6th Constellation: Targets who take DMG from Wind's Grand Ode have their Anemo RES decreased by 20%.<br>If an Elemental<br>Absorption occurred, then their RES towards the corresponding Element is also decreased by 20%."),
        from: BuffFrom::Character(CharacterName::Venti),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "is_convert",
            title: "Transformative Reaction",
            config: ItemConfigType::Bool { default: true },
        },
        ItemConfig {
            name: "element",
            title: "Element",
            config: ItemConfigType::Element4 { default: Element::Electro }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (is_convert, element) = match *b {
            BuffConfig::VentiC6 { is_convert, element } => (is_convert, element),
            _ => (false, Element::Electro)
        };

        Box::new(BuffVentiC6 {
            is_convert, element
        })
    }
}
