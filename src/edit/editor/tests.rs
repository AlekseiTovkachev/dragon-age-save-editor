use super::{
    AbilityListKind, BackpackItemReplacement, CharacterTarget, EditError, InventoryContainer,
    ItemMetadataPatch, PlotBooleanPatch, PlotIntegerPatch, SaveEditor,
};
use crate::domain::gamedata::{DEFAULT_GAME_DATA_PATH, GameDataLookup, GameId, SqliteGameData};
use crate::domain::save::SaveGame;
use crate::domain::stats::{CoreStat, CoreStatsPatch, PointPoolsPatch};
use crate::gff4::GffFile;
use crate::gff4::fields::{SAVEGAME_MONEY, SAVEGAME_PARTYLIST};
use crate::test_support::{da2_save_path, dao_save_path};
use std::fs;
use std::path::PathBuf;

#[test]
fn lists_characters_with_stable_targets() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let editor = SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
    let characters = editor.list_characters();
    assert_eq!(characters[0].target, CharacterTarget::MainCharacter);
    assert_eq!(characters[1].target, CharacterTarget::Companion(0));
}

#[test]
fn setting_money_updates_expected_field() {
    let mut editor = SaveEditor::from_path(dao_save_path()).unwrap();
    editor.set_money(424242).unwrap();
    let party = editor.raw().root.get_struct(SAVEGAME_PARTYLIST).unwrap();
    assert_eq!(
        party.get(SAVEGAME_MONEY).and_then(super::value_to_u32),
        Some(424242)
    );
}

#[test]
fn editing_companion_stats_targets_by_index() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let mut editor =
        SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
    let original_second = editor.save().companions[1].core_stats.magic;
    editor
        .patch_character_core_stats(
            CharacterTarget::Companion(0),
            CoreStatsPatch {
                magic: Some(77),
                ..CoreStatsPatch::default()
            },
        )
        .unwrap();
    assert_eq!(editor.save().companions[0].core_stats.magic, 77);
    assert_eq!(
        editor.save().companions[1].core_stats.magic,
        original_second
    );
}

#[test]
fn missing_stat_row_returns_explicit_error() {
    let mut editor = SaveEditor::from_path(dao_save_path()).unwrap();
    remove_stat_row(&mut editor, CharacterTarget::MainCharacter, 1);
    let error = editor
        .set_character_stat(CharacterTarget::MainCharacter, CoreStat::Strength, 88)
        .unwrap_err();
    match error {
        EditError::MissingStatRow {
            target: CharacterTarget::MainCharacter,
            stat_id: 1,
        } => {}
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn replaces_skill_list_and_preserves_order() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let mut editor =
        SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
    let original = editor
        .save()
        .main_character
        .skills
        .iter()
        .map(|ability| ability.id)
        .collect::<Vec<_>>();
    let replacement = vec![original[1], original[0]];
    editor
        .replace_character_abilities(
            CharacterTarget::MainCharacter,
            AbilityListKind::Skills,
            &replacement,
            &lookup,
        )
        .unwrap();
    assert_eq!(
        editor
            .save()
            .main_character
            .skills
            .iter()
            .map(|ability| ability.id)
            .collect::<Vec<_>>(),
        replacement
    );
}

#[test]
fn rejects_cross_type_ability_replacement() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let mut editor =
        SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
    let spell_id = editor
        .save()
        .main_character
        .talents
        .first()
        .map(|ability| ability.id)
        .unwrap();
    let error = editor
        .replace_character_abilities(
            CharacterTarget::MainCharacter,
            AbilityListKind::Skills,
            &[spell_id],
            &lookup,
        )
        .unwrap_err();
    match error {
        EditError::InvalidAbilityKind { .. } => {}
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn replaces_da2_talent_list_with_valid_da2_ids() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let replacement = vec![101000, 101001, 101010];
    let abilities = super::load_validated_abilities(
        CharacterTarget::MainCharacter,
        AbilityListKind::Talents,
        &replacement,
        &std::collections::BTreeSet::new(),
        &lookup,
        Some(crate::domain::gamedata::GameId::Da2),
    )
    .unwrap();

    assert_eq!(
        abilities
            .iter()
            .map(|ability| ability.id)
            .collect::<Vec<_>>(),
        replacement
    );
}

#[test]
fn specialization_talent_requires_specialization_core() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let current_ids = std::collections::BTreeSet::from([23_u32]);
    let error = super::load_validated_abilities(
        CharacterTarget::MainCharacter,
        AbilityListKind::Talents,
        &[23],
        &current_ids,
        &lookup,
        Some(crate::domain::gamedata::GameId::Dao),
    )
    .unwrap_err();

    match error {
        EditError::MissingCoreAbility {
            required_id: 4021, ..
        } => {}
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn archery_talent_accepts_either_rogue_or_warrior_core() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let abilities = super::load_validated_abilities(
        CharacterTarget::MainCharacter,
        AbilityListKind::Talents,
        &[4020, 3071],
        &std::collections::BTreeSet::new(),
        &lookup,
        Some(crate::domain::gamedata::GameId::Dao),
    )
    .unwrap();

    assert_eq!(
        abilities
            .iter()
            .map(|ability| ability.id)
            .collect::<Vec<_>>(),
        vec![4020, 3071]
    );
}

#[test]
fn core_talent_can_be_removed_when_dependents_are_removed() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let current_ids = std::collections::BTreeSet::from([4022_u32, 1_u32]);
    let abilities = super::load_validated_abilities(
        CharacterTarget::MainCharacter,
        AbilityListKind::Talents,
        &[],
        &current_ids,
        &lookup,
        Some(crate::domain::gamedata::GameId::Dao),
    )
    .unwrap();

    assert!(abilities.is_empty());
}

