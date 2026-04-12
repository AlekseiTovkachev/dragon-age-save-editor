use super::{
    AbilityListKindDto, BackpackItemReplacementDto, CharacterTargetDto, ItemMetadataPatchDto,
    SaveCommand, SaveCommandResult, SaveDocument,
};
use crate::gff4::GffFile;
use crate::gff4::fields::{SAVEGAME_BACKPACK, SAVEGAME_PARTYLIST};
use crate::test_support::{da2_save_path, dao_save_path, flat_sample_save_path};
use std::fs;
use std::path::PathBuf;

#[test]
fn serializes_and_deserializes_command_dtos() {
    let command = SaveCommand::ReplaceAbilityList {
        target: CharacterTargetDto::Companion { index: 0 },
        list: AbilityListKindDto::Talents,
        ability_ids: vec![100100, 100200],
    };

    let json = serde_json::to_string(&command).unwrap();
    let decoded: SaveCommand = serde_json::from_str(&json).unwrap();

    assert_eq!(decoded, command);
}

#[test]
fn command_execution_returns_updated_summary() {
    let mut document = SaveDocument::open(dao_save_path()).unwrap();

    let response = document
        .execute(SaveCommand::SetMoney { money: 321321 })
        .unwrap();

    match response {
        SaveCommandResult::Summary { summary } => {
            assert_eq!(summary.money, 321321);
            assert!(summary.dirty);
        }
        other => panic!("unexpected response: {other:?}"),
    }
}

#[test]
fn validate_command_reports_healthy_save_without_dirtying_document() {
    let mut document = SaveDocument::open(dao_save_path()).unwrap();

    let response = document.execute(SaveCommand::Validate).unwrap();

    match response {
        SaveCommandResult::Validation { report } => {
            assert!(report.is_valid);
        }
        other => panic!("unexpected response: {other:?}"),
    }
    let summary = document.summary();
    assert!(!summary.dirty);
}

#[test]
fn list_equipment_items_returns_items() {
    let mut document = SaveDocument::open(dao_save_path()).unwrap();

    let response = document
        .execute(SaveCommand::ListEquipmentItems {
            target: CharacterTargetDto::MainCharacter,
        })
        .unwrap();

    match response {
        SaveCommandResult::Items { items } => {
            assert!(!items.is_empty());
        }
        other => panic!("unexpected response: {other:?}"),
    }
}

#[test]
fn da2_talent_browsing_is_scoped_to_da2_rows() {
    let mut document = SaveDocument::open(da2_save_path()).unwrap();

    let response = document
        .execute(SaveCommand::ListAvailableAbilities {
            list: AbilityListKindDto::Talents,
        })
        .unwrap();

    match response {
        SaveCommandResult::AvailableAbilities { abilities, .. } => {
            assert!(abilities.iter().any(|ability| ability.id == 101000));
            assert!(!abilities.iter().any(|ability| ability.id == 23));
        }
        other => panic!("unexpected response: {other:?}"),
    }
}

#[test]
fn da2_spell_browsing_uses_da2_spell_rows() {
    let mut document = SaveDocument::open(da2_save_path()).unwrap();

    let response = document
        .execute(SaveCommand::ListAvailableAbilities {
            list: AbilityListKindDto::Spells,
        })
        .unwrap();

    match response {
        SaveCommandResult::AvailableAbilities { abilities, .. } => {
            assert!(abilities.iter().any(|ability| ability.id == 301000));
            assert!(!abilities.iter().any(|ability| ability.id == 101000));
        }
        other => panic!("unexpected response: {other:?}"),
    }
}

#[test]
fn dao_spell_browsing_includes_mage_specialization_unlocks() {
    let mut document = SaveDocument::open(dao_save_path()).unwrap();

    let response = document
        .execute(SaveCommand::ListAvailableAbilities {
            list: AbilityListKindDto::Spells,
        })
        .unwrap();

    match response {
        SaveCommandResult::AvailableAbilities { abilities, .. } => {
            for ability_id in [4012_u32, 4017, 4018, 4025] {
                assert!(abilities.iter().any(|ability| ability.id == ability_id));
            }
            assert!(!abilities.iter().any(|ability| ability.id >= 400_000));
        }
        other => panic!("unexpected response: {other:?}"),
    }
}

#[test]
fn awakening_spell_browsing_includes_awakening_mage_specializations() {
    let Some(path) = flat_sample_save_path("testingawakening.das") else {
        return;
    };
    let mut document = SaveDocument::open(path).unwrap();

    let response = document
        .execute(SaveCommand::ListAvailableAbilities {
            list: AbilityListKindDto::Spells,
        })
        .unwrap();

    match response {
        SaveCommandResult::AvailableAbilities { abilities, .. } => {
            for ability_id in [401002_u32, 401003] {
                assert!(abilities.iter().any(|ability| ability.id == ability_id));
            }
        }
        other => panic!("unexpected response: {other:?}"),
    }
}

