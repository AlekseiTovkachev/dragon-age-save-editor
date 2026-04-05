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
    pub properties: Vec<ItemProperty>,
}
