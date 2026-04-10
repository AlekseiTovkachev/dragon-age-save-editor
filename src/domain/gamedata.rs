use crate::domain::ability::{AbilityKind, AbilityRef};
use crate::domain::item::{MaterialFamily, MaterialInfo, MaterialProfile, MaterialTarget};
use rusqlite::{params, Connection, OptionalExtension};
use std::error::Error;
use std::fmt;
use std::path::Path;

pub const DEFAULT_GAME_DATA_PATH: &str = "data/gamedata.db";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameId {
    Dao,
    Da2,
}

impl GameId {
    fn as_db_value(self) -> &'static str {
        match self {
            GameId::Dao => "dao",
            GameId::Da2 => "da2",
        }
    }
}

pub trait GameDataLookup {
    fn item_name(
        &self,
        resref: &str,
        preferred_game: Option<GameId>,
    ) -> Result<Option<String>, LookupError>;

    fn ability(
        &self,
        ability_id: u32,
        preferred_game: Option<GameId>,
    ) -> Result<Option<AbilityRef>, LookupError>;

    fn abilities_by_kind(
        &self,
        kind: AbilityKind,
        preferred_game: Option<GameId>,
    ) -> Result<Vec<AbilityRef>, LookupError>;

    fn item_properties(&self) -> Result<Vec<(u32, Option<String>)>, LookupError>;

    fn item_property_name(&self, property_id: u32) -> Result<Option<String>, LookupError>;

    fn material_info(
        &self,
        material_code: u32,
        preferred_game: Option<GameId>,
    ) -> Result<Option<MaterialInfo>, LookupError>;

    fn item_material_profile(
        &self,
        resref: &str,
        preferred_game: Option<GameId>,
    ) -> Result<Option<MaterialProfile>, LookupError>;

    fn material_options(
        &self,
        family: MaterialFamily,
        target: MaterialTarget,
        preferred_game: Option<GameId>,
    ) -> Result<Vec<MaterialInfo>, LookupError>;
}

#[derive(Debug)]
pub struct SqliteGameData {
    connection: Connection,
}

#[derive(Debug)]
pub enum LookupError {
    Sqlite(rusqlite::Error),
}

impl fmt::Display for LookupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LookupError::Sqlite(err) => write!(f, "{err}"),
        }
    }
}

impl Error for LookupError {}

impl From<rusqlite::Error> for LookupError {
    fn from(value: rusqlite::Error) -> Self {
        LookupError::Sqlite(value)
    }
}

impl SqliteGameData {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, LookupError> {
        let connection = Connection::open(path)?;
        Ok(Self { connection })
    }

    fn map_ability_row(
        ability_id: u32,
        row: (String, Option<String>, Option<String>, Option<String>, Option<String>),
    ) -> AbilityRef {
        let (id_text, name, core_id, tree, ability_type) = row;
        let parsed_id = id_text.parse().unwrap_or(ability_id);
        let core_ids = core_id
            .unwrap_or_default()
            .split(',')
            .filter_map(|part| {
                let trimmed = part.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    trimmed.parse::<u32>().ok()
                }
            })
            .collect();

