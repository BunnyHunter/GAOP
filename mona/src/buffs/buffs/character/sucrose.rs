use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::Element;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffSucroseTalent1;

impl<A: Attribute> Buff<A> for BuffSucroseTalent1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "BUFF: Sucrose -  Catalyst Conversion ", 50.0);
    }
}

impl BuffMeta for BuffSucroseTalent1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::SucroseTalent1,
        chs: "Sucrose -  Catalyst Conversion ",
        image: BuffImage::Avatar(CharacterName::Sucrose),
        genre: BuffGenre::Character,
        description: Some("Sucrose's 1st Ascesion Talent: When Sucrose triggers a Swirl effect, all characters in the party with the matching element (excluding Sucrose) have their Elemental Mastery increased by 50 for 8s."),
        from: BuffFrom::Character(CharacterName::Sucrose),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffSucroseTalent1)
    }
}


pub struct BuffSucroseTalent2 {
    pub em: f64
}

impl<A: Attribute> Buff<A> for BuffSucroseTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        let v = self.em * 0.2;
        attribute.set_value_by(AttributeName::ElementalMastery, "BUFF: 砂糖天赋「小小的慧Anemo」", v);
    }
}

impl BuffMeta for BuffSucroseTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::SucroseTalent2,
        chs: "Sucrose - Mollis Favonius ",
        image: BuffImage::Avatar(CharacterName::Sucrose),
        genre: BuffGenre::Character,
        description: Some("Sucrose's 2nd Ascension Talent: When Astable Anemohypostasis Creation - 6308 or Forbidden Creation - Isomer 75 / Type II hits an opponent, increases all party members' (excluding Sucrose) Elemental Mastery based on 20% of Sucrose's Elemental Mastery for 8s."),
        from: BuffFrom::Character(CharacterName::Sucrose),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "em",
            title: "Sucrose Elemental Mastery",
            config: ItemConfigType::FloatInput { default: 200.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let em = match *b {
            BuffConfig::SucroseTalent2 { em } => em,
            _ => 0.0
        };

        Box::new(BuffSucroseTalent2 {
            em
        })
    }
}


pub struct BuffSucroseC6 {
    pub element: Element
}

impl<A: Attribute> Buff<A> for BuffSucroseC6 {
    fn change_attribute(&self, attribute: &mut A) {
        let name = AttributeName::bonus_name_by_element(self.element);
        attribute.set_value_by(name, "BUFF: Sucrose - Chaotic Entropy ", 0.2);
    }
}

impl BuffMeta for BuffSucroseC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::SucroseC6,
        chs: "Sucrose - Chaotic Entropy ",
        image: BuffImage::Avatar(CharacterName::Sucrose),
        genre: BuffGenre::Character,
        description: Some("Sucrose's 6th Constellation: If Forbidden Creation - Isomer 75 / Type II triggers an Elemental Absorption, all party members gain a 20% Elemental DMG Bonus for the corresponding absorbed element during its duration."),
        from: BuffFrom::Character(CharacterName::Sucrose),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "element",
            title: "Swirl Type",
            config: ItemConfigType::Element4 { default: Element::Electro }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let element = match *b {
            BuffConfig::SucroseC6 { element } => element,
            _ => Element::Electro
        };

        Box::new(BuffSucroseC6 {
            element
        })
    }
}
