use crate::attribute::{Attribute, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffThrillingTalesOfDragonSlayers {
    pub refine: usize,
}

impl<A: Attribute> Buff<A> for BuffThrillingTalesOfDragonSlayers {
    fn change_attribute(&self, attribute: &mut A) {
        let v = self.refine as f64 * 0.06 + 0.18;
        attribute.add_atk_percentage("BUFF: Thrilling Tales of Dragon Slayers - Heritage ", v);
    }
}

impl BuffMeta for BuffThrillingTalesOfDragonSlayers {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ThrillingTalesOfDragonSlayers,
        chs: "Thrilling Tales of Dragon Slayers - Heritage ",
        image: BuffImage::Weapon(WeaponName::ThrillingTalesOfDragonSlayers),
        genre: BuffGenre::Weapon,
        description: Some("Heritage: When switching characters, the new character taking the field has their ATK increased by 24/30/36/42/48% for 10s."),
        from: BuffFrom::Weapon(WeaponName::ThrillingTalesOfDragonSlayers),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let refine = match *b {
            BuffConfig::ThrillingTalesOfDragonSlayers { refine } => refine,
            _ => 1
        };

        Box::new(BuffThrillingTalesOfDragonSlayers {
            refine
        })
    }
}