#[test]
fn coercion_requires_player_skill_unlock() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let error = super::load_validated_abilities(
        CharacterTarget::MainCharacter,
        AbilityListKind::Skills,
        &[100011],
        &std::collections::BTreeSet::new(),
        &lookup,
        Some(crate::domain::gamedata::GameId::Dao),
    )
    .unwrap_err();

    match error {
        EditError::MissingCoreAbility {
            required_id: 4001, ..
        } => {}
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn humanoid_skill_requires_humanoid_skill_unlock() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let error = super::load_validated_abilities(
        CharacterTarget::MainCharacter,
        AbilityListKind::Skills,
        &[100021],
        &std::collections::BTreeSet::new(),
        &lookup,
        Some(crate::domain::gamedata::GameId::Dao),
    )
    .unwrap_err();

    match error {
        EditError::MissingCoreAbility {
            required_id: 4002, ..
        } => {}
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn humanoid_skill_list_succeeds_without_player_skill_unlock() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let current_ids = std::collections::BTreeSet::from([4002_u32, 100021_u32]);
    let abilities = super::load_validated_abilities(
        CharacterTarget::MainCharacter,
        AbilityListKind::Skills,
        &[4002, 100021],
        &current_ids,
        &lookup,
        Some(crate::domain::gamedata::GameId::Dao),
    )
    .unwrap();

    assert_eq!(
        abilities
            .iter()
            .map(|ability| ability.id)
            .collect::<Vec<_>>(),
        vec![4002, 100021]
    );
}

#[test]
fn rejects_dao_talent_id_when_editing_da2_talents() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let mut editor =
        SaveEditor::from_path_with_lookup(da2_save_path(), Some(&lookup), None).unwrap();

    let error = editor
        .replace_character_abilities(
            CharacterTarget::MainCharacter,
            AbilityListKind::Talents,
            &[23],
            &lookup,
        )
        .unwrap_err();

    match error {
        EditError::UnknownAbility { ability_id: 23 } => {}
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn replacing_da2_talents_preserves_existing_spells() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let mut editor =
        SaveEditor::from_path_with_lookup(da2_save_path(), Some(&lookup), None).unwrap();
    let original_spells = editor
        .save()
        .main_character
        .spells
        .iter()
        .map(|ability| ability.id)
        .collect::<Vec<_>>();
    let mut replacement = editor
        .save()
        .main_character
        .talents
        .iter()
        .map(|ability| ability.id)
        .filter(|id| lookup.ability(*id, Some(GameId::Da2)).unwrap().is_some())
        .collect::<Vec<_>>();
    replacement.reverse();

    editor
        .replace_character_abilities(
            CharacterTarget::MainCharacter,
            AbilityListKind::Talents,
            &replacement,
            &lookup,
        )
        .unwrap();

    assert_eq!(
        editor
            .save()
            .main_character
            .spells
            .iter()
            .map(|ability| ability.id)
            .collect::<Vec<_>>(),
        original_spells
    );
}

#[test]
fn patches_backpack_item_metadata() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let mut editor =
        SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
    editor
        .patch_item_metadata(
            InventoryContainer::Backpack,
            0,
            ItemMetadataPatch {
                item_cost: Some(1234),
                material: Some(5),
                item_level: Some(2),
            },
        )
        .unwrap();
    let item = &editor.save().backpack[0];
    assert_eq!(item.item_cost, Some(1234));
    assert_eq!(item.material, Some(5));
    assert_eq!(item.item_level, Some(2));
}

