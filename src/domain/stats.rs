use crate::domain::gamedata::GameId;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointPoolKind {
    Attribute,
    Skill,
    Talent,
    Specialization,
}

pub fn core_stat_id(stat: CoreStat) -> u32 {
    match stat {
        CoreStat::Strength => 1,
        CoreStat::Dexterity => 2,
        CoreStat::Willpower => 3,
        CoreStat::Magic => 4,
        CoreStat::Cunning => 5,
        CoreStat::Constitution => 6,
    }
}

pub fn core_stat_from_id(stat_id: u32) -> Option<CoreStat> {
    match stat_id {
        1 => Some(CoreStat::Strength),
        2 => Some(CoreStat::Dexterity),
        3 => Some(CoreStat::Willpower),
        4 => Some(CoreStat::Magic),
        5 => Some(CoreStat::Cunning),
        6 => Some(CoreStat::Constitution),
        _ => None,
    }
}

pub fn level_stat_id(preferred_game: Option<GameId>) -> u32 {
    match preferred_game {
        Some(GameId::Da2) => 36,
        _ => 15,
    }
}

pub fn experience_stat_id(preferred_game: Option<GameId>) -> u32 {
    match preferred_game {
        Some(GameId::Da2) => 35,
        _ => 19,
    }
}

pub fn point_pool_stat_id(kind: PointPoolKind, preferred_game: Option<GameId>) -> Option<u32> {
    match (preferred_game, kind) {
        (Some(GameId::Da2), PointPoolKind::Attribute) => Some(38),
        (Some(GameId::Da2), PointPoolKind::Talent) => Some(39),
        (Some(GameId::Da2), PointPoolKind::Skill | PointPoolKind::Specialization) => None,
        (_, PointPoolKind::Attribute) => Some(34),
        (_, PointPoolKind::Skill) => Some(35),
        (_, PointPoolKind::Talent) => Some(36),
        (_, PointPoolKind::Specialization) => Some(38),
    }
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

#[cfg(test)]
mod tests {
    use super::{
        CoreStat, PointPoolKind, core_stat_from_id, core_stat_id, experience_stat_id,
        level_stat_id, point_pool_stat_id,
    };
    use crate::domain::gamedata::GameId;

    #[test]
    fn core_stat_ids_are_stable() {
        assert_eq!(core_stat_id(CoreStat::Strength), 1);
        assert_eq!(core_stat_id(CoreStat::Dexterity), 2);
        assert_eq!(core_stat_id(CoreStat::Willpower), 3);
        assert_eq!(core_stat_id(CoreStat::Magic), 4);
        assert_eq!(core_stat_id(CoreStat::Cunning), 5);
        assert_eq!(core_stat_id(CoreStat::Constitution), 6);
        assert_eq!(core_stat_from_id(4), Some(CoreStat::Magic));
        assert_eq!(core_stat_from_id(99), None);
    }

    #[test]
    fn dao_family_and_unknown_use_dao_progress_stat_ids() {
        for game in [None, Some(GameId::Dao), Some(GameId::DaoAwakening)] {
            assert_eq!(level_stat_id(game), 15);
            assert_eq!(experience_stat_id(game), 19);
            assert_eq!(point_pool_stat_id(PointPoolKind::Attribute, game), Some(34));
            assert_eq!(point_pool_stat_id(PointPoolKind::Skill, game), Some(35));
            assert_eq!(point_pool_stat_id(PointPoolKind::Talent, game), Some(36));
            assert_eq!(
                point_pool_stat_id(PointPoolKind::Specialization, game),
                Some(38)
            );
        }
    }

    #[test]
    fn da2_uses_da2_progress_stat_ids() {
        let game = Some(GameId::Da2);
        assert_eq!(level_stat_id(game), 36);
        assert_eq!(experience_stat_id(game), 35);
        assert_eq!(point_pool_stat_id(PointPoolKind::Attribute, game), Some(38));
        assert_eq!(point_pool_stat_id(PointPoolKind::Talent, game), Some(39));
        assert_eq!(point_pool_stat_id(PointPoolKind::Skill, game), None);
        assert_eq!(
            point_pool_stat_id(PointPoolKind::Specialization, game),
            None
        );
    }
}
