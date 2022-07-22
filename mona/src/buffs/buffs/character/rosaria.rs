use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffRosariaTalent2 {
    pub crit: f64,
}

impl<A: Attribute> Buff<A> for BuffRosariaTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        let c = (self.crit * 0.15).clamp(0.0, 0.15);
        attribute.set_value_by(AttributeName::CriticalBase, "BUFF: Rosaria - Shadow Samaritan", c);
    }
}

impl BuffMeta for BuffRosariaTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::RosariaTalent2,
        chs: "Rosaria - Shadow Samaritan ",
        image: BuffImage::Avatar(CharacterName::Rosaria),
        genre: BuffGenre::Character,
        description: Some("Rosaria's 2nd Ascension Talent: Casting Rites of Termination increases CRIT Rate of all nearby party members (except Rosaria herself) by 15% of Rosaria's CRIT Rate for 10s."),
        from: BuffFrom::Character(CharacterName::Rosaria),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "crit",
            title: "Rosaria's Crit Rate",
            config: ItemConfigType::FloatPercentageInput { default: 70.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let v = match *b {
            BuffConfig::RosariaTalent2 { crit } => crit / 100.0,
            _ => 0.0
        };

        Box::new(BuffRosariaTalent2 {
            crit: v
        })
    }
}


pub struct BuffRosariaC6;

impl<A: Attribute> Buff<A> for BuffRosariaC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusPhysical, "BUFF: Rosaria - Divine Retribution ", 0.2);
    }
}

impl BuffMeta for BuffRosariaC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::RosariaC6,
        chs: "Rosaria - Divine Retribution ",
        image: BuffImage::Avatar(CharacterName::Rosaria),
        genre: BuffGenre::Character,
        description: Some("Rosaria's 6th Constellation: Rites of Termination's attack decreases opponents' Physical RES by 20% for 10s."),
        from: BuffFrom::Character(CharacterName::Rosaria),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffRosariaC6)
    }
}