        let ability_type = ability_type.map(|value| value.trim().to_string());
        AbilityRef {
            id: parsed_id,
            name,
            tree,
            kind: AbilityKind::from_db_type(ability_type.as_deref()),
            ability_type,
            core_ids,
        }
    }

    fn ability_row(
        &self,
        ability_id: u32,
        preferred_game: Option<GameId>,
    ) -> Result<Option<(String, Option<String>, Option<String>, Option<String>, Option<String>)>, LookupError> {
        let id = ability_id.to_string();

        if let Some(game) = preferred_game {
            let row = self
                .connection
                .query_row(
                    "SELECT id, name, core_id, tree, type FROM abilities WHERE id = ?1 AND game = ?2",
                    params![id, game.as_db_value()],
                    |row| {
                        Ok((
                            row.get::<_, String>(0)?,
                            row.get::<_, Option<String>>(1)?,
                            row.get::<_, Option<String>>(2)?,
                            row.get::<_, Option<String>>(3)?,
                            row.get::<_, Option<String>>(4)?,
                        ))
                    },
                )
                .optional()?;
            if row.is_some() {
                return Ok(row);
            }
        }

        Ok(None)
    }

    fn family_from_db(value: &str) -> Option<MaterialFamily> {
        match value {
            "metal" => Some(MaterialFamily::Metal),
            "wood" => Some(MaterialFamily::Wood),
            "leather" => Some(MaterialFamily::Leather),
            _ => None,
        }
    }

    fn target_from_db(value: &str) -> Option<MaterialTarget> {
        match value {
            "armor" => Some(MaterialTarget::Armor),
            "weapon" => Some(MaterialTarget::Weapon),
            "shield" => Some(MaterialTarget::Shield),
            _ => None,
        }
    }

    fn family_to_db(value: MaterialFamily) -> &'static str {
        match value {
            MaterialFamily::Metal => "metal",
            MaterialFamily::Wood => "wood",
            MaterialFamily::Leather => "leather",
        }
    }

    fn target_to_db(value: MaterialTarget) -> &'static str {
        match value {
            MaterialTarget::Armor => "armor",
            MaterialTarget::Weapon => "weapon",
            MaterialTarget::Shield => "shield",
        }
    }
}

impl GameDataLookup for SqliteGameData {
    fn item_name(
        &self,
        resref: &str,
        preferred_game: Option<GameId>,
    ) -> Result<Option<String>, LookupError> {
        let cleaned = resref.trim_end_matches('\0').to_ascii_lowercase();

        if let Some(game) = preferred_game {
            let result = self
                .connection
                .query_row(
                    "SELECT name FROM items WHERE resref = ?1 AND game = ?2",
                    params![cleaned, game.as_db_value()],
                    |row| row.get::<_, Option<String>>(0),
                )
                .optional()?
                .flatten();

            if result.is_some() {
                return Ok(result);
            }
        }

        let result = self
            .connection
            .query_row(
                "SELECT name FROM items WHERE resref = ?1 ORDER BY CASE game WHEN 'dao' THEN 0 WHEN 'da2' THEN 1 ELSE 2 END LIMIT 1",
                params![cleaned],
                |row| row.get::<_, Option<String>>(0),
            )
            .optional()?
            .flatten();

        Ok(result)
    }

    fn ability(
        &self,
        ability_id: u32,
        preferred_game: Option<GameId>,
    ) -> Result<Option<AbilityRef>, LookupError> {
        let Some((id_text, name, core_id, tree, ability_type)) =
            self.ability_row(ability_id, preferred_game)?
        else {
            return Ok(None);
        };
        Ok(Some(Self::map_ability_row(
            ability_id,
            (id_text, name, core_id, tree, ability_type),
        )))
    }

