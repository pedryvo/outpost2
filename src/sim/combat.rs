//! Authentic Outpost 2 Combat Simulation & Ballistics Engine
//!
//! Models unit combat engagements, weapon systems, firing arcs, laser tracers,
//! and kinetic/explosive impact detonations.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponType {
    Laser,
    Microwave,
    Railgun,
    RPG,
    EMP,
    AcidCloud,
}

impl WeaponType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Laser => "High-Energy Pulse Laser",
            Self::Microwave => "Microwave Thermal Emitter",
            Self::Railgun => "Hyper-Velocity Railgun",
            Self::RPG => "Rocket Propelled Grenade",
            Self::EMP => "EMP Disruption Arc",
            Self::AcidCloud => "Microbial Acid Cloud",
        }
    }

    pub fn damage(&self) -> u32 {
        match self {
            Self::Laser => 24,
            Self::Microwave => 18,
            Self::Railgun => 45,
            Self::RPG => 35,
            Self::EMP => 12,
            Self::AcidCloud => 28,
        }
    }

    pub fn range_tiles(&self) -> f64 {
        match self {
            Self::Laser => 5.5,
            Self::Microwave => 4.0,
            Self::Railgun => 7.0,
            Self::RPG => 6.0,
            Self::EMP => 4.5,
            Self::AcidCloud => 5.0,
        }
    }

    pub fn fire_interval(&self) -> f64 {
        match self {
            Self::Laser => 0.8,
            Self::Microwave => 0.6,
            Self::Railgun => 1.8,
            Self::RPG => 1.4,
            Self::EMP => 2.0,
            Self::AcidCloud => 1.2,
        }
    }

    pub fn beam_color(&self) -> &'static str {
        match self {
            Self::Laser => "#00d4ff",
            Self::Microwave => "#facc15",
            Self::Railgun => "#e2e8f0",
            Self::RPG => "#f97316",
            Self::EMP => "#c084fc",
            Self::AcidCloud => "#22c55e",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CombatBeam {
    pub from_x: f64,
    pub from_y: f64,
    pub to_x: f64,
    pub to_y: f64,
    pub color: &'static str,
    pub lifetime: f64,
    pub max_lifetime: f64,
}

#[derive(Debug, Clone)]
pub struct ExplosionEffect {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub max_radius: f64,
    pub lifetime: f64,
    pub max_lifetime: f64,
}
