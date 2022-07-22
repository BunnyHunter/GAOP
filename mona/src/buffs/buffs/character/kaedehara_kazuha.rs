use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::Element;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffKaedeharaKazuhaTalent2 {
    pub em: f64,
    pub element: Element
}

impl<A: Attribute> Buff<A> for BuffKaedeharaKazuhaTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        let name = AttributeName::bonus_name_by_element(self.element);
        let value = 0.0004 * self.em;
        attribute.set_value_by(name, "BUFF: Kaedehara Kazuha - Poetics of Fuubutsu", value);
    }
}

impl BuffMeta for BuffKaedeharaKazuhaTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::KaedeharaKazuhaTalent2,
        chs: "Kaedehara Kazuha - Poetics of Fuubutsu ",
        image: BuffImage::Avatar(CharacterName::KaedeharaKazuha),
        genre: BuffGenre::Character,
        description: Some("Kaedehara Kazuha's 2nd Ascension Talent: Upon triggering a Swirl reaction, Kaedehara Kazuha will grant all party members a 0.04% Elemental DMG Bonus to the element absorbed by Swirl for every point of Elemental Mastery he has for 8s. Bonuses for different elements obtained through this method can co-exist."),
        from: BuffFrom::Character(CharacterName::KaedeharaKazuha),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "element",
            title: "Swirl Element",
            config: ItemConfigType::Element4 { default: Element::Electro }
        },
        ItemConfig {
            name: "em",
            title: "Kaedehara Kazuha's Elemental Mastery",
            config: ItemConfigType::FloatInput { default: 800.0 },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (element, em) = match *b {
            BuffConfig::KaedeharaKazuhaTalent2 { element, em } => (element, em),
            _ => (Element::Electro, 0.0)
        };

        Box::new(BuffKaedeharaKazuhaTalent2 {
            element, em
        })
    }
}

pub struct BuffKaedeharaKazuhaC2;

impl<A: Attribute> Buff<A> for BuffKaedeharaKazuhaC2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "BUFF: Kaedehara Kazuha - Yamaarashi Tailwind", 200.0);
    }
}

impl BuffMeta for BuffKaedeharaKazuhaC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::KaedeharaKazuhaC2,
        chs: "Kaedehara Kazuha - Yamaarashi Tailwind ",
        image: BuffImage::Avatar(CharacterName::KaedeharaKazuha),
        genre: BuffGenre::Character,
        description: Some("Kaedehara Kazuha's 2nd Constellation: The Autumn Whirlwind field created by Kazuha Slash has the following effects: Increases Kaedehara Kazuha's own Elemental Mastery by 200.Increases the Elemental Mastery of characters within the field by 200.The Elemental Mastery-increasing effects of this Constellation do not stack."),
        from: BuffFrom::Character(CharacterName::KaedeharaKazuha),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffKaedeharaKazuhaC2)
    }
}