#[test]
fn removes_backpack_item() {
    let mut editor = SaveEditor::from_path(dao_save_path()).unwrap();
    let original_len = editor.save().backpack.len();
    editor.remove_backpack_item(0).unwrap();
    assert_eq!(editor.save().backpack.len(), original_len - 1);
}

#[test]
fn dao_stack_size_allows_stackable_item() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let mut editor =
        SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
    let index = first_stackable_backpack_item(&editor);

    editor.set_backpack_item_stack_size(index, 2).unwrap();

    assert_eq!(editor.save().backpack[index].item_stacksize, Some(2));
    assert_eq!(raw_backpack_stack_size(&editor, index), Some(2));
}

#[test]
fn dao_stack_size_rejects_non_stackable_item() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let mut editor =
        SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
    let index = first_non_stackable_backpack_item(&editor);

    let error = editor.set_backpack_item_stack_size(index, 2).unwrap_err();

    match error {
        EditError::ItemIsNotStackable { index: error_index } => assert_eq!(error_index, index),
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn dao_clones_non_stackable_backpack_item() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let mut editor =
        SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
    let index = first_non_stackable_backpack_item(&editor);
    let original_len = editor.save().backpack.len();
    let original_resref = editor.save().backpack[index].resref.clone();
    let original_object_id = editor.save().backpack[index].object_id;
    let next_object_id = super::next_object_id(editor.raw()).unwrap();

    let cloned_index = editor.clone_backpack_item(index).unwrap();

    assert_eq!(cloned_index, original_len);
    assert_eq!(editor.save().backpack.len(), original_len + 1);
    assert_eq!(editor.save().backpack[cloned_index].resref, original_resref);
    assert_eq!(
        editor.save().backpack[cloned_index].object_id,
        Some(next_object_id as i32)
    );
    assert_ne!(
        editor.save().backpack[cloned_index].object_id,
        original_object_id
    );
    assert_eq!(worlddb_last_id(&editor), Some(next_object_id));
}

#[test]
fn dao_clone_rejects_stackable_item() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let mut editor =
        SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
    let index = first_stackable_backpack_item(&editor);

    let error = editor.clone_backpack_item(index).unwrap_err();

    match error {
        EditError::ItemIsStackable { index: error_index } => assert_eq!(error_index, index),
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn da2_clones_non_stackable_backpack_item() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let mut editor =
        SaveEditor::from_path_with_lookup(da2_save_path(), Some(&lookup), None).unwrap();
    let index = first_non_stackable_backpack_item(&editor);
    let original_len = editor.save().backpack.len();
    let original_resref = editor.save().backpack[index].resref.clone();
    let original_object_id = editor.save().backpack[index].object_id;
    let next_object_id = super::next_object_id(editor.raw()).unwrap();

    let cloned_index = editor.clone_backpack_item(index).unwrap();

    assert_eq!(cloned_index, original_len);
    assert_eq!(editor.save().backpack.len(), original_len + 1);
    assert_eq!(editor.save().backpack[cloned_index].resref, original_resref);
    assert_eq!(
        editor.save().backpack[cloned_index].object_id,
        Some(next_object_id as i32)
    );
    assert_ne!(
        editor.save().backpack[cloned_index].object_id,
        original_object_id
    );
    assert_eq!(worlddb_last_id(&editor), Some(next_object_id));
}

#[test]
fn da2_clone_rejects_stackable_item() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let mut editor =
        SaveEditor::from_path_with_lookup(da2_save_path(), Some(&lookup), None).unwrap();
    let index = first_stackable_backpack_item(&editor);

    let error = editor.clone_backpack_item(index).unwrap_err();

    match error {
        EditError::ItemIsStackable { index: error_index } => assert_eq!(error_index, index),
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn da2_stack_size_allows_stackable_item() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let mut editor =
        SaveEditor::from_path_with_lookup(da2_save_path(), Some(&lookup), None).unwrap();
    let index = first_stackable_backpack_item(&editor);

    editor.set_backpack_item_stack_size(index, 42).unwrap();

    assert_eq!(editor.save().backpack[index].item_stacksize, Some(42));
    assert_eq!(raw_backpack_stack_size(&editor, index), Some(42));
}

