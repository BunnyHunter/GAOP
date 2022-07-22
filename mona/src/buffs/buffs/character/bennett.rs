use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::characters::Bennett;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffBennettQ {
    pub base_atk: f64,
    pub c1: bool,
    pub skill3: usize,
}

impl<A: Attribute> Buff<A> for BuffBennettQ {
    fn change_attribute(&self, attribute: &mut A) {
        let p = Bennett::SKILL.elemental_burst_atk_bonus[self.skill3 - 1] + (if self.c1 { 0.2 } else { 0.0 });
        let v = p * self.base_atk;

        attribute.set_value_by(AttributeName::ATKFixed, "BUFF：Bennett - Fantastic Voyage ", v);
    }
}

impl BuffMeta for BuffBennettQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::BennettQ,
        chs: "Bennett - Fantastic Voyage ",
        image: BuffImage::Avatar(CharacterName::Bennett),
        genre: BuffGenre::Character,
        description: Some("Bennett's Elemental Burst: If the health of a character within the AoE is higher than 70%, they gain an ATK Bonus that is based on Bennett's Base ATK. "),
        from: BuffFrom::Character(CharacterName::Bennett),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "base_atk",
            title: "Bennett's Base Attack",
            config: ItemConfigType::FloatInput { default: 800.0 },
        },
        ItemConfig {
            name: "c1",
            title: "Constellation 1",
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "skill3",
            title: "Skill Level",
            config: ItemConfigType::Int { min: 1, max: 15, default: 10 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (base_atk, c1, skill3) = match *b {
            BuffConfig::BennettQ { base_atk, c1, skill3 } => (base_atk, c1, skill3),
            _ => (0.0, false, 1)
        };

        Box::new(BuffBennettQ {
            base_atk, c1, skill3
        })
    }
}

pub struct BuffBennettC6;

impl<A: Attribute> Buff<A> for BuffBennettC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusPyro, "BUFF: Bennett - Fire Ventures with Me ", 0.15);
    }
}

impl BuffMeta for BuffBennettC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::BennettC6,
        chs: "Bennett - Fire Ventures with Me ",
        image: BuffImage::Avatar(CharacterName::Bennett),
        genre: BuffGenre::Character,
        description: Some("Bennett's 6th Constellation: Sword, Claymore, or Polearm-wielding characters inside Fantastic Voyage's radius gain a 15% Pyro DMG Bonus and their weapons are infused with Pyro."),
        from: BuffFrom::Character(CharacterName::Bennett)
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffBennettC6)
    }
}
