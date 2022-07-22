use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffElegyOfTheEnd {
    pub refine: usize,
}

impl<A: Attribute> Buff<A> for BuffElegyOfTheEnd {
    fn change_attribute(&self, attribute: &mut A) {
        let v = self.refine * 25 + 75;
        attribute.set_value_by(AttributeName::ElementalMastery, "Elegy For The End - The Parting Refrain", v as f64);

        let v = (self.refine * 5 + 15) as f64 / 100.0;
        attribute.add_atk_percentage("Elegy For The End - The Parting Refrain", v);
    }
}

impl BuffMeta for BuffElegyOfTheEnd {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ElegyOfTheEnd,
        chs: "Elegy For The End - The Parting Refrain ",
        image: BuffImage::Weapon(WeaponName::ElegyOfTheEnd),
        genre: BuffGenre::Weapon,
        description: Some("The Parting Realm：A part of the 'Millennial Movement' that wanders amidst the winds. Increases Elemental Mastery by 60/75/90/105/120. When the Elemental Skills or Elemental Bursts of the character wielding this weapon hit opponents, that character gains a Sigil of Remembrance. This effect can be triggered once every 0.2s and can be triggered even if said character is not on the field. When you possess 4 Sigils of Remembrance, all of them will be consumed and all nearby party members will obtain the 'Millennial Movement: Farewell Song' effect for 12s. 'Millennial Movement: Farewell Song' increases Elemental Mastery by 100/125/150/175/200 and increases ATK by 20/25/30/35/40%. Once this effect is triggered, you will not gain Sigils of Remembrance for 20s. Of the many effects of the 'Millennial Movement' buffs of the same type will not stack."),
        from: BuffFrom::Weapon(WeaponName::ElegyOfTheEnd),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let refine = match *b {
            BuffConfig::ElegyOfTheEnd { refine } => refine,
            _ => 1
        };

        Box::new(BuffElegyOfTheEnd {
            refine
        })
    }
}