    fn abilities_by_kind(
        &self,
        kind: AbilityKind,
        preferred_game: Option<GameId>,
    ) -> Result<Vec<AbilityRef>, LookupError> {
        let Some(game) = preferred_game else {
            return Ok(Vec::new());
        };

        let mut statement = self.connection.prepare(
            "SELECT id, name, core_id, tree, type FROM abilities WHERE game = ?1 ORDER BY name, CAST(id AS INTEGER)",
        )?;
        let rows = statement.query_map(params![game.as_db_value()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Option<String>>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, Option<String>>(4)?,
            ))
        })?;

        let mut abilities = Vec::new();
        for row in rows {
            let row = row?;
            let ability = Self::map_ability_row(
                row.0.parse().unwrap_or_default(),
                row,
            );
            if ability.kind == kind {
                abilities.push(ability);
            }
        }
        Ok(abilities)
    }

    fn item_properties(&self) -> Result<Vec<(u32, Option<String>)>, LookupError> {
        let mut statement = self
            .connection
            .prepare("SELECT id, label FROM item_properties ORDER BY label, id")?;
        let rows = statement.query_map([], |row| {
            Ok((row.get::<_, u32>(0)?, row.get::<_, Option<String>>(1)?))
        })?;

        let mut properties = Vec::new();
        for row in rows {
            properties.push(row?);
        }
        Ok(properties)
    }

    fn item_property_name(&self, property_id: u32) -> Result<Option<String>, LookupError> {
        let result = self
            .connection
            .query_row(
                "SELECT label FROM item_properties WHERE id = ?1",
                params![property_id],
                |row| row.get::<_, Option<String>>(0),
            )
            .optional()?
            .flatten();

        Ok(result)
    }

    fn material_info(
        &self,
        material_code: u32,
        preferred_game: Option<GameId>,
    ) -> Result<Option<MaterialInfo>, LookupError> {
        if preferred_game != Some(GameId::Dao) {
            return Ok(None);
        }

        self.connection
            .query_row(
                "SELECT code, tier, name, family, target
                 FROM material_codes
                 WHERE game = 'dao' AND code = ?1",
                params![material_code],
                |row| {
                    let family = row.get::<_, String>(3)?;
                    let target = row.get::<_, String>(4)?;
                    Ok(MaterialInfo {
                        code: row.get::<_, u32>(0)?,
                        tier: row.get::<_, u8>(1)?,
                        name: row.get::<_, String>(2)?,
                        family: Self::family_from_db(&family).ok_or_else(|| {
                            rusqlite::Error::FromSqlConversionFailure(
                                3,
                                rusqlite::types::Type::Text,
                                Box::<dyn std::error::Error + Send + Sync>::from("invalid family"),
                            )
                        })?,
                        target: Self::target_from_db(&target).ok_or_else(|| {
                            rusqlite::Error::FromSqlConversionFailure(
                                4,
                                rusqlite::types::Type::Text,
                                Box::<dyn std::error::Error + Send + Sync>::from("invalid target"),
                            )
                        })?,
                    })
                },
            )
            .optional()
            .map_err(LookupError::from)
    }

    fn item_material_profile(
        &self,
        resref: &str,
        preferred_game: Option<GameId>,
    ) -> Result<Option<MaterialProfile>, LookupError> {
        let Some(game) = preferred_game else {
            return Ok(None);
        };
        let cleaned = resref.trim_end_matches('\0').to_ascii_lowercase();
        self.connection
            .query_row(
                "SELECT material_family, material_target FROM items WHERE resref = ?1 AND game = ?2",
                params![cleaned, game.as_db_value()],
                |row| {
                    let family = row.get::<_, Option<String>>(0)?;
                    let target = row.get::<_, Option<String>>(1)?;
                    Ok(match (family.as_deref(), target.as_deref()) {
                        (Some(family), Some(target)) => Some(MaterialProfile {
                            family: Self::family_from_db(family).ok_or_else(|| {
                                rusqlite::Error::FromSqlConversionFailure(
                                    0,
                                    rusqlite::types::Type::Text,
                                    Box::<dyn std::error::Error + Send + Sync>::from("invalid family"),
                                )
                            })?,
                            target: Self::target_from_db(target).ok_or_else(|| {
                                rusqlite::Error::FromSqlConversionFailure(
                                    1,
                                    rusqlite::types::Type::Text,
                                    Box::<dyn std::error::Error + Send + Sync>::from("invalid target"),
                                )
                            })?,
                        }),
                        _ => None,
                    })
                },
            )
            .optional()
            .map(|result| result.flatten())
            .map_err(LookupError::from)
    }

    fn material_options(
        &self,
        family: MaterialFamily,
        target: MaterialTarget,
        preferred_game: Option<GameId>,
    ) -> Result<Vec<MaterialInfo>, LookupError> {
        if preferred_game != Some(GameId::Dao) {
            return Ok(Vec::new());
        }

        let mut statement = self.connection.prepare(
            "SELECT code, tier, name, family, target
             FROM material_codes
             WHERE game = 'dao' AND family = ?1 AND target = ?2
             ORDER BY tier, code",
        )?;
        let rows = statement.query_map(
            params![Self::family_to_db(family), Self::target_to_db(target)],
            |row| {
                let family = row.get::<_, String>(3)?;
                let target = row.get::<_, String>(4)?;
                Ok(MaterialInfo {
                    code: row.get::<_, u32>(0)?,
                    tier: row.get::<_, u8>(1)?,
                    name: row.get::<_, String>(2)?,
                    family: Self::family_from_db(&family).ok_or_else(|| {
                        rusqlite::Error::FromSqlConversionFailure(
                            3,
                            rusqlite::types::Type::Text,
                            Box::<dyn std::error::Error + Send + Sync>::from("invalid family"),
                        )
                    })?,
                    target: Self::target_from_db(&target).ok_or_else(|| {
                        rusqlite::Error::FromSqlConversionFailure(
                            4,
                            rusqlite::types::Type::Text,
                            Box::<dyn std::error::Error + Send + Sync>::from("invalid target"),
                        )
                    })?,
                })
            },
        )?;

        let mut materials = Vec::new();
        for row in rows {
            materials.push(row?);
        }
        Ok(materials)
    }
}

