use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{CharacterConfig, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::common::ChangeAttribute;
use crate::common::item_config_type::ItemConfig;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

#[derive(Clone)]
pub struct CharacterSkillMapItem {
    pub index: usize,
    pub chs: &'static str,
}

pub struct CharacterSkillMap {
    pub skill1: Option<&'static [CharacterSkillMapItem]>,
    pub skill2: Option<&'static [CharacterSkillMapItem]>,
    pub skill3: Option<&'static [CharacterSkillMapItem]>,
}

pub trait CharacterTrait {
//character metadata
    const STATIC_DATA: CharacterStaticData;
    //character skill type
    type SkillType;
    //Character skill value constant
    const SKILL: Self::SkillType;
    //Character damage key
    type DamageEnumType: Copy + Clone + Into<usize>;
    //character positioning enumeration
    type RoleEnum;

    //character skill mapping
    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap;

    //character parameters
    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = None;
    //character skill parameters
    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = None;

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig) -> D::Result;

    fn damage<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: Self::DamageEnumType, config: &CharacterSkillConfig) -> D::Result {
        Self::damage_internal::<D>(context, s.into(), config)
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>>;

    fn get_target_function_by_role(
        role_index: usize,
        team: &TeamQuantization,
        c: &CharacterCommonData,
        w: &WeaponCommonData
    ) -> Box<dyn TargetFunction>;
}