#[test]
fn da2_stack_size_rejects_non_stackable_item() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let mut editor =
        SaveEditor::from_path_with_lookup(da2_save_path(), Some(&lookup), None).unwrap();
    let index = first_non_stackable_backpack_item(&editor);

    let error = editor.set_backpack_item_stack_size(index, 2).unwrap_err();

    match error {
        EditError::ItemIsNotStackable { index: error_index } => assert_eq!(error_index, index),
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn stack_size_rejects_zero() {
    let mut editor = SaveEditor::from_path(dao_save_path()).unwrap();

    let error = editor.set_backpack_item_stack_size(0, 0).unwrap_err();

    match error {
        EditError::InvalidStackSize { stack_size: 0 } => {}
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn stack_size_rejects_above_99() {
    let mut editor = SaveEditor::from_path(dao_save_path()).unwrap();

    let error = editor.set_backpack_item_stack_size(0, 100).unwrap_err();

    match error {
        EditError::InvalidStackSize { stack_size: 100 } => {}
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn sets_companion_approval_by_object_id() {
    let input = dao_save_path();
    let output = test_output_path("companion-approval-edit.das");
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let mut editor = SaveEditor::from_path_with_lookup(&input, Some(&lookup), None).unwrap();

    editor
        .set_character_approval(CharacterTarget::Companion(0), 12)
        .unwrap();
    editor.write_to_path(&output).unwrap();

    let reloaded = GffFile::from_path(&output).unwrap();
    let save = SaveGame::from_gff_with_lookup(&reloaded, Some(&lookup), None).unwrap();
    let leliana = save
        .companions
        .iter()
        .find(|character| character.template_resref.as_deref() == Some("gen00fl_leliana"))
        .unwrap();
    let dog = save
        .companions
        .iter()
        .find(|character| character.template_resref.as_deref() == Some("gen00fl_dog"))
        .unwrap();

    assert_eq!(leliana.approval, Some(12));
    assert_eq!(dog.approval, Some(100));
}

#[test]
fn inserts_missing_companion_point_pool_stat_row() {
    let input = dao_save_path();
    let output = test_output_path("companion-point-pool-edit.das");
    let mut editor = SaveEditor::from_path(&input).unwrap();
    remove_stat_row(&mut editor, CharacterTarget::Companion(0), 34);
    editor.save.companions[0].point_pools.attribute_points = None;

    editor
        .patch_character_point_pools(
            CharacterTarget::Companion(0),
            PointPoolsPatch {
                attribute_points: Some(9),
                ..Default::default()
            },
        )
        .unwrap();
    editor.write_to_path(&output).unwrap();

    let reloaded = GffFile::from_path(&output).unwrap();
    let save = SaveGame::from_gff(&reloaded).unwrap();
    assert_eq!(save.companions[0].point_pools.attribute_points, Some(9));
}

#[test]
fn allows_same_resref_backpack_replacement() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let mut editor =
        SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
    let resref = editor.save().backpack[0].resref.clone().unwrap();
    editor
        .replace_backpack_item(
            0,
            BackpackItemReplacement {
                resref,
                item_cost: Some(2222),
                material: Some(3),
                item_level: Some(4),
            },
        )
        .unwrap();
    assert_eq!(editor.save().backpack[0].item_cost, Some(2222));
}

#[test]
fn rejects_backpack_replacement_with_different_resref() {
    let mut editor = SaveEditor::from_path(dao_save_path()).unwrap();
    let error = editor
        .replace_backpack_item(
            0,
            BackpackItemReplacement {
                resref: "totally_different_item".to_string(),
                item_cost: Some(1),
                material: None,
                item_level: None,
            },
        )
        .unwrap_err();
    match error {
        EditError::BackpackResrefMismatch { .. } => {}
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn patches_equipped_item_metadata_in_place() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let mut editor =
        SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
    editor
        .patch_item_metadata(
            InventoryContainer::Equipment {
                target: CharacterTarget::MainCharacter,
            },
            0,
            ItemMetadataPatch {
                item_cost: Some(555),
                material: Some(7),
                item_level: Some(9),
            },
        )
        .unwrap();
    let item = &editor
        .equipment_items(CharacterTarget::MainCharacter)
        .unwrap()[0];
    assert_eq!(item.item_cost, Some(555));
}

#[test]
fn adds_item_property_to_backpack_item() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let mut editor =
        SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
    let index = first_backpack_item_with_properties(&editor).unwrap_or(0);
    let original_len = editor.save().backpack[index].properties.len();
    editor
        .add_item_property(
            InventoryContainer::Backpack,
            index,
            3011,
            12.5,
            Some(&lookup),
        )
        .unwrap();
    let properties = &editor.save().backpack[index].properties;
    assert_eq!(properties.len(), original_len + 1);
    assert_eq!(properties.last().unwrap().id, 3011);
    assert_eq!(properties.last().unwrap().power, 12.5);
}

#[test]
fn updates_item_property_power() {
    let mut editor = SaveEditor::from_path(dao_save_path()).unwrap();
    let index = first_backpack_item_with_properties(&editor).unwrap();
    editor
        .set_item_property_power(InventoryContainer::Backpack, index, 0, 33.0)
        .unwrap();
    assert_eq!(editor.save().backpack[index].properties[0].power, 33.0);
}

#[test]
fn removes_item_property() {
    let mut editor = SaveEditor::from_path(dao_save_path()).unwrap();
    let index = first_backpack_item_with_properties(&editor).unwrap();
    let original_len = editor.save().backpack[index].properties.len();
    editor
        .remove_item_property(InventoryContainer::Backpack, index, 0)
        .unwrap();
    assert_eq!(
        editor.save().backpack[index].properties.len(),
        original_len - 1
    );
}

#[test]
fn replaces_item_property_id_in_place() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let mut editor =
        SaveEditor::from_path_with_lookup(dao_save_path(), Some(&lookup), None).unwrap();
    let index = first_backpack_item_with_properties(&editor).unwrap();
    editor
        .set_item_property_id(InventoryContainer::Backpack, index, 0, 3011, Some(&lookup))
        .unwrap();
    assert_eq!(editor.save().backpack[index].properties[0].id, 3011);
}

#[test]
fn rejects_property_edits_when_raw_arrays_are_mismatched() {
    let mut editor = SaveEditor::from_path(dao_save_path()).unwrap();
    let index = first_backpack_item_with_properties(&editor).unwrap();
    corrupt_first_backpack_property_power_list(&mut editor);

    let error = editor
        .set_item_property_power(InventoryContainer::Backpack, index, 0, 10.0)
        .unwrap_err();

    match error {
        EditError::InvalidPropertyArrayParity { .. } => {}
        other => panic!("unexpected error: {other}"),
    }
}

#[test]
fn write_reload_main_character_level_edit() {
    let input = dao_save_path();
    let output = test_output_path("main-level-edit.das");
    let mut editor = SaveEditor::from_path(&input).unwrap();
    editor
        .set_character_level(CharacterTarget::MainCharacter, 31)
        .unwrap();
    editor.write_to_path(&output).unwrap();
    let reloaded = GffFile::from_path(&output).unwrap();
    let save = SaveGame::from_gff(&reloaded).unwrap();
    assert_eq!(save.main_character.level, Some(31));
}

#[test]
fn write_reload_main_character_experience_edit() {
    let input = dao_save_path();
    let output = test_output_path("main-experience-edit.das");
    let mut editor = SaveEditor::from_path(&input).unwrap();
    editor
        .set_character_experience(CharacterTarget::MainCharacter, 123456)
        .unwrap();
    editor.write_to_path(&output).unwrap();
    let reloaded = GffFile::from_path(&output).unwrap();
    let save = SaveGame::from_gff(&reloaded).unwrap();
    assert_eq!(save.main_character.experience, Some(123456));
}

#[test]
fn write_reload_da2_main_character_level_and_experience_edit() {
    let input = da2_save_path();
    let output = test_output_path("da2-main-progress-edit.das");
    let mut editor = SaveEditor::from_path(&input).unwrap();
    editor
        .set_character_level(CharacterTarget::MainCharacter, 13)
        .unwrap();
    editor
        .set_character_experience(CharacterTarget::MainCharacter, 76543)
        .unwrap();
    editor.write_to_path(&output).unwrap();
    let reloaded = GffFile::from_path(&output).unwrap();
    let save = SaveGame::from_gff(&reloaded).unwrap();
    assert_eq!(save.main_character.level, Some(13));
    assert_eq!(save.main_character.experience, Some(76543));
}

#[test]
fn write_reload_da2_plot_flag_edit() {
    let input = da2_save_path();
    let output = test_output_path("da2-plot-flag-edit.das");
    let mut editor = SaveEditor::from_path(&input).unwrap();

    editor
        .patch_plot_flags(
            &[
                PlotBooleanPatch {
                    id: 2000,
                    value: true,
                },
                PlotBooleanPatch {
                    id: 2005,
                    value: false,
                },
            ],
            &[
                PlotIntegerPatch { id: 1000, value: 2 },
                PlotIntegerPatch { id: 1001, value: 3 },
            ],
        )
        .unwrap();
    editor.write_to_path(&output).unwrap();

    let reloaded = GffFile::from_path(&output).unwrap();
    let save = SaveGame::from_gff(&reloaded).unwrap();
    assert_eq!(save.plot_flags.booleans.get(&2000), Some(&true));
    assert_eq!(save.plot_flags.booleans.get(&2005), Some(&false));
    assert_eq!(save.plot_flags.integers.get(&1000), Some(&2));
    assert_eq!(save.plot_flags.integers.get(&1001), Some(&3));
}

#[test]
fn write_reload_ability_edit() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let input = dao_save_path();
    let output = test_output_path("ability-edit.das");
    let mut editor = SaveEditor::from_path_with_lookup(&input, Some(&lookup), None).unwrap();
    let original = editor
        .save()
        .main_character
        .skills
        .iter()
        .map(|ability| ability.id)
        .collect::<Vec<_>>();
    let replacement = vec![original[1], original[0]];
    editor
        .replace_character_abilities(
            CharacterTarget::MainCharacter,
            AbilityListKind::Skills,
            &replacement,
            &lookup,
        )
        .unwrap();
    editor.write_to_path(&output).unwrap();
    let reloaded = GffFile::from_path(&output).unwrap();
    let save = SaveGame::from_gff_with_lookup(&reloaded, Some(&lookup), None).unwrap();
    assert_eq!(
        save.main_character
            .skills
            .iter()
            .map(|ability| ability.id)
            .collect::<Vec<_>>(),
        replacement
    );
}

#[test]
fn write_reload_backpack_metadata_edit() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let input = dao_save_path();
    let output = test_output_path("backpack-metadata-edit.das");
    let mut editor = SaveEditor::from_path_with_lookup(&input, Some(&lookup), None).unwrap();
    editor
        .patch_item_metadata(
            InventoryContainer::Backpack,
            0,
            ItemMetadataPatch {
                item_cost: Some(6060),
                material: Some(4),
                item_level: Some(2),
            },
        )
        .unwrap();
    editor.write_to_path(&output).unwrap();
    let reloaded = GffFile::from_path(&output).unwrap();
    let save = SaveGame::from_gff_with_lookup(&reloaded, Some(&lookup), None).unwrap();
    assert_eq!(save.backpack[0].item_cost, Some(6060));
}

#[test]
fn write_reload_dao_stack_size_edit() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let input = dao_save_path();
    let output = test_output_path("dao-backpack-stack-edit.das");
    let mut editor = SaveEditor::from_path_with_lookup(&input, Some(&lookup), None).unwrap();
    let index = first_stackable_backpack_item(&editor);

    editor.set_backpack_item_stack_size(index, 2).unwrap();
    editor.write_to_path(&output).unwrap();

    let reloaded = GffFile::from_path(&output).unwrap();
    let save = SaveGame::from_gff_with_lookup(&reloaded, Some(&lookup), None).unwrap();
    assert_eq!(save.backpack[index].item_stacksize, Some(2));
}

