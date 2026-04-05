#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityKind {
    Skill,
    Talent,
    Spell,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbilityRef {
    pub id: u32,
    pub name: Option<String>,
    pub tree: Option<String>,
    pub ability_type: Option<String>,
    pub kind: AbilityKind,
    pub core_ids: Vec<u32>,
}

impl AbilityKind {
    pub fn from_db_type(value: Option<&str>) -> Self {
        let Some(value) = value else {
            return Self::Unknown;
        };

        match value.trim().to_ascii_lowercase().as_str() {
            "skill" => Self::Skill,
            "spell" => Self::Spell,
            "talent" | "specialization" | "class" => Self::Talent,
            _ => Self::Unknown,
        }
    }
}
