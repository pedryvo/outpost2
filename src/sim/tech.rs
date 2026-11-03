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

        Self { techs }
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
