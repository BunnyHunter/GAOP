use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffThomaTalent1 {
    pub stack: f64
}

impl<A: Attribute> Buff<A> for BuffThomaTalent1 {
    fn change_attribute(&self, attribute: &mut A) {
        let v = 0.05 * self.stack;
        attribute.set_value_by(AttributeName::ShieldStrength, "BUFF: Thoma - Imbricated Armor", v);
    }
}

impl BuffMeta for BuffThomaTalent1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ThomaTalent1,
        chs: "Thoma - Imbricated Armor ",
        image: BuffImage::Avatar(CharacterName::Thoma),
        genre: BuffGenre::Character,
        description: Some("Thoma's 1st Ascension Talent: When your current active character obtains or refreshes a Blazing Barrier, this character's Shield Strength will increase by 5% for 6s.<br>This effect can be triggered once every 0.3 seconds. Max 5 stacks."),
        from: BuffFrom::Character(CharacterName::Thoma),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: "Number of Stacks",
            config: ItemConfigType::Float { min: 0.0, max: 5.0, default: 2.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let stack = match *b {
            BuffConfig::ThomaTalent1 { stack } => stack,
            _ => 0.0
        };

        Box::new(BuffThomaTalent1 {
            stack
        })
    }
}


pub struct BuffThomaC6;

impl<A: Attribute> Buff<A> for BuffThomaC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "BUFF: Thoma - Burning Heart", 0.15);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "BUFF: Thoma - Burning Heart", 0.15);
        attribute.set_value_by(AttributeName::BonusPlungingAttack, "BUFF: Thoma - Burning Heart", 0.15);
    }
}

impl BuffMeta for BuffThomaC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ThomaC6,
        chs: "Thoma - Burning Heart ",
        image: BuffImage::Avatar(CharacterName::Thoma),
        genre: BuffGenre::Character,
        description: Some("Thoma's 6th Constellation: When a Blazing Barrier is obtained or refreshed, the DMG dealt by all party members' Normal, Charged, and Plunging Attacks is increased by 15% for 6s."),
        from: BuffFrom::Character(CharacterName::Thoma),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffThomaC6)
    }
}
