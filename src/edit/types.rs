#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterTarget {
    MainCharacter,
    Companion(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterSummary {
    pub target: CharacterTarget,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityListKind {
    Skills,
    Talents,
    Spells,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryContainer {
    Backpack,
    Equipment { target: CharacterTarget },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ItemMetadataPatch {
    pub item_cost: Option<u32>,
    pub material: Option<u32>,
    pub item_level: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackpackItemReplacement {
    pub resref: String,
    pub item_cost: Option<u32>,
    pub material: Option<u32>,
    pub item_level: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlotBooleanPatch {
    pub id: u16,
    pub value: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlotIntegerPatch {
    pub id: u16,
    pub value: i32,
}
