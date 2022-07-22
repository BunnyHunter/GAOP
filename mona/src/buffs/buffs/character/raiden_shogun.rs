use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::characters::RaidenShogun;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffRaidenShogunE {
    pub skill2: usize,
    pub energy: usize,
}

impl<A: Attribute> Buff<A> for BuffRaidenShogunE {
    fn change_attribute(&self, attribute: &mut A) {
        let p = RaidenShogun::SKILL.elemental_skill_q_bonus[self.skill2 - 1];
        let q_bonus = p * self.energy as f64;
        attribute.set_value_by(AttributeName::BonusElementalBurst, "BUFF: Raiden Shogun - Transcendence: Baleful Omen", q_bonus);
    }
}

impl BuffMeta for BuffRaidenShogunE {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::RaidenShogunE,
        chs: "Raiden Shogun - Transcendence: Baleful Omen ",
        image: BuffImage::Avatar(CharacterName::RaidenShogun),
        genre: BuffGenre::Character,
        description: Some("Raiden Shogun's Elemental Skill: The Raiden Shogun unveils a shard of her Euthymia, dealing Electro DMG to nearby opponents, and granting nearby party members the Eye of Stormy Judgment.<br>Eye of Stormy Judgment<br>When characters with this buff attack and deal DMG to opponents, the Eye will unleash a coordinated attack, dealing AoE Electro DMG at the opponent's position.<br>Characters who gain the Eye of Stormy Judgment will have their Elemental Burst DMG increased based on the Energy Cost of the Elemental Burst during the Eye's duration."),
        from: BuffFrom::Character(CharacterName::RaidenShogun),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "skill2",
            title: "Elemental Skill Level",
            config: ItemConfigType::Int { min: 1, max: 15, default: 8 }
        },
        ItemConfig {
            name: "energy",
            title: "Maximum Energy Recharge of Buffed Character",
            config: ItemConfigType::Int { min: 20, max: 100, default: 80 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (energy, skill2) = match *b {
            BuffConfig::RaidenShogunE { energy, skill2 } => (energy, skill2),
            _ => (0, 1)
        };

        Box::new(BuffRaidenShogunE {
            energy, skill2
        })
    }
}


pub struct BuffRaidenShogunC4;

impl<A: Attribute> Buff<A> for BuffRaidenShogunC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_atk_percentage("BUFF: Raiden Shogun - Pledge of Propriety", 0.3);
    }
}

impl BuffMeta for BuffRaidenShogunC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::RaidenShogunC4,
        chs: "Raiden Shogun - Pledge of Propriety ",
        image: BuffImage::Avatar(CharacterName::RaidenShogun),
        genre: BuffGenre::Character,
        description: Some("Raiden Shogun's 4th Constellation: When the Musou Isshin state applied by Secret Art: Musou Shinsetsu expires, all nearby party members (excluding the Raiden Shogun) gain 30% bonus ATK for 10s. "),
        from: BuffFrom::Character(CharacterName::RaidenShogun),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffRaidenShogunC4)
    }
}
