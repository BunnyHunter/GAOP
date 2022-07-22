use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffFreedomSworn {
    pub refine: usize
}

impl<A: Attribute> Buff<A> for BuffFreedomSworn {
    fn change_attribute(&self, attribute: &mut A) {
        let v = self.refine as f64 * 0.04 + 0.12;
        attribute.set_value_by(AttributeName::BonusNormalAttack, "BUFF: Freedom-Sworn - Revolutionary Chorale ", v);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "BUFF: Freedom-Sworn - Revolutionary Chorale ", v);
        attribute.set_value_by(AttributeName::BonusPlungingAttack, "BUFF: Freedom-Sworn - Revolutionary Chorale ", v);

        let v = self.refine as f64 * 0.05 + 0.15;
        attribute.add_atk_percentage("BUFF: Freedom-Sworn - Revolutionary Chorale ", v);
    }
}

impl BuffMeta for BuffFreedomSworn {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::FreedomSworn,
        chs: "Freedom-Sworn - Revolutionary Chorale ",
        image: BuffImage::Weapon(WeaponName::FreedomSworn),
        genre: BuffGenre::Weapon,
        description: Some("Revolutionary Chorale: A part of the 'Millennial Movement' that wanders amidst the winds. Increases DMG by 10/12.5/15/17.5/20%. When the character wielding this weapon triggers Elemental Reactions, they gain a Sigil of Rebellion. This effect can be triggered once every 0.5s and can be triggered even if said character is not on the field. When you possess 2 Sigils of Rebellion, all of them will be consumed and all nearby party members will obtain 'Millennial Movement: Song of Resistance' for 12s. 'Millennial Movement: Song of Resistance' increases Normal, Charged, and Plunging Attack DMG by 16/20/24/28/32% and increases ATK by 20/25/30/35/40%. Once this effect is triggered, you will not gain Sigils of Rebellion for 20s. Of the many effects of the 'Millennial Movement,' buffs of the same type will not stack."),
        from: BuffFrom::Weapon(WeaponName::FreedomSworn),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let refine = match *b {
            BuffConfig::FreedomSworn { refine } => refine,
            _ => 1
        };

        Box::new(BuffFreedomSworn {
            refine
        })
    }
}