#[cfg(test)]
mod tests {
    use super::{AbilityKind, GameDataLookup, GameId, SqliteGameData, DEFAULT_GAME_DATA_PATH};
    use crate::domain::item::{MaterialFamily, MaterialTarget};
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_db_path(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("dragon_age_save_editor_{name}_{nonce}.db"))
    }

    fn create_test_lookup() -> SqliteGameData {
        let path = temp_db_path("abilities");
        let connection = rusqlite::Connection::open(&path).unwrap();
        connection
            .execute_batch(
                "
                CREATE TABLE abilities (
                    id TEXT NOT NULL,
                    name TEXT,
                    core_id TEXT,
                    tree TEXT,
                    type TEXT,
                    game TEXT NOT NULL,
                    PRIMARY KEY (id, game)
                );
                CREATE TABLE material_codes (
                    code INTEGER NOT NULL,
                    tier INTEGER NOT NULL,
                    name TEXT NOT NULL,
                    family TEXT NOT NULL,
                    target TEXT NOT NULL,
                    game TEXT NOT NULL,
                    PRIMARY KEY (code, game)
                );
                CREATE TABLE items (
                    resref TEXT NOT NULL,
                    name TEXT,
                    game TEXT NOT NULL,
                    material_family TEXT,
                    material_target TEXT,
                    PRIMARY KEY (resref, game)
                );
                INSERT INTO abilities (id, name, core_id, tree, type, game) VALUES
                    ('5000', 'DAO Test Talent', '5001', 'DAO Tree', ' Talent ', 'dao'),
                    ('5000', 'DA2 Test Talent', '5002', 'DA2 Tree', 'Specialization', 'da2'),
                    ('5001', 'DAO Skill', NULL, 'DAO Skills', 'Skill', 'dao'),
                    ('5002', 'DA2 Spell', NULL, 'DA2 Spells', 'Spell', 'da2');
                INSERT INTO material_codes (code, tier, name, family, target, game) VALUES
                    (45, 6, 'Silverite', 'metal', 'weapon', 'dao');
                INSERT INTO items (resref, name, game, material_family, material_target) VALUES
                    ('gen_im_wep_mel_lsw_lsw', 'Longsword', 'dao', 'metal', 'weapon');
                ",
            )
            .unwrap();
        drop(connection);

        SqliteGameData::open(&path).unwrap()
    }

    #[test]
    fn ability_lookup_returns_dao_row_when_requested() {
        let lookup = create_test_lookup();
        let ability = lookup.ability(5000, Some(GameId::Dao)).unwrap().unwrap();
        assert_eq!(ability.name.as_deref(), Some("DAO Test Talent"));
        assert_eq!(ability.tree.as_deref(), Some("DAO Tree"));
        assert_eq!(ability.kind, AbilityKind::Talent);
    }

    #[test]
    fn ability_lookup_returns_da2_row_when_requested() {
        let lookup = create_test_lookup();
        let ability = lookup.ability(5000, Some(GameId::Da2)).unwrap().unwrap();
        assert_eq!(ability.name.as_deref(), Some("DA2 Test Talent"));
        assert_eq!(ability.tree.as_deref(), Some("DA2 Tree"));
        assert_eq!(ability.kind, AbilityKind::Talent);
    }

    #[test]
    fn abilities_by_kind_is_scoped_to_requested_game() {
        let lookup = create_test_lookup();

        let dao_talents = lookup
            .abilities_by_kind(AbilityKind::Talent, Some(GameId::Dao))
            .unwrap();
        let da2_talents = lookup
            .abilities_by_kind(AbilityKind::Talent, Some(GameId::Da2))
            .unwrap();

        assert_eq!(dao_talents.len(), 1);
        assert_eq!(dao_talents[0].name.as_deref(), Some("DAO Test Talent"));
        assert_eq!(da2_talents.len(), 1);
        assert_eq!(da2_talents[0].name.as_deref(), Some("DA2 Test Talent"));
    }

    #[test]
    fn material_lookup_returns_dao_material_metadata() {
        let lookup = create_test_lookup();
        let material = lookup.material_info(45, Some(GameId::Dao)).unwrap().unwrap();
        assert_eq!(material.tier, 6);
        assert_eq!(material.name, "Silverite");
    }

    #[test]
    fn item_material_profile_returns_db_backed_shape() {
        let lookup = create_test_lookup();
        let profile = lookup
            .item_material_profile("gen_im_wep_mel_lsw_lsw", Some(GameId::Dao))
            .unwrap()
            .unwrap();
        assert_eq!(profile.family, MaterialFamily::Metal);
        assert_eq!(profile.target, MaterialTarget::Weapon);
    }

    #[test]
    fn material_options_are_scoped_by_family_and_target() {
        let lookup = create_test_lookup();
        let options = lookup
            .material_options(MaterialFamily::Metal, MaterialTarget::Weapon, Some(GameId::Dao))
            .unwrap();
        assert_eq!(options.len(), 1);
        assert_eq!(options[0].code, 45);
    }

    #[test]
    fn real_da2_mage_abilities_are_classified_as_spells() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let ability = lookup.ability(301000, Some(GameId::Da2)).unwrap().unwrap();
        assert_eq!(ability.name.as_deref(), Some("Elemental"));
        assert_eq!(ability.kind, AbilityKind::Spell);
    }

    #[test]
    fn dao_core_skill_rows_have_behavioral_names() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();

        let player_unlock = lookup.ability(4001, Some(GameId::Dao)).unwrap().unwrap();
        let humanoid_unlock = lookup.ability(4002, Some(GameId::Dao)).unwrap().unwrap();

        assert_eq!(player_unlock.name.as_deref(), Some("Player Skill Unlock"));
        assert_eq!(humanoid_unlock.name.as_deref(), Some("Humanoid Skill Unlock"));
    }

    #[test]
    fn dao_coercion_rows_require_player_skill_unlock() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();

        for ability_id in [100011_u32, 100012, 100013, 100014] {
            let ability = lookup.ability(ability_id, Some(GameId::Dao)).unwrap().unwrap();
            assert_eq!(ability.core_ids, vec![4001]);
        }
    }

    #[test]
    fn dao_normal_skills_require_humanoid_skill_unlock() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();

        for ability_id in [100021_u32, 100061, 100100, 100110] {
            let ability = lookup.ability(ability_id, Some(GameId::Dao)).unwrap().unwrap();
            assert_eq!(ability.core_ids, vec![4002]);
        }
    }

    #[test]
    fn awakening_skills_require_humanoid_skill_unlock() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();

        for ability_id in [410000_u32, 410100, 410200] {
            let ability = lookup.ability(ability_id, Some(GameId::Dao)).unwrap().unwrap();
            assert_eq!(ability.core_ids, vec![4002]);
        }
    }
}