#[test]
fn write_reload_dao_cloned_backpack_item() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let input = dao_save_path();
    let output = test_output_path("dao-backpack-clone.das");
    let mut editor = SaveEditor::from_path_with_lookup(&input, Some(&lookup), None).unwrap();
    let index = first_non_stackable_backpack_item(&editor);
    let original_len = editor.save().backpack.len();
    let original_resref = editor.save().backpack[index].resref.clone();
    let original_object_id = editor.save().backpack[index].object_id;
    let cloned_index = editor.clone_backpack_item(index).unwrap();

    editor.write_to_path(&output).unwrap();

    let reloaded = GffFile::from_path(&output).unwrap();
    let save = SaveGame::from_gff_with_lookup(&reloaded, Some(&lookup), None).unwrap();
    assert_eq!(save.backpack.len(), original_len + 1);
    assert_eq!(save.backpack[cloned_index].resref, original_resref);
    assert_ne!(save.backpack[cloned_index].object_id, original_object_id);
}

#[test]
fn write_reload_da2_cloned_backpack_item() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let input = da2_save_path();
    let output = test_output_path("da2-backpack-clone.das");
    let mut editor = SaveEditor::from_path_with_lookup(&input, Some(&lookup), None).unwrap();
    let index = first_non_stackable_backpack_item(&editor);
    let original_len = editor.save().backpack.len();
    let original_resref = editor.save().backpack[index].resref.clone();
    let original_object_id = editor.save().backpack[index].object_id;
    let cloned_index = editor.clone_backpack_item(index).unwrap();

    editor.write_to_path(&output).unwrap();

    let reloaded = GffFile::from_path(&output).unwrap();
    let save = SaveGame::from_gff_with_lookup(&reloaded, Some(&lookup), None).unwrap();
    assert_eq!(save.backpack.len(), original_len + 1);
    assert_eq!(save.backpack[cloned_index].resref, original_resref);
    assert_ne!(save.backpack[cloned_index].object_id, original_object_id);
}

