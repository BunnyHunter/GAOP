use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffKleeC2;

impl<A: Attribute> Buff<A> for BuffKleeC2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::DefMinus, "BUFF: Klee - Explosive Frags", 0.23);
    }
}

impl BuffMeta for BuffKleeC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::KleeC2,
        chs: "Klee - Explosive Frags",
        image: BuffImage::Avatar(CharacterName::Klee),
        genre: BuffGenre::Character,
        description: Some("Klee's 2nd Constellation: Being hit by Jumpy Dumpty's mines decreases opponents' DEF by 23% for 10s."),
        from: BuffFrom::Character(CharacterName::Klee),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffKleeC2)
    }
}

pub struct BuffKleeC6;

impl<A: Attribute> Buff<A> for BuffKleeC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusPyro, "BUFF: 可莉六命「Pyro力全开」", 0.1);
    }
}

impl BuffMeta for BuffKleeC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::KleeC6,
        chs: "Klee - Blazing Delight ",
        image: BuffImage::Avatar(CharacterName::Klee),
        genre: BuffGenre::Character,
        description: Some("Klee's 2nd Constellation: While under the effects of Sparks 'n' Splash, Klee will regenerate 3 Energy for all members of the party (excluding Klee) every 3s. When Sparks 'n' Splash is used, all party members will gain a 10% Pyro DMG Bonus for 25s."),
        from: BuffFrom::Character(CharacterName::Klee),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffKleeC6)
    }
}
