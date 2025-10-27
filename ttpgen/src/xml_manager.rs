use crate::data_set::{CapacityConstraints, Distance, Rawdata, SeparationConstraints, Slot, Team};
use roxmltree::Document;
use std::fs;

/// Structure responsible for managing XML file reading and parsing.
pub struct XmlManager;

impl XmlManager {
    /// Reads an XML file from the given path and returns a `Rawdata` struct
    /// containing all extracted information, including teams, slots, distances,
    /// capacity constraints, and separation constraints.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice representing the path to the XML file.
    ///
    /// # Returns
    ///
    /// A `Rawdata` instance populated with all data found in the XML.
    pub fn read_xml(path: &str) -> Rawdata {
        let xml = fs::read_to_string(path).expect("Error opening XML file");
        let doc = Document::parse(&xml).expect("Error parsing XML");

        let mut raw_data = Rawdata {
            instance_name: String::new(),
            teams: Vec::new(),
            slots: Vec::new(),
            distances: Vec::new(),
            capacity_constraints: Vec::new(),
            separation_constraints: Vec::new(),
        };

        for node in doc.descendants().filter(|n| n.is_element()) {
            match node.tag_name().name() {
                "InstanceName" => {
                    if let Some(text) = node.text() {
                        raw_data.instance_name = text.to_string();
                    }
                }
                "team" => raw_data.teams.push(Self::parse_team(&node)),
                "slot" => raw_data.slots.push(Self::parse_slot(&node)),
                "distance" => raw_data.distances.push(Self::parse_distance(&node)),
                name if name.starts_with("CA") => raw_data.capacity_constraints.push(Self::parse_capacity(&node)),
                name if name.starts_with("SE") => raw_data.separation_constraints.push(Self::parse_separation(&node)),
                _ => {}
            }
        }

        raw_data
    }

    /// Parses a `Team` from an XML node.
    ///
    /// # Arguments
    ///
    /// * `node` - Reference to the XML node representing a team.
    ///
    /// # Returns
    ///
    /// A `Team` struct populated with attributes from the XML node.
    fn parse_team(node: &roxmltree::Node) -> Team {
        let mut team = Team::new();
        for attr in node.attributes() {
            match attr.name() {
                "id" => team.id = attr.value().parse().unwrap_or(0),
                "league" => team.league = attr.value().parse().unwrap_or(0),
                "name" => team.name = attr.value().to_string(),
                "teamGroups" => team.team_groups = attr.value().parse().unwrap_or(0),
                _ => {}
            }
        }
        team
    }

    /// Parses a `Slot` from an XML node.
    fn parse_slot(node: &roxmltree::Node) -> Slot {
        let mut slot = Slot::new();
        for attr in node.attributes() {
            match attr.name() {
                "id" => slot.id = attr.value().parse().unwrap_or(0),
                "name" => slot.name = attr.value().to_string(),
                _ => {}
            }
        }
        slot
    }

    /// Parses a `Distance` from an XML node.
    fn parse_distance(node: &roxmltree::Node) -> Distance {
        let mut distance = Distance::new();
        for attr in node.attributes() {
            match attr.name() {
                "dist" => distance.dist = attr.value().parse().unwrap_or(0),
                "team1" => distance.team1 = attr.value().parse().unwrap_or(0),
                "team2" => distance.team2 = attr.value().parse().unwrap_or(0),
                _ => {}
            }
        }
        distance
    }

    /// Parses a `CapacityConstraints` instance from an XML node.
    fn parse_capacity(node: &roxmltree::Node) -> CapacityConstraints {
        let mut cap = CapacityConstraints::new();
        for attr in node.attributes() {
            match attr.name() {
                "intp" => cap.c_intp = attr.value().parse().unwrap_or(0),
                "max" => cap.c_max = attr.value().parse().unwrap_or(0),
                "min" => cap.c_min = attr.value().parse().unwrap_or(0),
                "mode1" => cap.c_mode1 = attr.value().chars().next().unwrap_or('n'),
                "mode2" => cap.c_mode2 = attr.value().to_string(),
                "penalty" => cap.c_penalty = attr.value().parse().unwrap_or(0),
                "teamGroups1" => cap.c_team_groups1 = attr.value().parse().unwrap_or(0),
                "teamGroups2" => cap.c_team_groups2 = attr.value().parse().unwrap_or(0),
                "type" => cap.c_type = attr.value().to_string(),
                _ => {}
            }
        }
        cap
    }

    /// Parses a `SeparationConstraints` instance from an XML node.
    fn parse_separation(node: &roxmltree::Node) -> SeparationConstraints {
        let mut sep = SeparationConstraints::new();
        for attr in node.attributes() {
            match attr.name() {
                "max" => sep.c_max = attr.value().parse().unwrap_or(0),
                "min" => sep.c_min = attr.value().parse().unwrap_or(0),
                "penalty" => sep.c_penalty = attr.value().parse().unwrap_or(0),
                "teamGroups" => sep.c_team_groups = attr.value().parse().unwrap_or(0),
                "type" => sep.c_type = attr.value().to_string(),
                _ => {}
            }
        }
        sep
    }
}

