use crate::attribute::{Attribute, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffWolfsGravestone {
    pub refine: usize
}

impl<A: Attribute> Buff<A> for BuffWolfsGravestone {
    fn change_attribute(&self, attribute: &mut A) {
        let v = self.refine as f64 * 0.1 + 0.3;
        attribute.add_atk_percentage("BUFF: Wolfs Gravestone - Wolfish Tracker ", v);
    }
}

impl BuffMeta for BuffWolfsGravestone {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::WolfsGravestone,
        chs: "Wolfs Gravestone - Wolfish Tracker ",
        image: BuffImage::Weapon(WeaponName::WolfsGravestone),
        genre: BuffGenre::Weapon,
        description: Some("Wolfish Tracker: Increases ATK by 20/25/30/35/40%. On hit, attacks against enemies with less than 30% HP increase all party members' Base ATK by 40/50/60/70/80% for 12s. Can only occur once every 30s."),
        from: BuffFrom::Weapon(WeaponName::WolfsGravestone),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let refine = match *b {
            BuffConfig::WolfsGravestone { refine } => refine,
            _ => 1
        };

        Box::new(BuffWolfsGravestone {
            refine
        })
    }
}
