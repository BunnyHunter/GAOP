use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::characters::Yunjin;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffYunjinQ {
    pub skill3: usize,
    pub talent2: bool,
    pub ele_count: usize,
    pub def: f64,
}

impl<A: Attribute> Buff<A> for BuffYunjinQ {
    fn change_attribute(&self, attribute: &mut A) {
        let base = Yunjin::SKILL.elemental_burst_dmg_bonus[self.skill3 - 1];
        let extra = if self.talent2 {
            match self.ele_count {
                1 => 0.025,
                2 => 0.05,
                3 => 0.075,
                _ => 0.115
            }
        } else {
            0.0
        };
        let v = (base + extra) * self.def;

        attribute.set_value_by(AttributeName::ExtraDmgNormalAttack, "BUFF: Yunjin - Cliffbreaker's Banner ", v);
    }
}

impl BuffMeta for BuffYunjinQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::YunjinQ,
        chs: "Yunjin - Cliffbreaker's Banner",
        image: BuffImage::Avatar(CharacterName::Yunjin),
        genre: BuffGenre::Character,
        description: Some("Yunjin's Elemental Burst: When Normal Attack DMG is dealt to opponents, Bonus DMG will be dealt based on Yun Jin's current DEF.<br>The effects of this skill will be cleared after a set duration or after being triggered a specific number of times.<br>When one Normal Attack hits multiple opponents, the effect is triggered multiple times according to the number of opponents hit. The number of times that the effect is triggered is counted independently for each member of the party with Flying Cloud Flag Formation."),
        from: BuffFrom::Character(CharacterName::Yunjin),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "skill3",
            title: "Elemental Burst Level",
            config: ItemConfigType::Int { min: 1, max: 15, default: 10 }
        },
        ItemConfig {
            name: "def",
            title: "Yunjin's Defense",
            config: ItemConfigType::FloatInput { default: 2000.0 }
        },
        ItemConfig {
            name: "talent2",
            title: "Over Level 60",
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "ele_count",
            title: "Number of different elements on the team",
            config: ItemConfigType::Int { min: 1, max: 4, default: 4 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (skill3, def, talent2, ele_count) = match *b {
            BuffConfig::YunjinQ { skill3, def, talent2, ele_count } => (skill3, def, talent2, ele_count),
            _ => (1, 0.0, false, 1)
        };

        Box::new(BuffYunjinQ {
            skill3, def, talent2, ele_count
        })
    }
}


pub struct BuffYunjinC2;

impl<A: Attribute> Buff<A> for BuffYunjinC2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusNormalAttack, "BUFF: Yunjin - Myriad Mise-En-Scène ", 0.15);
    }
}

impl BuffMeta for BuffYunjinC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::YunjinC2,
        chs: "Yunjin - Myriad Mise-En-Scène ",
        image: BuffImage::Avatar(CharacterName::Yunjin),
        genre: BuffGenre::Character,
        description: Some("Yunjin's 2nd Constellation: After Cliffbreaker's Banner is unleashed, all nearby party members' Normal Attack DMG is increased by 15% for 12s."),
        from: BuffFrom::Character(CharacterName::Yunjin),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffYunjinC2)
    }
}
