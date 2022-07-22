use crate::attribute::{Attribute, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffYoimiyaTalent2 {
    pub talent1_stack: usize,
}

impl<A: Attribute> Buff<A> for BuffYoimiyaTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        let p = (self.talent1_stack + 10) as f64 / 100.0;
        attribute.add_atk_percentage("BUFF: Yoimiya - Tricks of the Trouble-Maker ", p);
    }
}

impl BuffMeta for BuffYoimiyaTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::YoimiyaTalent2,
        chs: "Yoimiya - Tricks of the Trouble-Maker ",
        image: BuffImage::Avatar(CharacterName::Yoimiya),
        genre: BuffGenre::Character,
        description: Some("Yoimiya's 1st Ascension Talent: During Niwabi Fire-Dance, shots from Yoimiya's Normal Attack will increase her Pyro DMG Bonus by 2% on hit. This effect lasts for 3s and can have a maximum of 10 stacks."),
        from: BuffFrom::Character(CharacterName::Yoimiya),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "talent1_stack",
            title: "Elemental Skill Stacks",
            config: ItemConfigType::Int { min: 0, max: 10, default: 0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let talent1_stack = match *b {
            BuffConfig::YoimiyaTalent2 { talent1_stack } => talent1_stack,
            _ => 0
        };

        Box::new(BuffYoimiyaTalent2 {
            talent1_stack
        })
    }
}
