use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::characters::Shenhe;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffShenheE {
    pub skill2: usize,
    pub atk: f64
}

impl<A: Attribute> Buff<A> for BuffShenheE {
    fn change_attribute(&self, attribute: &mut A) {
        let p = Shenhe::SKILL.elemental_skill_damage_bonus[self.skill2 - 1];
        let base = p * self.atk;
        attribute.set_value_by(AttributeName::ExtraDmgCryo, "BUFF: Shenhe - Spring Spirit Summoning ", base);
    }
}

impl BuffMeta for BuffShenheE {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ShenheE,
        chs: "Shenhe - Spring Spirit Summoning ",
        image: BuffImage::Avatar(CharacterName::Shenhe),
        genre: BuffGenre::Character,
        description: Some("Shenhe's Elemental Skill: When Normal, Charged, and Plunging Attacks, Elemental Skills, and Elemental Bursts deal Cryo DMG to opponents, the DMG dealt is increased based on Shenhe's current ATK."),
        from: BuffFrom::Character(CharacterName::Shenhe),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "atk",
            title: "Shenhe's Attack",
            config: ItemConfigType::FloatInput { default: 3000.0 }
        },
        ItemConfig {
            name: "skill2",
            title: "Shenhe's Elemental Skill Level",
            config: ItemConfigType::Int { min: 1, max: 15, default: 8 },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (atk, skill2) = match *b {
            BuffConfig::ShenheE { atk, skill2 } => (atk, skill2),
            _ => (0.0, 8)
        };

        Box::new(BuffShenheE {
            atk, skill2
        })
    }
}


pub struct BuffShenheQ {
    pub skill3: usize
}

impl<A: Attribute> Buff<A> for BuffShenheQ {
    fn change_attribute(&self, attribute: &mut A) {
        let v = Shenhe::SKILL.elemental_burst_res_minus[self.skill3 - 1];
        attribute.set_value_by(AttributeName::ResMinusPhysical, "BUFF: Shenhe - Divine Maiden's Deliverance", v);
        attribute.set_value_by(AttributeName::ResMinusCryo, "BUFF: Shenhe - Divine Maiden's Deliverance", v);
    }
}

impl BuffMeta for BuffShenheQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ShenheQ,
        chs: "Shenhe - Divine Maiden's Deliverance ",
        image: BuffImage::Avatar(CharacterName::Shenhe),
        genre: BuffGenre::Character,
        description: Some("Shenhe's Elemental Burst: Unleashes the power of the Talisman Spirit, allowing it to roam free in this plane, dealing AoE Cryo DMG.<br>The Talisman Spirit then creates a field that decreases the Cryo RES and Physical RES of opponents within it. It also deals periodic Cryo DMG to opponents within the field. "),
        from: BuffFrom::Character(CharacterName::Shenhe),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "skill3",
            title: "Shenhe's Elemental Burst Level",
            config: ItemConfigType::Int { min: 1, max: 15, default: 8 },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let skill3 = match *b {
            BuffConfig::ShenheQ { skill3 } => skill3,
            _ => 8
        };

        Box::new(BuffShenheQ {
            skill3
        })
    }
}


pub struct BuffShenheTalent1 {
    pub c2: bool
}

impl<A: Attribute> Buff<A> for BuffShenheTalent1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusCryo, "BUFF: Shenhe - Deific Embrace", 0.15);
        if self.c2 {
            attribute.set_value_by(AttributeName::CriticalDamageCryo, "BUFF: Shenhe - Deific Embrace", 0.15);
        }
    }
}

impl BuffMeta for BuffShenheTalent1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ShenheTalent1,
        chs: "Shenhe - Deific Embrace ",
        image: BuffImage::Avatar(CharacterName::Shenhe),
        genre: BuffGenre::Character,
        description: Some("Shenhe's  1st Ascension Talent: An active character within the field created by Divine Maiden's Deliverance gains 15% Cryo DMG Bonus."),
        from: BuffFrom::Character(CharacterName::Shenhe),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "c2",
            title: "Constellation 2",
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let c2 = match *b {
            BuffConfig::ShenheTalent1 { c2 } => c2,
            _ => false,
        };

        Box::new(BuffShenheTalent1 {
            c2
        })
    }
}


pub struct BuffShenheTalent2 {
    pub t: usize
}

impl<A: Attribute> Buff<A> for BuffShenheTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        if self.t == 0 {
            attribute.set_value_by(AttributeName::BonusElementalSkill, "BUFF: Shenhe - Spirit Communion Seal", 0.15);
            attribute.set_value_by(AttributeName::BonusElementalBurst, "BUFF: Shenhe - Spirit Communion Seal", 0.15);
        } else {
            attribute.set_value_by(AttributeName::BonusNormalAttack, "BUFF: Shenhe - Spirit Communion Seal", 0.15);
            attribute.set_value_by(AttributeName::BonusChargedAttack, "BUFF: Shenhe - Spirit Communion Seal", 0.15);
            attribute.set_value_by(AttributeName::BonusPlungingAttack, "BUFF: Shenhe - Spirit Communion Seal", 0.15);
        }
    }
}

impl BuffMeta for BuffShenheTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ShenheTalent2,
        chs: "Shenhe - Spirit Communion Seal ",
        image: BuffImage::Avatar(CharacterName::Shenhe),
        genre: BuffGenre::Character,
        description: Some("Shenhe's 2nd Ascension Talent: After Shenhe uses Spring Spirit Summoning, she will grant all nearby party members the following effects:<br>Press: Elemental Skill and Elemental Burst DMG increased by 15% for 10s.<br>Hold: Normal, Charged, and Plunging Attack DMG increased by 15% for 15s."),
        from: BuffFrom::Character(CharacterName::Shenhe),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "t",
            title: "Skill Releasing Options",
            config: ItemConfigType::Option {
                options: "Tap,Hold",
                default: 0
            }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let t = match *b {
            BuffConfig::ShenheTalent2 { t } => t,
            _ => 0
        };

        Box::new(BuffShenheTalent2 {
            t
        })
    }
}
