pub(crate) struct StaticCraftingRecipe {
    pub(crate) id: u32,
    pub(crate) name: &'static str,
    pub(crate) category: &'static str,
}

pub(crate) fn available_crafting_recipes(
    game: Option<crate::domain::gamedata::GameId>,
) -> &'static [StaticCraftingRecipe] {
    match game {
        Some(
            crate::domain::gamedata::GameId::Dao | crate::domain::gamedata::GameId::DaoAwakening,
        ) => DAO_CRAFTING_RECIPES,
        Some(crate::domain::gamedata::GameId::Da2) => DA2_CRAFTING_RECIPES,
        _ => &[],
    }
}

const DAO_CRAFTING_RECIPES: &[StaticCraftingRecipe] = &[
    StaticCraftingRecipe {
        id: 2,
        name: "Lesser Health Poultice Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 7,
        name: "Health Poultice Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 18,
        name: "Greater Health Poultice Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 90,
        name: "Potent Health Poultice Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 11,
        name: "Lesser Lyrium Potion Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 22,
        name: "Lyrium Potion Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 31,
        name: "Greater Lyrium Potion Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 91,
        name: "Potent Lyrium Potion Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 1,
        name: "Mabari Crunch Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 6,
        name: "Double-Baked Mabari Crunch Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 14,
        name: "Lesser Elixir of Grounding Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 34,
        name: "Greater Elixir of Grounding Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 12,
        name: "Lesser Ice Salve Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 32,
        name: "Greater Ice Salve Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 15,
        name: "Lesser Nature Salve Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 35,
        name: "Greater Nature Salve Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 27,
        name: "Lesser Spirit Balm Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 36,
        name: "Greater Spirit Balm Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 13,
        name: "Lesser Warmth Balm Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 33,
        name: "Greater Warmth Balm Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 16,
        name: "Lesser Injury Kit Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 28,
        name: "Injury Kit Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 37,
        name: "Greater Injury Kit Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 89,
        name: "Dwarven Regicide Antidote Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 5,
        name: "Incense of Awareness Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 4,
        name: "Rock Salve Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 17,
        name: "Swift Salve Recipe",
        category: "Herbalism",
    },
    StaticCraftingRecipe {
        id: 79,
        name: "Acidic Grease Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 68,
        name: "Acidic Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 85,
        name: "Soulrot Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 72,
        name: "Mild Lure Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 82,
        name: "Interesting Lure Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 87,
        name: "Overpowering Lure Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 67,
        name: "Spring Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 80,
        name: "Poisoned Caltrop Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 65,
        name: "Small Caltrop Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 66,
        name: "Small Claw Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 64,
        name: "Small Grease Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 73,
        name: "Small Shrapnel Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 70,
        name: "Large Caltrop Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 74,
        name: "Large Claw Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 69,
        name: "Large Grease Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 83,
        name: "Large Shrapnel Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 71,
        name: "Mild Choking Powder Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 86,
        name: "Choking Powder Cloud Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 81,
        name: "Choking Powder Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 75,
        name: "Mild Sleeping Gas Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 84,
        name: "Sleeping Gas Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 88,
        name: "Sleeping Gas Cloud Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 76,
        name: "Fire Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 77,
        name: "Freeze Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 78,
        name: "Shock Trap Plans",
        category: "Trap-Making",
    },
    StaticCraftingRecipe {
        id: 41,
        name: "Acid Flask Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 42,
        name: "Acidic Coating Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 39,
        name: "Venom Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 43,
        name: "Concentrated Venom Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 54,
        name: "Adder's Kiss Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 63,
        name: "Quiet Death Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 44,
        name: "Crow Poison Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 56,
        name: "Concentrated Crow Poison Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 40,
        name: "Deathroot Extract Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 45,
        name: "Concentrated Deathroot Extract Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 55,
        name: "Demonic Poison Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 62,
        name: "Concentrated Demonic Poison Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 47,
        name: "Magebane Poison Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 59,
        name: "Concentrated Magebane Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 46,
        name: "Soldier's Bane Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 58,
        name: "Concentrated Soldier's Bane Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 48,
        name: "Fire Bomb Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 51,
        name: "Flaming Coating Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 49,
        name: "Freeze Bomb Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 52,
        name: "Freezing Coating Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 50,
        name: "Shock Bomb Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 53,
        name: "Shock Coating Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 60,
        name: "Soulrot Bomb Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 61,
        name: "Soulrot Coating Recipe",
        category: "Poison-Making",
    },
    StaticCraftingRecipe {
        id: 57,
        name: "Fleshrot Recipe",
        category: "Poison-Making",
    },
];

