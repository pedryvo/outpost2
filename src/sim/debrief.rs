//! Mission Debriefing, Score & Colony Statistics Engine
//!
//! Evaluates mission completion statistics, calculates weighted colonial
//! achievement ratings, assigns commander ranks, and determines victory/defeat
//! sequence presentation.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use crate::sim::Faction;

/// Commander rank awarded based on composite colonial performance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColonialRank {
    SupremeColonySavior,
    DistinguishedPioneer,
    TethysSurvivor,
    ColonyFailure,
}

impl ColonialRank {
    pub fn title(&self) -> &'static str {
        match self {
            Self::SupremeColonySavior => "Supreme Colony Savior",
            Self::DistinguishedPioneer => "Distinguished Pioneer",
            Self::TethysSurvivor => "Tethys Survivor",
            Self::ColonyFailure => "Colony Failure",
        }
    }

    pub fn badge(&self) -> &'static str {
        match self {
            Self::SupremeColonySavior => "🎖️ Supreme Colony Savior",
            Self::DistinguishedPioneer => "🌟 Distinguished Pioneer",
            Self::TethysSurvivor => "🛡️ Tethys Survivor",
            Self::ColonyFailure => "☠️ Colony Failure",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::SupremeColonySavior => "Flawless leadership: Minimal casualties, superior industrial output, and complete strategic dominance.",
            Self::DistinguishedPioneer => "Commendable command: Colonial objectives secured with resilience despite hazardous planetary conditions.",
            Self::TethysSurvivor => "Pyrrhic survival: Colony survived through heavy adversity and significant resource expenditure.",
            Self::ColonyFailure => "Catastrophic collapse: Planetary outpost lost, civilian casualties critical, or primary directives failed.",
        }
    }
}

/// Victory condition category achieved in the campaign scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VictoryType {
    StarshipEvacuation,
    TerraformerSaturation,
    CitadelDismantled,
    ObjectivesCompleted,
}

impl VictoryType {
    pub fn description(&self) -> &'static str {
        match self {
            Self::StarshipEvacuation => "Evacuation Starship fully assembled and successfully launched into orbit.",
            Self::TerraformerSaturation => "Atmospheric Terraformer achieved 100% planetary saturation.",
            Self::CitadelDismantled => "Hostile Citadel and defense divisions dismantled.",
            Self::ObjectivesCompleted => "All primary colonial directives successfully executed.",
        }
    }
}

/// Root cause of mission termination upon defeat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefeatReason {
    CommandCenterDestroyed,
    TerraformerDestroyed,
    ContagionBreach,
    PopulationExtinct,
    EvacuationFailed,
    MissionFailed,
}

impl DefeatReason {
    pub fn description(&self) -> &'static str {
        match self {
            Self::CommandCenterDestroyed => "Colony Command Center has been obliterated.",
            Self::TerraformerDestroyed => "Atmospheric Terraformer destroyed by hostile attack.",
            Self::ContagionBreach => "Nanite blight contagion corrupted atmospheric processing.",
            Self::PopulationExtinct => "All colonial civilian personnel perished.",
            Self::EvacuationFailed => "Evacuation quota was not fulfilled before colony collapse.",
            Self::MissionFailed => "Critical scenario failure condition triggered.",
        }
    }
}

/// Comprehensive post-mission statistics and evaluation metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionDebrief {
    pub sol_survival_time: f64,
    pub total_common_ore_mined: u32,
    pub total_rare_ore_mined: u32,
    pub techs_researched: u32,
    pub enemy_units_destroyed: u32,
    pub friendly_units_lost: u32,
    pub civilian_population_saved: u32,
    pub civilian_casualties: u32,
    pub starship_launch_progress: f64,
    pub score: u32,
    pub rank: ColonialRank,
}

impl MissionDebrief {
    pub fn new(
        sol_survival_time: f64,
        total_common_ore_mined: u32,
        total_rare_ore_mined: u32,
        techs_researched: u32,
        enemy_units_destroyed: u32,
        friendly_units_lost: u32,
        civilian_population_saved: u32,
        civilian_casualties: u32,
        starship_launch_progress: f64,
        is_victory: bool,
    ) -> Self {
        let (score, rank) = Self::calculate_score(
            sol_survival_time,
            total_common_ore_mined,
            total_rare_ore_mined,
            techs_researched,
            enemy_units_destroyed,
            friendly_units_lost,
            civilian_population_saved,
            civilian_casualties,
            starship_launch_progress,
            is_victory,
        );

        Self {
            sol_survival_time,
            total_common_ore_mined,
            total_rare_ore_mined,
            techs_researched,
            enemy_units_destroyed,
            friendly_units_lost,
            civilian_population_saved,
            civilian_casualties,
            starship_launch_progress,
            score,
            rank,
        }
    }

