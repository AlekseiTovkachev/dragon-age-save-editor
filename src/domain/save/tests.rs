use super::SaveGame;
use crate::domain::gamedata::{DEFAULT_GAME_DATA_PATH, GameId, SqliteGameData};
use crate::gff4::GffFile;
use crate::test_support::{da2_save_path, dao_save_path, flat_sample_save_path};

#[test]
fn extracts_dao_read_only_summary() {
    let gff = GffFile::from_path(dao_save_path()).unwrap();
    let save = SaveGame::from_gff(&gff).unwrap();

    assert!(!save.main_character.name.is_empty());
    assert!(!save.companions.is_empty());
    assert!(save.main_character.core_stats.strength > 0);
    assert!(save.main_character.level.is_some());
    assert!(save.main_character.experience.is_some());
    assert!(save.companions[0].core_stats.magic > 0);
}

#[test]
fn extracts_da2_read_only_summary() {
    let gff = GffFile::from_path(da2_save_path()).unwrap();
    let save = SaveGame::from_gff(&gff).unwrap();

    assert!(!save.main_character.name.is_empty());
    assert!(save.money <= u32::MAX);
    assert!(!save.companions.is_empty());
    assert!(save.main_character.core_stats.strength > 0);
    assert!(save.main_character.core_stats.dexterity > 0);
    assert_eq!(save.main_character.level, Some(1));
    assert_eq!(save.main_character.experience, Some(700));
}

#[test]
fn extracts_da2_world_vault_ints_and_booleans_from_expected_lists() {
    let gff = GffFile::from_path(da2_save_path()).unwrap();
    let save = SaveGame::from_gff(&gff).unwrap();

    assert_eq!(save.plot_flags.integers.get(&1000), Some(&1));
    assert!(
        save.plot_flags
            .integers
            .get(&1001)
            .is_some_and(|value| (1..=3).contains(value))
    );
    assert!(save.plot_flags.booleans.contains_key(&2000));
    assert!(save.plot_flags.booleans.contains_key(&2108));
    assert!(!save.plot_flags.integers.contains_key(&2000));
    assert!(!save.plot_flags.booleans.contains_key(&1000));
}

#[test]
fn enriches_domain_with_db_lookups() {
    let gff = GffFile::from_path(dao_save_path()).unwrap();
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let save = SaveGame::from_gff_with_lookup(&gff, Some(&lookup), None).unwrap();

    assert!(!save.main_character.skills.is_empty());
    assert!(
        save.main_character
            .skills
            .iter()
            .any(|ability| ability.name.is_some())
    );
    assert_eq!(save.preferred_game, Some(GameId::Dao));
    assert!(
        save.companions
            .iter()
            .all(|character| !character.name.trim().is_empty())
    );
    assert!(save.backpack.iter().any(|item| item.name.is_some()));
    assert!(save.backpack.iter().any(|item| {
        item.properties
            .iter()
            .any(|property| property.name.is_some())
    }));
}

#[test]
fn enriches_da2_names_with_inferred_game() {
    let gff = GffFile::from_path(da2_save_path()).unwrap();
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let save = SaveGame::from_gff_with_lookup(&gff, Some(&lookup), None).unwrap();

    assert_eq!(save.preferred_game, Some(GameId::Da2));
    assert!(
        save.companions
            .iter()
            .all(|character| !character.name.trim().is_empty())
    );
    assert!(save.backpack.iter().any(|item| item.name.is_some()));
}

#[test]
fn infers_dao_awakening_content_from_campaign_resource() {
    for name in [
        "testingawakening.das",
        "testingwitchhunt.das",
        "testinggolems.das",
    ] {
        let Some(path) = flat_sample_save_path(name) else {
            continue;
        };
        let gff = GffFile::from_path(path).unwrap();
        let save = SaveGame::from_gff(&gff).unwrap();

        assert_eq!(save.preferred_game, Some(GameId::DaoAwakening), "{name}");
    }
}

#[test]
fn infers_vanilla_dao_for_base_campaign_resource() {
    let gff = GffFile::from_path(dao_save_path()).unwrap();
    let save = SaveGame::from_gff(&gff).unwrap();

    assert_eq!(save.preferred_game, Some(GameId::Dao));
}

#[test]
fn loads_existing_da2_abilities_from_combined_ability_list() {
    let gff = GffFile::from_path(da2_save_path()).unwrap();
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let save = SaveGame::from_gff_with_lookup(&gff, Some(&lookup), None).unwrap();

    assert!(!save.main_character.talents.is_empty());
    assert!(
        save.main_character
            .talents
            .iter()
            .any(|ability| ability.name.is_some())
    );
    assert!(
        save.main_character
            .talents
            .iter()
            .any(|ability| ability.id == 200000 || ability.id == 201000 || ability.id == 201001)
    );
    assert!(
        save.main_character
            .talents
            .iter()
            .chain(save.main_character.spells.iter())
            .all(|ability| ability.id != 0)
    );
}

#[test]
fn decodes_da2_integer_backed_item_property_powers_as_float_bits() {
    let gff = GffFile::from_path(da2_save_path()).unwrap();
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let save = SaveGame::from_gff_with_lookup(&gff, Some(&lookup), None).unwrap();

    assert!(
        save.backpack
            .iter()
            .flat_map(|item| item.properties.iter())
            .any(|property| (property.power - 1.0).abs() < f32::EPSILON)
    );
    assert!(
        !save
            .backpack
            .iter()
            .flat_map(|item| item.properties.iter())
            .any(|property| property.power > 100_000.0)
    );
}

#[test]
fn maps_dao_companion_approval_by_object_id() {
    let gff = GffFile::from_path(dao_save_path()).unwrap();
    let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
    let save = SaveGame::from_gff_with_lookup(&gff, Some(&lookup), None).unwrap();

    let approvals = save
        .companions
        .iter()
        .map(|character| (character.template_resref.as_deref(), character.approval))
        .collect::<Vec<_>>();

    assert!(approvals.contains(&(Some("gen00fl_leliana"), Some(95))));
    assert!(approvals.contains(&(Some("gen00fl_wynne"), Some(98))));
    assert!(approvals.contains(&(Some("gen00fl_alistair"), Some(74))));
    assert!(approvals.contains(&(Some("gen00fl_dog"), Some(100))));
}