#[test]
fn write_reload_da2_stack_size_edit() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let input = da2_save_path();
    let output = test_output_path("da2-backpack-stack-edit.das");
    let mut editor = SaveEditor::from_path_with_lookup(&input, Some(&lookup), None).unwrap();
    let index = first_stackable_backpack_item(&editor);

    editor.set_backpack_item_stack_size(index, 42).unwrap();
    editor.write_to_path(&output).unwrap();

    let reloaded = GffFile::from_path(&output).unwrap();
    let save = SaveGame::from_gff_with_lookup(&reloaded, Some(&lookup), None).unwrap();
    assert_eq!(save.backpack[index].item_stacksize, Some(42));
}

#[test]
fn write_reload_crafting_recipe_edit() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let input = dao_save_path();
    let output = test_output_path("crafting-recipe-edit.das");
    let mut editor = SaveEditor::from_path_with_lookup(&input, Some(&lookup), None).unwrap();

    editor.replace_crafting_recipes(&[2, 11, 20019]).unwrap();
    editor.write_to_path(&output).unwrap();

    let reloaded = GffFile::from_path(&output).unwrap();
    let save = SaveGame::from_gff_with_lookup(&reloaded, Some(&lookup), None).unwrap();
    assert_eq!(save.crafting_recipes, vec![2, 11, 20019]);
}