    /// Calculate weighted composite colonial score and award rank
    pub fn calculate_score(
        sol_survival_time: f64,
        total_common_ore_mined: u32,
        total_rare_ore_mined: u32,
        techs_researched: u32,
        enemy_units_destroyed: u32,
        friendly_units_lost: u32,
        civilian_population_saved: u32,
        civilian_casualties: u32,
        starship_launch_progress: f64,
        is_victory: bool,
    ) -> (u32, ColonialRank) {
        if !is_victory {
            // Defeat yields Colony Failure rank
            let mut def_score = (civilian_population_saved * 15)
                + (total_common_ore_mined / 50)
                + (enemy_units_destroyed * 50);
            def_score = def_score.saturating_sub(friendly_units_lost * 100);
            def_score = def_score.saturating_sub(civilian_casualties * 100);
            return (def_score.min(2499), ColonialRank::ColonyFailure);
        }

        let mut score: i64 = 5000; // Base victory bounty

        // Civilian population saved bonus (+100 per colonist)
        score += civilian_population_saved as i64 * 100;

        // Casualties penalty (-150 per civilian casualty or lost friendly combat vehicle)
        score -= (civilian_casualties as i64 + friendly_units_lost as i64) * 150;

        // Mineral extraction composite (+1 per 10 common ore, +1 per 5 rare ore)
        score += (total_common_ore_mined as i64) / 10;
        score += (total_rare_ore_mined as i64) / 5;

        // Technological advancements (+250 per researched technology)
        score += techs_researched as i64 * 250;

        // Tactical combat superiority (+150 per enemy vehicle / structure destroyed)
        score += enemy_units_destroyed as i64 * 150;

        // Starship launch progress bonus (up to 2,000 points at 100% assembly)
        score += (starship_launch_progress * 20.0).round() as i64;

        // Sol efficiency bonus: rewards prompt strategic completion
        if sol_survival_time > 0.0 {
            let sol_bonus = ((100.0 - sol_survival_time).max(0.0) * 40.0) as i64;
            score += sol_bonus;
        }

        let final_score = score.max(0) as u32;

        let rank = if final_score >= 15000 {
            ColonialRank::SupremeColonySavior
        } else if final_score >= 8000 {
            ColonialRank::DistinguishedPioneer
        } else if final_score >= 2500 {
            ColonialRank::TethysSurvivor
        } else {
            ColonialRank::ColonyFailure
        };

        (final_score, rank)
    }
}

/// End game cinematic sequence representation encapsulating faction, outcome, and audio triggers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EndGameSequence {
    Victory {
        faction: Faction,
        victory_type: VictoryType,
        score: MissionDebrief,
    },
    Defeat {
        faction: Faction,
        defeat_reason: DefeatReason,
        score: MissionDebrief,
    },
}

impl EndGameSequence {
    pub fn is_victory(&self) -> bool {
        matches!(self, Self::Victory { .. })
    }

    pub fn faction(&self) -> Faction {
        match self {
            Self::Victory { faction, .. } => *faction,
            Self::Defeat { faction, .. } => *faction,
        }
    }

    pub fn score(&self) -> &MissionDebrief {
        match self {
            Self::Victory { score, .. } => score,
            Self::Defeat { score, .. } => score,
        }
    }