const DA2_CRAFTING_RECIPES: &[StaticCraftingRecipe] = &[
    StaticCraftingRecipe {
        id: 10000,
        name: "Elfroot Potion",
        category: "Potions",
    },
    StaticCraftingRecipe {
        id: 10001,
        name: "Elixir of Purity",
        category: "Potions",
    },
    StaticCraftingRecipe {
        id: 10003,
        name: "Restoration Potion",
        category: "Potions",
    },
    StaticCraftingRecipe {
        id: 10004,
        name: "Life Ward Potion",
        category: "Potions",
    },
    StaticCraftingRecipe {
        id: 10005,
        name: "Rock Armor Potion",
        category: "Potions",
    },
    StaticCraftingRecipe {
        id: 10006,
        name: "Mighty Offense Potion",
        category: "Potions",
    },
    StaticCraftingRecipe {
        id: 10007,
        name: "Elixir of Heroism",
        category: "Potions",
    },
    StaticCraftingRecipe {
        id: 20000,
        name: "Combustion Grenade",
        category: "Grenades",
    },
    StaticCraftingRecipe {
        id: 20001,
        name: "Tar Bomb",
        category: "Grenades",
    },
    StaticCraftingRecipe {
        id: 20002,
        name: "Mythal's Favor",
        category: "Grenades",
    },
    StaticCraftingRecipe {
        id: 20003,
        name: "Fell Grenade",
        category: "Grenades",
    },
    StaticCraftingRecipe {
        id: 21000,
        name: "Debilitating Poison",
        category: "Poisons",
    },
    StaticCraftingRecipe {
        id: 21001,
        name: "Crow Venom",
        category: "Poisons",
    },
    StaticCraftingRecipe {
        id: 21002,
        name: "Deathroot Toxin",
        category: "Poisons",
    },
    StaticCraftingRecipe {
        id: 21003,
        name: "Arcane Poison",
        category: "Poisons",
    },
    StaticCraftingRecipe {
        id: 21004,
        name: "Fell Poison",
        category: "Poisons",
    },
    StaticCraftingRecipe {
        id: 30000,
        name: "Protection",
        category: "Armor Runes",
    },
    StaticCraftingRecipe {
        id: 30001,
        name: "Fortune",
        category: "Armor Runes",
    },
    StaticCraftingRecipe {
        id: 30002,
        name: "Frost Warding",
        category: "Armor Runes",
    },
    StaticCraftingRecipe {
        id: 30003,
        name: "Fire Warding",
        category: "Armor Runes",
    },
    StaticCraftingRecipe {
        id: 30004,
        name: "Lightning Warding",
        category: "Armor Runes",
    },
    StaticCraftingRecipe {
        id: 30005,
        name: "Nature Warding",
        category: "Armor Runes",
    },
    StaticCraftingRecipe {
        id: 30006,
        name: "Spirit Warding",
        category: "Armor Runes",
    },
    StaticCraftingRecipe {
        id: 30007,
        name: "Defense",
        category: "Armor Runes",
    },
    StaticCraftingRecipe {
        id: 30008,
        name: "Valiance",
        category: "Armor Runes",
    },
    StaticCraftingRecipe {
        id: 31000,
        name: "Impact",
        category: "Weapon Runes",
    },
    StaticCraftingRecipe {
        id: 31001,
        name: "Frost",
        category: "Weapon Runes",
    },
    StaticCraftingRecipe {
        id: 31002,
        name: "Fire",
        category: "Weapon Runes",
    },
    StaticCraftingRecipe {
        id: 31003,
        name: "Lightning",
        category: "Weapon Runes",
    },
    StaticCraftingRecipe {
        id: 31004,
        name: "Nature",
        category: "Weapon Runes",
    },
    StaticCraftingRecipe {
        id: 31005,
        name: "Spirit",
        category: "Weapon Runes",
    },
    StaticCraftingRecipe {
        id: 31006,
        name: "Striking",
        category: "Weapon Runes",
    },
    StaticCraftingRecipe {
        id: 31007,
        name: "Devastation",
        category: "Weapon Runes",
    },
];
