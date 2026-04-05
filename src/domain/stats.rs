#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CoreStats {
    pub strength: u32,
    pub dexterity: u32,
    pub willpower: u32,
    pub magic: u32,
    pub cunning: u32,
    pub constitution: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PointPools {
    pub attribute_points: Option<u32>,
    pub skill_points: Option<u32>,
    pub talent_points: Option<u32>,
    pub specialization_points: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PointPoolsPatch {
    pub attribute_points: Option<u32>,
    pub skill_points: Option<u32>,
    pub talent_points: Option<u32>,
    pub specialization_points: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CoreStatsPatch {
    pub strength: Option<u32>,
    pub dexterity: Option<u32>,
    pub willpower: Option<u32>,
    pub magic: Option<u32>,
    pub cunning: Option<u32>,
    pub constitution: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreStat {
    Strength,
    Dexterity,
    Willpower,
    Magic,
    Cunning,
    Constitution,
}

impl CoreStats {
    pub fn get(self, stat: CoreStat) -> u32 {
        match stat {
            CoreStat::Strength => self.strength,
            CoreStat::Dexterity => self.dexterity,
            CoreStat::Willpower => self.willpower,
            CoreStat::Magic => self.magic,
            CoreStat::Cunning => self.cunning,
            CoreStat::Constitution => self.constitution,
        }
    }

    pub fn set(&mut self, stat: CoreStat, value: u32) {
        match stat {
            CoreStat::Strength => self.strength = value,
            CoreStat::Dexterity => self.dexterity = value,
            CoreStat::Willpower => self.willpower = value,
            CoreStat::Magic => self.magic = value,
            CoreStat::Cunning => self.cunning = value,
            CoreStat::Constitution => self.constitution = value,
        }
    }

    pub fn apply_patch(&mut self, patch: CoreStatsPatch) {
        if let Some(value) = patch.strength {
            self.strength = value;
        }
        if let Some(value) = patch.dexterity {
            self.dexterity = value;
        }
        if let Some(value) = patch.willpower {
            self.willpower = value;
        }
        if let Some(value) = patch.magic {
            self.magic = value;
        }
        if let Some(value) = patch.cunning {
            self.cunning = value;
        }
        if let Some(value) = patch.constitution {
            self.constitution = value;
        }
    }
}

impl PointPools {
    pub fn apply_patch(&mut self, patch: PointPoolsPatch) {
        if let Some(value) = patch.attribute_points {
            self.attribute_points = Some(value);
        }
        if let Some(value) = patch.skill_points {
            self.skill_points = Some(value);
        }
        if let Some(value) = patch.talent_points {
            self.talent_points = Some(value);
        }
        if let Some(value) = patch.specialization_points {
            self.specialization_points = Some(value);
        }
    }
}