#[test]
fn da2_character_fetch_returns_loaded_talents_and_spells() {
    let mut document = SaveDocument::open(da2_save_path()).unwrap();

    let response = document
        .execute(SaveCommand::GetCharacter {
            target: CharacterTargetDto::MainCharacter,
        })
        .unwrap();

    match response {
        SaveCommandResult::Character { character, .. } => {
            assert!(!character.talents.is_empty());
            assert!(
                !character.skills.is_empty()
                    || !character.talents.is_empty()
                    || !character.spells.is_empty()
            );
        }
        other => panic!("unexpected response: {other:?}"),
    }
}

#[test]
fn da2_plot_flags_include_full_article_30_catalog() {
    let mut document = SaveDocument::open(da2_save_path()).unwrap();

    let response = document
        .execute(SaveCommand::ListAvailablePlotFlags)
        .unwrap();

    match response {
        SaveCommandResult::AvailablePlotFlags { booleans, integers } => {
            assert_eq!(
                integers.iter().map(|flag| flag.id).collect::<Vec<_>>(),
                vec![1000, 1001]
            );
            assert!(integers.iter().all(|flag| flag.category == "Hero"));
            assert!(integers.iter().all(|flag| !flag.description.is_empty()));

            assert!(
                booleans
                    .iter()
                    .any(|flag| flag.id == 2007 && flag.description == "Connor lives")
            );
            assert!(booleans.iter().any(|flag| flag.id == 2108));
            assert_eq!(booleans.len(), 109);
            assert_eq!(
                booleans.iter().map(|flag| flag.id).collect::<Vec<_>>(),
                (2000_u16..=2108).collect::<Vec<_>>()
            );
            assert!(
                booleans
                    .iter()
                    .any(|flag| flag.id == 2072 && flag.category == "Return to Ostagar")
            );
            assert!(
                booleans.iter().any(|flag| flag.id == 2076
                    && flag.description == "Shale was recruited and survived")
            );
            assert!(
                booleans
                    .iter()
                    .any(|flag| flag.id == 2078 && flag.category == "Witch Hunt")
            );
            assert!(
                booleans
                    .iter()
                    .any(|flag| flag.id == 2103 && flag.category == "Golems of Amgarrak")
            );
            assert!(booleans.iter().all(|flag| !flag.description.is_empty()));
        }
        other => panic!("unexpected response: {other:?}"),
    }
}

#[test]
fn save_as_writes_new_file_and_keeps_original_unchanged() {
    let input = dao_save_path();
    let original = fs::read(&input).unwrap();
    let output = test_output_path("document-save-as.das");
    let mut document = SaveDocument::open(&input).unwrap();

    document
        .execute(SaveCommand::PatchItemMetadata {
            container: super::InventoryContainerDto::Backpack,
            index: 0,
            patch: ItemMetadataPatchDto {
                item_cost: Some(777),
                material: None,
                item_level: None,
            },
        })
        .unwrap();
    let response = document
        .execute(SaveCommand::SaveAs {
            output_path: output.display().to_string(),
        })
        .unwrap();

    match response {
        SaveCommandResult::Saved {
            output_path,
            summary,
        } => {
            assert_eq!(PathBuf::from(output_path), output);
            assert!(!summary.dirty);
        }
        other => panic!("unexpected response: {other:?}"),
    }
    assert_eq!(fs::read(&input).unwrap(), original);
    assert!(output.exists());
}

#[test]
fn replace_backpack_item_command_uses_same_resref_policy() {
    let mut document = SaveDocument::open(dao_save_path()).unwrap();

    let error = document
        .execute(SaveCommand::ReplaceBackpackItem {
            index: 0,
            replacement: BackpackItemReplacementDto {
                resref: "different_item".to_string(),
                item_cost: Some(1),
                material: None,
                item_level: None,
            },
        })
        .unwrap_err();

    assert_eq!(error.code, super::CommandErrorCode::BackpackResrefMismatch);
}

