//! Outpost 2 Technology Tree and Research Parser

#[derive(Debug, Clone)]
pub struct TechNode {
    pub id: u32,
    pub name: String,
    pub category: u32,
    pub cost: u32,
    pub description: String,
    pub is_researched: bool,
}

#[derive(Debug, Clone)]
pub struct TechTree {
    pub techs: Vec<TechNode>,
}

impl TechTree {
    /// Create the canonical initial technology tree for Outpost 2
    pub fn new() -> Self {
        let mut techs = Vec::new();

        techs.push(TechNode {
            id: 2106,
            name: "Basic Laboratory".into(),
            category: 0,
            cost: 0,
            description: "Essential scientific facility to unlock planetary exploration and advanced survival technologies.".into(),
            is_researched: true,
        });

        techs.push(TechNode {
            id: 2701,
            name: "Astronomy".into(),
            category: 1,
            cost: 400,
            description: "Rebuilds our astronomical satellite database, mapping resource distribution and orbital hazards on New Terra.".into(),
            is_researched: false,
        });

        techs.push(TechNode {
            id: 3301,
            name: "Hydroponics Automation".into(),
            category: 2,
            cost: 650,
            description: "Automated nutrient feeding arrays for higher food output per bio-dome habitat.".into(),
            is_researched: false,
        });

        techs.push(TechNode {
            id: 3402,
            name: "High-Temperature Smelting".into(),
            category: 3,
            cost: 800,
            description: "Advanced plasma-arc furnaces allowing rare metals refinement at 30% greater throughput.".into(),
            is_researched: false,
        });

        techs.push(TechNode {
            id: 4201,
            name: "Lava Containment Walls".into(),
            category: 4,
            cost: 1100,
            description: "Reinforced ceramic thermal barriers designed to withstand slow-moving basaltic lava flows.".into(),
            is_researched: false,
        });

        techs.push(TechNode {
            id: 5104,
            name: "Lynx Chassis Reinforced Armor".into(),
            category: 5,
            cost: 950,
            description: "Alloy composite plates for light scout vehicles, increasing combat chassis durability by 25%.".into(),
            is_researched: false,
        });

        techs.push(TechNode {
            id: 5202,
            name: "Pulse Laser Optics".into(),
            category: 5,
            cost: 1200,
            description: "Refined optical focusing crystals granting +15 beam intensity to laser armaments.".into(),
            is_researched: false,
        });

        techs.push(TechNode {
            id: 6101,
            name: "Cryogenic Evacuation Modules".into(),
            category: 1,
            cost: 1500,
            description: "Advanced passenger stasis units ensuring high survivor payload for the Evacuation Starship.".into(),
            is_researched: false,
        });

        Self { techs }
    }

    pub fn is_researched(&self, id: u32) -> bool {
        self.techs.iter().any(|t| t.id == id && t.is_researched)
    }

    pub fn category_name(cat: u32) -> &'static str {
        match cat {
            0 => "Core Science",
            1 => "Space & Orbital",
            2 => "Life Support",
            3 => "Metallurgy",
            4 => "Engineering",
            5 => "Military Robotics",
            _ => "Applied Physics",
        }
    }

    /// Extra armor points for combat vehicles
    pub fn armor_bonus(&self) -> u32 {
        if self.is_researched(5104) { 50 } else { 0 }
    }

    /// Bonus damage for laser weaponry
    pub fn laser_damage_bonus(&self) -> u32 {
        if self.is_researched(5202) { 15 } else { 0 }
    }

    /// Ore smelting output multiplier
    pub fn smelting_multiplier(&self) -> f64 {
        if self.is_researched(3402) { 1.25 } else { 1.0 }
    }

    /// Food production multiplier
    pub fn food_multiplier(&self) -> f64 {
        if self.is_researched(3301) { 1.50 } else { 1.0 }
    }

    /// Whether fortified walls can be deployed
    pub fn can_build_walls(&self) -> bool {
        self.is_researched(4201)
    }

    /// Attempt to complete research on a technology node
    pub fn research(&mut self, tech_id: u32, ore_budget: &mut u32) -> Result<String, String> {
        if let Some(node) = self.techs.iter_mut().find(|t| t.id == tech_id) {
            if node.is_researched {
                return Err(format!("'{}' is already researched.", node.name));
            }
            if *ore_budget < node.cost {
                return Err(format!("Insufficient common ore! Need {} t, have {} t.", node.cost, ore_budget));
            }
            *ore_budget -= node.cost;
            node.is_researched = true;
            Ok(format!("Research complete: '{}'!", node.name))
        } else {
            Err("Tech node not found in database".into())
        }
    }
}
