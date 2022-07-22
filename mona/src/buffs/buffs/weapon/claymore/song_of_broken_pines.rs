use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffSongOfBrokenPines {
    pub refine: usize,
}

impl<A: Attribute> Buff<A> for BuffSongOfBrokenPines {
    fn change_attribute(&self, attribute: &mut A) {
        let v = self.refine as f64 * 0.05 + 0.15;
        attribute.add_atk_percentage("BUFF: Song of Broken Pines - Rebel’s Banner-Hymn", v);

        let v = self.refine as f64 * 0.03 + 0.09;
        attribute.set_value_by(AttributeName::SpeedNormalAttack, "BUFF: Song of Broken Pines - Rebel’s Banner-Hymn", v);
    }
}

impl BuffMeta for BuffSongOfBrokenPines {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::SongOfBrokenPines,
        chs: "Song of Broken Pines - Rebel’s Banner-Hymn",
        image: BuffImage::Weapon(WeaponName::SongOfBrokenPines),
        genre: BuffGenre::Weapon,
        description: Some("Rebel’s Banner-Hymn: A part of the “Millennial Movement” that wanders amidst the winds. Increases ATK by 16/20/24/28/32%, and when Normal or Charged Attacks hit opponents, the character gains a Sigil of Whispers. This effect can be triggered once every 0.3s.<br>When you possess 4 Sigils of Whispers, all of them will be consumed and all nearby party members will obtain the “Millennial Movement: Banner-Hymn” effect for 12s.”Millennial Movement: Banner-Hymn” increases Normal ATK SPD by 12/15/18/21/24% and increases ATK by 20/25/30/35/40%. Once this effect is triggered, you will not gain Sigils of Whispers for 20s. Of the many effects of the “Millennial Movement,” buffs of the same type will not stack."),
        from: BuffFrom::Weapon(WeaponName::SongOfBrokenPines),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let refine = match *b {
            BuffConfig::SongOfBrokenPines { refine } => refine,
            _ => 1
        };

        Box::new(BuffSongOfBrokenPines {
            refine
        })
    }
}
