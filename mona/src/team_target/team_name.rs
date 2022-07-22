use num_derive::FromPrimitive;
use mona_derive::{TeamTargetFunctionData, EnumLen};

#[derive(Copy, Clone)]
#[derive(FromPrimitive, TeamTargetFunctionData, EnumLen)]
pub enum TeamName {
    // Test,
    // RaidenXianglingBennettXingqiu,      // Electro神国家队
    RaidenKujouKazuhaBennett,           // Electro九万班
    AyakaRosariaKokomiKazuha,           // 神罗心万
}
