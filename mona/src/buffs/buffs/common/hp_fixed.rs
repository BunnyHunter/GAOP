use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;

pub struct BuffHPFixed {
    pub value: f64
}

impl<A: Attribute> Buff<A> for BuffHPFixed {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::HPFixed, "BUFF: HP", self.value);
    }
}

impl BuffMeta for BuffHPFixed {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::HPFixed,
        chs: "HP",
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
            BuffConfig::HPFixed { value } => value,
            _ => 0.0
        };
        Box::new(BuffHPFixed {
            value
        })
    }
}