#[test]
fn stack_size_command_returns_updated_item_snapshot() {
    let mut document = SaveDocument::open(dao_save_path()).unwrap();
    let response = document.execute(SaveCommand::ListBackpackItems).unwrap();
    let selected_index = match response {
        SaveCommandResult::Items { items } => items
            .iter()
            .find(|entry| entry.item.stackable)
            .map(|entry| entry.index)
            .expect("expected stackable DAO backpack item"),
        other => panic!("unexpected response: {other:?}"),
    };

    let response = document
        .execute(SaveCommand::SetBackpackItemStackSize {
            index: selected_index,
            stack_size: 2,
        })
        .unwrap();

    match response {
        SaveCommandResult::Item {
            container,
            index,
            item,
        } => {
            assert_eq!(container, super::InventoryContainerDto::Backpack);
            assert_eq!(index, selected_index);
            assert_eq!(item.item_stacksize, Some(2));
            assert!(!item.category.value.is_empty());
            assert!(!item.category.label.is_empty());
        }
        other => panic!("unexpected response: {other:?}"),
    }
    assert!(document.summary().dirty);
}

#[test]
fn crafting_recipe_command_updates_recipe_ids() {
    let mut document = SaveDocument::open(dao_save_path()).unwrap();

    let response = document
        .execute(SaveCommand::ReplaceCraftingRecipeList {
            recipe_ids: vec![2, 11, 2, 20019],
        })
        .unwrap();

    match response {
        SaveCommandResult::CraftingRecipes { recipe_ids } => {
            assert_eq!(recipe_ids, vec![2, 11, 20019]);
        }
        other => panic!("unexpected response: {other:?}"),
    }
    assert!(document.summary().dirty);
}

#[test]
fn da2_available_crafting_recipes_are_named() {
    let mut document = SaveDocument::open(da2_save_path()).unwrap();

    let response = document
        .execute(SaveCommand::ListAvailableCraftingRecipes)
        .unwrap();

    match response {
        SaveCommandResult::AvailableCraftingRecipes { recipes } => {
            assert!(recipes.iter().any(|recipe| {
                recipe.id == 10000
                    && recipe.name == "Elfroot Potion"
                    && recipe.category == "Potions"
            }));
            assert!(recipes.iter().any(|recipe| {
                recipe.id == 31007
                    && recipe.name == "Devastation"
                    && recipe.category == "Weapon Runes"
            }));
        }
        other => panic!("unexpected response: {other:?}"),
    }
}

#[test]
fn dao_available_crafting_recipes_are_named() {
    let mut document = SaveDocument::open(dao_save_path()).unwrap();

    let response = document
        .execute(SaveCommand::ListAvailableCraftingRecipes)
        .unwrap();

    match response {
        SaveCommandResult::AvailableCraftingRecipes { recipes } => {
            assert!(recipes.iter().any(|recipe| {
                recipe.id == 2
                    && recipe.name == "Lesser Health Poultice Recipe"
                    && recipe.category == "Herbalism"
            }));
            assert!(recipes.iter().any(|recipe| {
                recipe.id == 78
                    && recipe.name == "Shock Trap Plans"
                    && recipe.category == "Trap-Making"
            }));
            assert!(recipes.iter().any(|recipe| {
                recipe.id == 57
                    && recipe.name == "Fleshrot Recipe"
                    && recipe.category == "Poison-Making"
            }));
        }
        other => panic!("unexpected response: {other:?}"),
    }
}

#[test]
fn dao_clone_backpack_item_command_returns_new_item_snapshot() {
    let mut document = SaveDocument::open(dao_save_path()).unwrap();
    let response = document.execute(SaveCommand::ListBackpackItems).unwrap();
    let (index, original_resref) = match response {
        SaveCommandResult::Items { items } => items
            .iter()
            .find(|entry| !entry.item.stackable)
            .map(|entry| (entry.index, entry.item.resref.clone()))
            .expect("expected non-stackable DAO backpack item"),
        other => panic!("unexpected response: {other:?}"),
    };

    let response = document
        .execute(SaveCommand::CloneBackpackItem { index })
        .unwrap();

    match response {
        SaveCommandResult::Item {
            container,
            index: cloned_index,
            item,
        } => {
            assert_eq!(container, super::InventoryContainerDto::Backpack);
            assert!(cloned_index > index);
            assert_eq!(item.resref, original_resref);
        }
        other => panic!("unexpected response: {other:?}"),
    }
    assert!(document.summary().dirty);
}

#[test]
fn da2_clone_backpack_item_command_returns_new_item_snapshot() {
    let mut document = SaveDocument::open(da2_save_path()).unwrap();
    let response = document.execute(SaveCommand::ListBackpackItems).unwrap();
    let (index, original_resref) = match response {
        SaveCommandResult::Items { items } => items
            .iter()
            .find(|entry| !entry.item.stackable)
            .map(|entry| (entry.index, entry.item.resref.clone()))
            .expect("expected non-stackable DA2 backpack item"),
        other => panic!("unexpected response: {other:?}"),
    };

    let response = document
        .execute(SaveCommand::CloneBackpackItem { index })
        .unwrap();

    match response {
        SaveCommandResult::Item {
            container,
            index: cloned_index,
            item,
        } => {
            assert_eq!(container, super::InventoryContainerDto::Backpack);
            assert!(cloned_index > index);
            assert_eq!(item.resref, original_resref);
        }
        other => panic!("unexpected response: {other:?}"),
    }
    assert!(document.summary().dirty);
}

