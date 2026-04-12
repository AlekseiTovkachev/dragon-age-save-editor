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

macro_rules! item_categories {
    ($($variant:ident => ($db:literal, $label:literal),)+) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum ItemCategory {
            $($variant,)+
        }

        impl ItemCategory {
            pub fn as_db_value(self) -> &'static str {
                match self {
                    $(Self::$variant => $db,)+
                }
            }

            pub fn from_db_value(value: &str) -> Self {
                match value {
                    $($db => Self::$variant,)+
                    _ => Self::Uncategorized,
                }
            }

            pub fn label(self) -> &'static str {
                match self {
                    $(Self::$variant => $label,)+
                }
            }
        }
    };
}

item_categories! {
    AccessoriesAmulets => ("accessories.amulets", "Accessories > Amulets"),
    AccessoriesBelts => ("accessories.belts", "Accessories > Belts"),
    AccessoriesRings => ("accessories.rings", "Accessories > Rings"),
    ArmorBoots => ("armor.boots", "Armor > Boots"),
    ArmorBootsHeavy => ("armor.boots.heavy", "Armor > Boots > Heavy"),
    ArmorBootsLight => ("armor.boots.light", "Armor > Boots > Light"),
    ArmorBootsMedium => ("armor.boots.medium", "Armor > Boots > Medium"),
    ArmorChestpieces => ("armor.chestpieces", "Armor > Chestpieces"),
    ArmorChestpiecesHeavy => ("armor.chestpieces.heavy", "Armor > Chestpieces > Heavy"),
    ArmorChestpiecesLight => ("armor.chestpieces.light", "Armor > Chestpieces > Light"),
    ArmorChestpiecesMedium => ("armor.chestpieces.medium", "Armor > Chestpieces > Medium"),
    ArmorGloves => ("armor.gloves", "Armor > Gloves"),
    ArmorGlovesHeavy => ("armor.gloves.heavy", "Armor > Gloves > Heavy"),
    ArmorGlovesLight => ("armor.gloves.light", "Armor > Gloves > Light"),
    ArmorGlovesMedium => ("armor.gloves.medium", "Armor > Gloves > Medium"),
    ArmorHelmets => ("armor.helmets", "Armor > Helmets"),
    ArmorHelmetsHeavy => ("armor.helmets.heavy", "Armor > Helmets > Heavy"),
    ArmorHelmetsLight => ("armor.helmets.light", "Armor > Helmets > Light"),
    ArmorHelmetsMedium => ("armor.helmets.medium", "Armor > Helmets > Medium"),
    Clothing => ("clothing", "Clothing"),
    CompanionItemsArmor => ("companion_items.armor", "Companion Items > Armor"),
    CompanionItemsArmorUpgrades => ("companion_items.armor_upgrades", "Companion Items > Armor Upgrades"),
    CompanionItemsGifts => ("companion_items.gifts", "Companion Items > Gifts"),
    CompanionItemsRomance => ("companion_items.romance", "Companion Items > Romance"),
    CompanionItemsWeapons => ("companion_items.weapons", "Companion Items > Weapons"),
    ConsumablesBackpacks => ("consumables.backpacks", "Consumables > Backpacks"),
    ConsumablesBombs => ("consumables.bombs", "Consumables > Bombs"),
    ConsumablesFoodForDog => ("consumables.food_for_dog", "Consumables > Food for Dog"),
    ConsumablesHealthPoultices => ("consumables.health_poultices", "Consumables > Health Poultices"),
    ConsumablesInjuryKits => ("consumables.injury_kits", "Consumables > Injury Kits"),
    ConsumablesLyriumPotions => ("consumables.lyrium_potions", "Consumables > Lyrium Potions"),
    ConsumablesPoisons => ("consumables.poisons", "Consumables > Poisons"),
    ConsumablesPotions => ("consumables.potions", "Consumables > Potions"),
    ConsumablesSalvesIncensesBalms => ("consumables.salves_incenses_balms", "Consumables > Salves, Incenses, Balms"),
    ConsumablesTomes => ("consumables.tomes", "Consumables > Tomes"),
    ConsumablesTraps => ("consumables.traps", "Consumables > Traps"),
    ConsumablesUnique => ("consumables.unique", "Consumables > Unique"),
    ConsumablesWeaponCoating => ("consumables.weapon_coating", "Consumables > Weapon Coating"),
    CraftingReagents => ("crafting.reagents", "Crafting > Reagents"),
    CraftingRecipes => ("crafting.recipes", "Crafting > Recipes"),
    CraftingRecipesHerbalism => ("crafting.recipes.herbalism", "Crafting > Recipes > Herbalism"),
    CraftingRecipesPoisonMaking => ("crafting.recipes.poison_making", "Crafting > Recipes > Poison-Making"),
    CraftingRecipesTrapMakingPlans => ("crafting.recipes.trap_making_plans", "Crafting > Recipes > Trap-Making Plans"),
    CraftingRecipesUnique => ("crafting.recipes.unique", "Crafting > Recipes > Unique"),
    CraftingResources => ("crafting.resources", "Crafting > Resources"),
    CraftingRunecraftingTracingsArmorRuneTracings => ("crafting.runecrafting_tracings.armor_rune_tracings", "Crafting > Runecrafting Tracings > Armor Rune Tracings"),
    CraftingRunecraftingTracingsWeaponRuneTracings => ("crafting.runecrafting_tracings.weapon_rune_tracings", "Crafting > Runecrafting Tracings > Weapon Rune Tracings"),
    CraftingTrapMakingPlans => ("crafting.trap_making_plans", "Crafting > Trap-Making Plans"),
    DogAccessoriesCollars => ("dog_accessories.collars", "Dog Accessories > Collars"),
    DogAccessoriesKaddis => ("dog_accessories.kaddis", "Dog Accessories > Kaddis"),
    Gems => ("gems", "Gems"),
    GenericItems => ("generic_items", "Generic Items"),
    Gifts => ("gifts", "Gifts"),
    Junk => ("junk", "Junk"),
    MageRobes => ("mage_robes", "Mage Robes"),
    Manuals => ("manuals", "Manuals"),
    Miscellaneous => ("miscellaneous", "Miscellaneous"),
    NpcItems => ("npc_items", "NPC Items"),
    PlotItems => ("plot_items", "Plot Items"),
    PlotItemsFindAndReturnQuestItems => ("plot_items.find_and_return_quest_items", "Plot Items > Find and Return Quest Items"),
    PlotItemsOther => ("plot_items.other", "Plot Items > Other"),
    Runes => ("runes", "Runes"),
    RunesArmorRunes => ("runes.armor_runes", "Runes > Armor Runes"),
    RunesWeaponRunes => ("runes.weapon_runes", "Runes > Weapon Runes"),
    ShaleCrystalsLarge => ("shale_crystals.large", "Shale Crystals > Large Crystals"),
    ShaleCrystalsSmall => ("shale_crystals.small", "Shale Crystals > Small Crystals"),
    ShieldHeraldries => ("shield_heraldries", "Shield Heraldries"),
    Shields => ("shields", "Shields"),
    ShieldsBucklers => ("shields.bucklers", "Shields > Bucklers"),
    ShieldsHeavy => ("shields.heavy", "Shields > Heavy"),
    ShieldsKite => ("shields.kite", "Shields > Kite"),
    ShieldsTarges => ("shields.targes", "Shields > Targes"),
    Tomes => ("tomes", "Tomes"),
    Uncategorized => ("uncategorized", "Uncategorized"),
    UnusableEnemyWeapons => ("unusable_enemy_weapons", "Unusable Enemy Weapons"),
    WeaponsArrowsBolts => ("weapons.arrows_bolts", "Weapons > Arrows & Bolts"),
    WeaponsBattleaxes => ("weapons.battleaxes", "Weapons > Battleaxes"),
    WeaponsBows => ("weapons.bows", "Weapons > Bows"),
    WeaponsCrossbows => ("weapons.crossbows", "Weapons > Crossbows"),
    WeaponsDaggers => ("weapons.daggers", "Weapons > Daggers"),
    WeaponsGreatswords => ("weapons.greatswords", "Weapons > Greatswords"),
    WeaponsLongbows => ("weapons.longbows", "Weapons > Longbows"),
    WeaponsLongswords => ("weapons.longswords", "Weapons > Longswords"),
    WeaponsMaces => ("weapons.maces", "Weapons > Maces"),
    WeaponsMauls => ("weapons.mauls", "Weapons > Mauls"),
    WeaponsOneHanded => ("weapons.one_handed", "Weapons > One-Handed"),
    WeaponsShortbows => ("weapons.shortbows", "Weapons > Shortbows"),
    WeaponsStaves => ("weapons.staves", "Weapons > Staves"),
    WeaponsTwoHanded => ("weapons.two_handed", "Weapons > Two-Handed"),
    WeaponsWaraxes => ("weapons.waraxes", "Weapons > Waraxes"),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemCatalogEntry {
    pub name: Option<String>,
    pub wiki_url: Option<String>,
    pub category: ItemCategory,
    pub stackable: bool,
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
    pub wiki_url: Option<String>,
    pub category: ItemCategory,
    pub stackable: bool,
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