#[test]
fn write_reload_item_property_edit() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let input = dao_save_path();
    let output = test_output_path("item-property-edit.das");
    let mut editor = SaveEditor::from_path_with_lookup(&input, Some(&lookup), None).unwrap();
    let index = first_backpack_item_with_properties(&editor).unwrap();
    editor
        .set_item_property_power(InventoryContainer::Backpack, index, 0, 21.0)
        .unwrap();
    editor
        .add_item_property(
            InventoryContainer::Backpack,
            index,
            3011,
            9.0,
            Some(&lookup),
        )
        .unwrap();
    editor.write_to_path(&output).unwrap();
    let reloaded = GffFile::from_path(&output).unwrap();
    let save = SaveGame::from_gff_with_lookup(&reloaded, Some(&lookup), None).unwrap();
    assert_eq!(save.backpack[index].properties[0].power, 21.0);
    assert_eq!(save.backpack[index].properties.last().unwrap().id, 3011);
}

#[test]
fn write_reload_item_property_id_edit() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let input = dao_save_path();
    let output = test_output_path("item-property-id-edit.das");
    let mut editor = SaveEditor::from_path_with_lookup(&input, Some(&lookup), None).unwrap();
    let index = first_backpack_item_with_properties(&editor).unwrap();
    editor
        .set_item_property_id(InventoryContainer::Backpack, index, 0, 3011, Some(&lookup))
        .unwrap();
    editor.write_to_path(&output).unwrap();
    let reloaded = GffFile::from_path(&output).unwrap();
    let save = SaveGame::from_gff_with_lookup(&reloaded, Some(&lookup), None).unwrap();
    assert_eq!(save.backpack[index].properties[0].id, 3011);
}

