#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialFamily {
    Metal,
    Wood,
    Leather,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialTarget {
    Armor,
    Weapon,
    Shield,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialProfile {
    pub family: MaterialFamily,
    pub target: MaterialTarget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialInfo {
    pub code: u32,
    pub tier: u8,
    pub name: String,
    pub family: MaterialFamily,
    pub target: MaterialTarget,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemProperty {
    pub id: u32,
    pub name: Option<String>,
    pub power: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub resref: Option<String>,
    pub name: Option<String>,
    pub object_id: Option<i32>,
    pub equipment_slot: Option<u32>,
    pub item_cost: Option<u32>,
    pub item_stacksize: Option<u32>,
    pub item_level: Option<u8>,
    pub material: Option<u32>,
    pub material_profile: Option<MaterialProfile>,
    pub material_info: Option<MaterialInfo>,
    pub properties: Vec<ItemProperty>,
}
