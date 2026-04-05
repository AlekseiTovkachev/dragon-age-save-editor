use crate::domain::ability::AbilityRef;
use crate::domain::item::Item;
use crate::domain::stats::{CoreStats, PointPools};

#[derive(Debug, Clone, PartialEq)]
pub struct Character {
    pub name: String,
    pub template_resref: Option<String>,
    pub approval: Option<i32>,
    pub level: Option<u32>,
    pub core_stats: CoreStats,
    pub point_pools: PointPools,
    pub equipment: Vec<Item>,
    pub skills: Vec<AbilityRef>,
    pub talents: Vec<AbilityRef>,
    pub spells: Vec<AbilityRef>,
}
