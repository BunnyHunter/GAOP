use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::characters::KujouSara;
use crate::character::prelude::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffKujouSaraEOrQ {
    pub c6: bool,
    pub base_atk: f64,
    pub skill2: usize,
}

impl<A: Attribute> Buff<A> for BuffKujouSaraEOrQ {
    fn change_attribute(&self, attribute: &mut A) {
        let p = KujouSara::SKILL.elemental_skill_atk_bonus[self.skill2 - 1];
        let atk_bonus = p * self.base_atk;
        attribute.set_value_by(AttributeName::ATKFixed, "BUFF: Kujou Sara - Tengu Stormcall ", atk_bonus);
        if self.c6 {
            attribute.set_value_by(AttributeName::CriticalDamageElectro, "BUFF: Kujou Sara - Subjugation: Koukou Sendou", 0.6);
        }
    }
}

impl BuffMeta for BuffKujouSaraEOrQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::KujouSaraEOrQ,
        chs: "Kujou Sara E Or Q ",
        image: BuffImage::Avatar(CharacterName::KujouSara),
        genre: BuffGenre::Character,
        description: Some("Kujou Sara E Or Q : Tengu Stormcall: Crowfeathers will trigger Tengu Juurai: Ambush after a short time, dealing Electro DMG and granting the active character within its AoE an ATK Bonus based on Kujou Sara's Base ATK. <br>Sin of Pride: The Electro DMG of characters who have had their ATK increased by Tengu Juurai has its Crit DMG increased by 60%."),
        from: BuffFrom::Character(CharacterName::KujouSara),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "base_atk",
            title: "Kujou Sara Attack",
            config: ItemConfigType::FloatInput { default: 700.0 },
        },
        ItemConfig {
            name: "c6",
            title: "Constellation 6",
            config: ItemConfigType::Bool { default: false },
        },
        ItemConfig {
            name: "skill2",
            title: "Elemental Skill Level ",
            config: ItemConfigType::Int { min: 1, max: 15, default: 10 },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (base_atk, c6, skill2) = match *b {
            BuffConfig::KujouSaraEOrQ { base_atk, c6, skill2 } => (base_atk, c6, skill2),
            _ => (0.0, false, 1)
        };

        Box::new(BuffKujouSaraEOrQ {
            base_atk, c6, skill2
        })
    }
}
