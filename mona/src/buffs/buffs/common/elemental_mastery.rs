use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffElementalMastery {
    pub value: f64
}

impl<A: Attribute> Buff<A> for BuffElementalMastery {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "BUFF: Elemental Mastery", self.value);
    }
}

impl BuffMeta for BuffElementalMastery {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ElementalMastery,
        chs: "Elemental Mastery",
        image: BuffImage::Misc("sword"),
        genre: BuffGenre::Common,
        description: None,
        from: BuffFrom::Common
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::BUFFV1
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let value = match *b {
            BuffConfig::ElementalMastery { value } => value,
            _ => 0.0
        };

        Box::new(BuffElementalMastery {
            value
        })
    }
}