    /// Audio sample trigger: returns "uni_grad.wav" on victory.
    pub fn audio_sample(&self) -> Option<&'static str> {
        match self {
            Self::Victory { .. } => Some("uni_grad.wav"),
            Self::Defeat { .. } => None,
        }
    }

    /// Legacy sound/voice event trigger: returns event 227 on defeat ("You have failed, our colony is doomed").
    pub fn defeat_voice_id(&self) -> Option<u32> {
        match self {
            Self::Victory { .. } => None,
            Self::Defeat { .. } => Some(227),
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            Self::Victory { .. } => "MISSION ACCOMPLISHED: VICTORY",
            Self::Defeat { .. } => "COLONIAL DEFEAT: MISSION TERMINATED",
        }
    }

    pub fn summary(&self) -> String {
        match self {
            Self::Victory { faction, victory_type, score } => {
                format!(
                    "{} Strategic Victory! {}\nRank Awarded: {} ({})\nComposite Score: {}",
                    match faction { Faction::Eden => "Eden Foundation", Faction::Plymouth => "Plymouth Colony" },
                    victory_type.description(),
                    score.rank.title(),
                    score.rank.badge(),
                    score.score,
                )
            }
            Self::Defeat { faction, defeat_reason, score } => {
                format!(
                    "{} Colonial Collapse! {}\nRank: {} ({})\nScore: {}",
                    match faction { Faction::Eden => "Eden Foundation", Faction::Plymouth => "Plymouth Colony" },
                    defeat_reason.description(),
                    score.rank.title(),
                    score.rank.badge(),
                    score.score,
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_calculation_supreme_colony_savior() {
        let (score, rank) = MissionDebrief::calculate_score(
            15.0,   // sol survival
            12_000, // common ore mined
            4_000,  // rare ore mined
            6,      // techs researched
            12,     // enemy units destroyed
            0,      // friendly units lost
            160,    // civilians saved
            0,      // civilian casualties
            100.0,  // starship progress
            true,   // is_victory
        );

        assert!(score >= 15000, "Score {} should meet SupremeColonySavior threshold (15000)", score);
        assert_eq!(rank, ColonialRank::SupremeColonySavior);
        assert_eq!(rank.title(), "Supreme Colony Savior");
    }

    #[test]
    fn test_score_calculation_distinguished_pioneer() {
        let (score, rank) = MissionDebrief::calculate_score(
            30.0,
            4_000,
            1_000,
            2,
            5,
            2,
            50,
            5,
            50.0,
            true,
        );

        assert!(score >= 8000 && score < 15000, "Score {} should fall in DistinguishedPioneer tier", score);
        assert_eq!(rank, ColonialRank::DistinguishedPioneer);
        assert_eq!(rank.title(), "Distinguished Pioneer");
    }

    #[test]
    fn test_score_calculation_tethys_survivor() {
        let (score, rank) = MissionDebrief::calculate_score(
            60.0,
            1_500,
            200,
            1,
            2,
            8,
            30,
            25,
            0.0,
            true,
        );

        assert!(score >= 2500 && score < 8000, "Score {} should fall in TethysSurvivor tier", score);
        assert_eq!(rank, ColonialRank::TethysSurvivor);
        assert_eq!(rank.title(), "Tethys Survivor");
    }

    #[test]
    fn test_score_calculation_defeat_colony_failure() {
        let (score, rank) = MissionDebrief::calculate_score(
            10.0,
            1_000,
            100,
            0,
            1,
            5,
            20,
            50,
            25.0,
            false, // defeat
        );

        assert!(score < 2500, "Defeat score {} should be below 2500", score);
        assert_eq!(rank, ColonialRank::ColonyFailure);
        assert_eq!(rank.title(), "Colony Failure");
    }

    #[test]
    fn test_end_game_sequence_audio_triggers() {
        let debrief = MissionDebrief::new(
            12.0, 5000, 1000, 3, 4, 1, 150, 0, 100.0, true
        );
        let victory_seq = EndGameSequence::Victory {
            faction: Faction::Eden,
            victory_type: VictoryType::StarshipEvacuation,
            score: debrief.clone(),
        };

        assert!(victory_seq.is_victory());
        assert_eq!(victory_seq.audio_sample(), Some("uni_grad.wav"));
        assert_eq!(victory_seq.defeat_voice_id(), None);
        assert_eq!(victory_seq.faction(), Faction::Eden);

        let defeat_seq = EndGameSequence::Defeat {
            faction: Faction::Plymouth,
            defeat_reason: DefeatReason::CommandCenterDestroyed,
            score: debrief,
        };

        assert!(!defeat_seq.is_victory());
        assert_eq!(defeat_seq.audio_sample(), None);
        assert_eq!(defeat_seq.defeat_voice_id(), Some(227));
        assert_eq!(defeat_seq.faction(), Faction::Plymouth);
    }
}
