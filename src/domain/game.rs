use crate::domain::gamedata::GameId;
use crate::domain::stats::{
    CoreStat, PointPoolKind, core_stat_id, experience_stat_id, level_stat_id, point_pool_stat_id,
};
use crate::gff4::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityListStyle {
    Separate,
    Combined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyPowerEncoding {
    Float,
    Da2Bitcast,
}

pub trait GameBehavior {
    fn stat_id_for(self, stat: CoreStat) -> u32;
    fn level_stat_id(self) -> u32;
    fn experience_stat_id(self) -> u32;
    fn point_pool_stat_id(self, kind: PointPoolKind) -> Option<u32>;
    fn encode_property_power(self, value: f32) -> Value;
    fn property_power_encoding(self) -> PropertyPowerEncoding;
    fn ability_list_style(self) -> AbilityListStyle;
    fn supports_plot_flags(self) -> bool;
}

impl GameBehavior for Option<GameId> {
    fn stat_id_for(self, stat: CoreStat) -> u32 {
        core_stat_id(stat)
    }

    fn level_stat_id(self) -> u32 {
        level_stat_id(self)
    }

    fn experience_stat_id(self) -> u32 {
        experience_stat_id(self)
    }

    fn point_pool_stat_id(self, kind: PointPoolKind) -> Option<u32> {
        point_pool_stat_id(kind, self)
    }

    fn encode_property_power(self, value: f32) -> Value {
        match self.property_power_encoding() {
            PropertyPowerEncoding::Float => Value::Float32(value),
            PropertyPowerEncoding::Da2Bitcast => Value::UInt32(value.to_bits()),
        }
    }

    fn property_power_encoding(self) -> PropertyPowerEncoding {
        if self.is_some_and(GameId::is_da2) {
            PropertyPowerEncoding::Da2Bitcast
        } else {
            PropertyPowerEncoding::Float
        }
    }

    fn ability_list_style(self) -> AbilityListStyle {
        if self.is_some_and(GameId::is_da2) {
            AbilityListStyle::Combined
        } else {
            AbilityListStyle::Separate
        }
    }

    fn supports_plot_flags(self) -> bool {
        self.is_some_and(GameId::is_da2)
    }
}

impl GameBehavior for GameId {
    fn stat_id_for(self, stat: CoreStat) -> u32 {
        Some(self).stat_id_for(stat)
    }

    fn level_stat_id(self) -> u32 {
        Some(self).level_stat_id()
    }

    fn experience_stat_id(self) -> u32 {
        Some(self).experience_stat_id()
    }

    fn point_pool_stat_id(self, kind: PointPoolKind) -> Option<u32> {
        Some(self).point_pool_stat_id(kind)
    }

    fn encode_property_power(self, value: f32) -> Value {
        Some(self).encode_property_power(value)
    }

    fn property_power_encoding(self) -> PropertyPowerEncoding {
        Some(self).property_power_encoding()
    }

    fn ability_list_style(self) -> AbilityListStyle {
        Some(self).ability_list_style()
    }

    fn supports_plot_flags(self) -> bool {
        Some(self).supports_plot_flags()
    }
}

#[cfg(test)]
mod tests {
    use super::{AbilityListStyle, GameBehavior, PropertyPowerEncoding};
    use crate::domain::gamedata::GameId;
    use crate::domain::stats::{CoreStat, PointPoolKind};
    use crate::gff4::Value;

    #[test]
    fn game_behavior_maps_core_features() {
        assert_eq!(GameId::Dao.stat_id_for(CoreStat::Strength), 1);
        assert_eq!(GameId::Dao.level_stat_id(), 15);
        assert_eq!(GameId::Da2.level_stat_id(), 36);
        assert_eq!(GameId::Da2.experience_stat_id(), 35);
        assert_eq!(GameId::Da2.point_pool_stat_id(PointPoolKind::Skill), None);
        assert_eq!(
            GameId::Da2.point_pool_stat_id(PointPoolKind::Talent),
            Some(39)
        );
    }

    #[test]
    fn game_behavior_covers_storage_and_plot_flags() {
        assert_eq!(GameId::Dao.ability_list_style(), AbilityListStyle::Separate);
        assert_eq!(GameId::Da2.ability_list_style(), AbilityListStyle::Combined);
        assert_eq!(
            GameId::Dao.property_power_encoding(),
            PropertyPowerEncoding::Float
        );
        assert_eq!(
            GameId::Da2.property_power_encoding(),
            PropertyPowerEncoding::Da2Bitcast
        );
        assert!(!GameId::Dao.supports_plot_flags());
        assert!(GameId::Da2.supports_plot_flags());
    }

    #[test]
    fn game_behavior_encodes_property_power() {
        assert_eq!(GameId::Dao.encode_property_power(1.5), Value::Float32(1.5));
        assert_eq!(
            GameId::Da2.encode_property_power(1.5),
            Value::UInt32(1.5f32.to_bits())
        );
    }
}
