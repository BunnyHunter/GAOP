use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::character::characters::Mona;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffMonaQ {
    pub c4: bool,
    pub skill3: usize,
}

impl<A: Attribute> Buff<A> for BuffMonaQ {
    fn change_attribute(&self, attribute: &mut A) {
        let bonus = Mona::SKILL.elemental_burst_bonus[self.skill3 - 1];
        attribute.set_value_by(AttributeName::BonusBase, "BUFF: Mona - Stellaris Phantasm", bonus);
        if self.c4 {
            attribute.set_value_by(AttributeName::CriticalBase, "BUFF: Mona - Prophecy of Oblivion", 0.15);
        }
    }
}

impl BuffMeta for BuffMonaQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::MonaQ,
        chs: "Mona - Stellaris Phantasm ",
        image: BuffImage::Avatar(CharacterName::Mona),
        genre: BuffGenre::Character,
        description: Some("Mona's Elemental Burst : Mona summons the sparkling waves and creates a reflection of the starry sky, applying the Illusory Bubble status to opponents in a large AoE.<br>Illusory Bubble<br>Traps opponents inside a pocket of destiny and also makes them Wet.Renders weaker opponents immobile.When an opponent affected by Illusory Bubble sustains DMG, it has the following effects:<br>Applies an Omen to the opponent, which gives a DMG Bonus, also increasing the DMG of the attack that causes it.<br>Removes the Illusory Bubble, dealing Hydro DMG in the process<br>Omen<br>During its duration, increases DMG taken by opponents."),
        from: BuffFrom::Character(CharacterName::Mona),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "skill3",
            title: "Elemental Burst Level",
            config: ItemConfigType::Int { min: 1, max: 15, default: 9 }
        },
        ItemConfig {
            name: "c4",
            title: "Constellation 4",
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (c4, skill3) = match *b {
            BuffConfig::MonaQ { c4, skill3 } => (c4, skill3),
            _ => (false, 1)
        };
        Box::new(BuffMonaQ {
            c4, skill3
        })
    }
}


pub struct BuffMonaC1;

impl<A: Attribute> Buff<A> for BuffMonaC1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::EnhanceElectroCharged, "BUFF: Mona - Prophecy of Submersion", 0.15);
        attribute.set_value_by(AttributeName::EnhanceVaporize, "BUFF: Mona - Prophecy of Submersion", 0.15);
        attribute.set_value_by(AttributeName::EnhanceSwirlHydro, "BUFF: Mona - Prophecy of Submersion", 0.15);
    }
}

impl BuffMeta for BuffMonaC1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::MonaC1,
        chs: "Mona - Prophecy of Submersion ",
        image: BuffImage::Avatar(CharacterName::Mona),
        genre: BuffGenre::Character,
        description: Some("Mona's 1st Constellation: When any of your own party members hits an opponent affected by an Omen, the effects of Hydro-related Elemental Reactions are enhanced for 8s:<br>Electro-Charged DMG increases by 15%.<br>Vaporize DMG increases by 15%.<br>Hydro Swirl DMG increases by 15%.<br>Frozen duration is extended by 15%."),
        from: BuffFrom::Character(CharacterName::Mona),
    };

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffMonaC1)
    }
}