#[test]
fn da2_added_item_property_uses_float_property_id_storage() {
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let mut editor =
        SaveEditor::from_path_with_lookup(da2_save_path(), Some(&lookup), None).unwrap();
    let index = first_backpack_item_with_properties(&editor).unwrap_or(0);
    clear_backpack_item_properties(&mut editor, index);

    editor
        .add_item_property(
            InventoryContainer::Backpack,
            index,
            1000,
            1.0,
            Some(&lookup),
        )
        .unwrap();

    let raw_item =
        super::raw_item_mut(&mut editor.raw, InventoryContainer::Backpack, index).unwrap();
    let property_ids = raw_item
        .get_list_by_name(super::ITEM_PROPERTIES_NAME)
        .unwrap();
    assert_eq!(property_ids.first(), Some(&super::Value::Float32(1000.0)));
    let property_powers = raw_item
        .get_list_by_name(super::ITEM_PROPERTY_POWERS_NAME)
        .unwrap();
    assert_eq!(
        property_powers.first(),
        Some(&super::Value::UInt32(1.0f32.to_bits()))
    );
}

fn remove_stat_row(editor: &mut SaveEditor, target: CharacterTarget, stat_id: u32) {
    let character = super::raw_character_mut(&mut editor.raw, target).unwrap();
    let stats = character
        .get_struct_mut_by_name(super::SAVEGAME_CREATURE_STATS_NAME)
        .unwrap();
    let stat_list = stats
        .get_list_mut_by_name(super::SAVEGAME_STATLIST_NAME)
        .unwrap();
    stat_list.retain(|value| {
        value
            .as_struct()
            .and_then(|row| row.get_by_name(super::SAVEGAME_STATPROPERTY_INDEX_NAME))
            .and_then(super::value_to_u32)
            != Some(stat_id)
    });
}

fn test_output_path(name: &str) -> PathBuf {
    let dir = PathBuf::from("target").join("test-output");
    fs::create_dir_all(&dir).unwrap();
    dir.join(name)
}

fn first_backpack_item_with_properties(editor: &SaveEditor) -> Option<usize> {
    editor
        .save()
        .backpack
        .iter()
        .position(|item| !item.properties.is_empty())
}

fn first_stackable_backpack_item(editor: &SaveEditor) -> usize {
    editor
        .save()
        .backpack
        .iter()
        .position(|item| item.stackable)
        .expect("expected stackable backpack item")
}

fn first_non_stackable_backpack_item(editor: &SaveEditor) -> usize {
    editor
        .save()
        .backpack
        .iter()
        .position(|item| !item.stackable)
        .expect("expected non-stackable backpack item")
}

fn raw_backpack_stack_size(editor: &SaveEditor, index: usize) -> Option<u32> {
    super::raw_item(editor.raw(), InventoryContainer::Backpack, index)
        .unwrap()
        .get(super::ITEM_STACKSIZE)
        .and_then(super::value_to_u32)
}

fn worlddb_last_id(editor: &SaveEditor) -> Option<u32> {
    editor
        .raw()
        .root()
        .get(super::SAVEGAME_WORLDDATABASE)
        .and_then(|value| super::find_field_value(value, super::SAVEGAME_WORLDDB_LASTID))
        .and_then(super::value_to_u32)
}

fn corrupt_first_backpack_property_power_list(editor: &mut SaveEditor) {
    let party = editor
        .raw
        .root_mut()
        .get_struct_mut(SAVEGAME_PARTYLIST)
        .unwrap();
    let items = party.get_list_mut(super::SAVEGAME_BACKPACK).unwrap();
    for value in items {
        let Some(item) = value.as_struct_mut() else {
            continue;
        };
        let Some(powers) = item.get_list_mut_by_name(super::ITEM_PROPERTY_POWERS_NAME) else {
            continue;
        };
        if !powers.is_empty() {
            powers.pop();
            return;
        }
    }
    panic!("expected backpack item with property powers");
}

fn clear_backpack_item_properties(editor: &mut SaveEditor, index: usize) {
    editor.save.backpack[index].properties.clear();
    let raw_item =
        super::raw_item_mut(&mut editor.raw, InventoryContainer::Backpack, index).unwrap();
    raw_item
        .get_list_mut_by_name(super::ITEM_PROPERTIES_NAME)
        .unwrap()
        .clear();
    raw_item
        .get_list_mut_by_name(super::ITEM_PROPERTY_POWERS_NAME)
        .unwrap()
        .clear();
}