#[test]
fn item_property_commands_update_document() {
    let mut document = SaveDocument::open(dao_save_path()).unwrap();
    let response = document.execute(SaveCommand::ListBackpackItems).unwrap();
    let index = match response {
        SaveCommandResult::Items { items } => items
            .iter()
            .position(|item| !item.item.properties.is_empty())
            .unwrap(),
        other => panic!("unexpected response: {other:?}"),
    };

    document
        .execute(SaveCommand::SetItemPropertyPower {
            container: super::InventoryContainerDto::Backpack,
            index,
            property_index: 0,
            power: 18.0,
        })
        .unwrap();
    document
        .execute(SaveCommand::AddItemProperty {
            container: super::InventoryContainerDto::Backpack,
            index,
            property_id: 3011,
            power: 7.5,
        })
        .unwrap();
    let response = document.execute(SaveCommand::ListBackpackItems).unwrap();

    match response {
        SaveCommandResult::Items { items } => {
            assert_eq!(items[index].item.properties[0].power, 18.0);
            assert_eq!(items[index].item.properties.last().unwrap().id, 3011);
        }
        other => panic!("unexpected response: {other:?}"),
    }
}

#[test]
fn item_property_id_command_returns_updated_item_snapshot() {
    let mut document = SaveDocument::open(dao_save_path()).unwrap();
    let response = document.execute(SaveCommand::ListBackpackItems).unwrap();
    let index = match response {
        SaveCommandResult::Items { items } => items
            .iter()
            .position(|item| !item.item.properties.is_empty())
            .unwrap(),
        other => panic!("unexpected response: {other:?}"),
    };

    let response = document
        .execute(SaveCommand::SetItemPropertyId {
            container: super::InventoryContainerDto::Backpack,
            index,
            property_index: 0,
            property_id: 3011,
        })
        .unwrap();

    match response {
        SaveCommandResult::Item { item, .. } => {
            assert_eq!(item.properties[0].id, 3011);
        }
        other => panic!("unexpected response: {other:?}"),
    }
    assert!(document.summary().dirty);
}

#[test]
fn validate_command_works_even_when_editor_cannot_be_built() {
    let mut raw = GffFile::from_path(dao_save_path()).unwrap();
    corrupt_first_backpack_property_power_list(&mut raw);
    let mut document = SaveDocument::from_gff("broken.das", raw).unwrap();

    let response = document.execute(SaveCommand::Validate).unwrap();

    match response {
        SaveCommandResult::Validation { report } => {
            assert!(!report.is_valid);
            assert!(report.findings.iter().any(|finding| {
                finding.code == super::ValidationCodeDto::InvalidPropertyArrayParity
            }));
        }
        other => panic!("unexpected response: {other:?}"),
    }

    let error = document
        .execute(SaveCommand::ListBackpackItems)
        .unwrap_err();
    assert_eq!(error.code, super::CommandErrorCode::Extract);
}

#[test]
fn document_assets_include_decoded_screenshot_when_available() {
    let mut document = SaveDocument::open(dao_save_path()).unwrap();
    let response = document.execute(SaveCommand::GetDocumentAssets).unwrap();

    match response {
        SaveCommandResult::DocumentAssets { assets } => {
            assert!(
                assets
                    .screenshot_data_url
                    .as_deref()
                    .is_some_and(|value| value.starts_with("data:image/png;base64,"))
            );
        }
        other => panic!("unexpected response: {other:?}"),
    }
}

fn test_output_path(name: &str) -> PathBuf {
    let dir = PathBuf::from("target").join("test-output");
    fs::create_dir_all(&dir).unwrap();
    dir.join(name)
}

fn corrupt_first_backpack_property_power_list(gff: &mut GffFile) {
    let party = gff.root_mut().get_struct_mut(SAVEGAME_PARTYLIST).unwrap();
    let items = party.get_list_mut(SAVEGAME_BACKPACK).unwrap();
    for value in items {
        let Some(item) = value.as_struct_mut() else {
            continue;
        };
        let Some(powers) = item.get_list_mut_by_name("ITEM_PROPERTY_POWERS") else {
            continue;
        };
        if !powers.is_empty() {
            powers.pop();
            return;
        }
    }
    panic!("expected backpack item with property powers");
}
