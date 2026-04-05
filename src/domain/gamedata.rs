use crate::domain::ability::{AbilityKind, AbilityRef};
use rusqlite::{params, Connection, OptionalExtension};
use std::error::Error;
use std::fmt;
use std::path::Path;

pub const DEFAULT_GAME_DATA_PATH: &str = "data\\gamedata.db";

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
}

#[cfg(test)]
mod tests {
    use super::{AbilityKind, GameDataLookup, GameId, SqliteGameData, DEFAULT_GAME_DATA_PATH};
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
                INSERT INTO abilities (id, name, core_id, tree, type, game) VALUES
                    ('5000', 'DAO Test Talent', '5001', 'DAO Tree', ' Talent ', 'dao'),
                    ('5000', 'DA2 Test Talent', '5002', 'DA2 Tree', 'Specialization', 'da2'),
                    ('5001', 'DAO Skill', NULL, 'DAO Skills', 'Skill', 'dao'),
                    ('5002', 'DA2 Spell', NULL, 'DA2 Spells', 'Spell', 'da2');
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
    fn real_da2_mage_abilities_are_classified_as_spells() {
        let lookup = SqliteGameData::open(DEFAULT_GAME_DATA_PATH).unwrap();
        let ability = lookup.ability(301000, Some(GameId::Da2)).unwrap().unwrap();
        assert_eq!(ability.name.as_deref(), Some("Elemental"));
        assert_eq!(ability.kind, AbilityKind::Spell);
    }
}
