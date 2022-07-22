use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::Element;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffHakushinRing {
    pub refine: usize,
    pub element: Element
}

impl<A: Attribute> Buff<A> for BuffHakushinRing {
    fn change_attribute(&self, attribute: &mut A) {
        let bonus_name = AttributeName::bonus_name_by_element(self.element);
        let refine = self.refine as f64;
        attribute.set_value_by(bonus_name, "BUFF: Hakushin Ring - Sakura Saiguu", refine * 0.025 + 0.075);
    }
}

impl BuffMeta for BuffHakushinRing {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::HakushinRing,
        chs: "Hakushin Ring - Sakura Saiguu",
        image: BuffImage::Weapon(WeaponName::HakushinRing),
        genre: BuffGenre::Weapon,
        description: Some("Sakura Saiguu: Increase Energy recharge by 24/30/36/42/48% for 10s after using an Elemental Skill."),
        from: BuffFrom::Weapon(WeaponName::HakushinRing)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE,
        ItemConfig {
            name: "element",
            title: "Element",
            config: ItemConfigType::Element8 { default: Element::Electro }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (refine, element) = match *b {
            BuffConfig::HakushinRing { refine, element } => (refine, element),
            _ => (1, Element::Electro)
        };

        Box::new(BuffHakushinRing {
            refine, element
        })
    }
}