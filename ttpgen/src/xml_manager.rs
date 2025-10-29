use crate::data_set::{CapacityConstraints, Distance, Rawdata, SeparationConstraints, Slot, Team};
use roxmltree::Document;
use std::fs;

/// Structure responsible for managing XML file reading and parsing.
pub struct XmlManager;

impl XmlManager {

    /// Reads an XML file and parses into a `Rawdata` struct.
    ///
    /// This function opens the specified XML file, parses it using `roxmltree`,
    /// and fills a `Rawdata` struct with information about the instance name,
    /// teams, slots, distances, capacity constraints, and separation constraints.
    ///
    /// The XML elements are mapped as follows:
    /// - `<InstanceName>` → `Rawdata.instance_name`
    /// - `<team>` → `Rawdata.teams`
    /// - `<slot>` → `Rawdata.slots`
    /// - `<distance>` → `Rawdata.distances`
    /// - Elements starting with `"CA"` → `Rawdata.capacity_constraints`
    /// - Elements starting with `"SE"` → `Rawdata.separation_constraints`
    ///
    /// # Arguments
    /// * `path` - A string slice representing the path to the XML file.
    ///
    /// # Returns
    /// A `Rawdata` struct containing all parsed information from the XML.
    ///
    /// # Panics
    /// This function will panic if the XML file cannot be opened or parsed.
    ///
    /// # Example
    /// ```
    /// let raw_data = read_xml("instances/example.xml");
    /// println!("Instance name: {}", raw_data.instance_name);
    /// println!("Number of teams: {}", raw_data.teams.len());
    /// ```
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

    /// Parses a `<Team>` XML node and converts it into a `Team` struct.
    ///
    /// This function reads the attributes of the given XML node and fills the corresponding
    /// fields in `Team`. If a numeric attribute is missing or cannot be parsed, it defaults to `0`.
    ///
    /// # Arguments
    /// * `node` - A reference to a `roxmltree::Node` representing the `<Team>` element.
    ///
    /// # Returns
    /// A `Team` struct populated with the parsed values.
    ///
    /// # Example
    /// ```
    /// let doc = roxmltree::Document::parse(r#"<Team id="5" league="1" name="Eagles" teamGroups="2"/>"#).unwrap();
    /// let node = doc.root_element();
    /// let team = parse_team(&node);
    /// assert_eq!(team.id, 5);
    /// assert_eq!(team.league, 1);
    /// assert_eq!(team.name, "Eagles".to_string());
    /// assert_eq!(team.team_groups, 2);
    /// ```
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

    /// Parses a `<Slot>` XML node and converts it into a `Slot` struct.
    ///
    /// This function reads the attributes of the given XML node and fills the corresponding
    /// fields in `Slot`. If an attribute is missing or cannot be parsed as a number,
    /// it defaults to `0`.
    ///
    /// # Arguments
    /// * `node` - A reference to a `roxmltree::Node` representing the `<Slot>` element.
    ///
    /// # Returns
    /// A `Slot` struct populated with the parsed values.
    ///
    /// # Example
    /// ```
    /// let doc = roxmltree::Document::parse(r#"<Slot id="3" name="ATL"/>"#).unwrap();
    /// assert_eq!(doc.id, 3);
    /// assert_eq!(doc.name, "ATL".to_string());
    /// ```
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

    /// Parses a `<Distance>` XML node and converts it into a `Distance` struct.
    ///
    /// This function reads the attributes of the given XML node and fills the corresponding
    /// fields in `Distance`. If an attribute is missing or cannot be parsed as a number,
    /// it defaults to `0`.
    ///
    /// # Arguments
    /// * `node` - A reference to a `roxmltree::Node` representing the `<Distance>` element.
    ///
    /// # Returns
    /// A `Distance` struct populated with the parsed values.
    ///
    /// # Example
    /// ```
    /// let doc = roxmltree::Document::parse(r#"<Distance dist="15" team1="2" team2="5"/>"#).unwrap();
    /// let node = doc.root_element();
    /// let distance = parse_distance(&node);
    /// assert_eq!(distance.dist, 15);
    /// assert_eq!(distance.team1, 2);
    /// assert_eq!(distance.team2, 5);
    /// ```
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

    /// Parses a `<CapacityConstraints>` XML node and converts it into a `CapacityConstraints` struct.
    ///
    /// This function reads the attributes of the given XML node and fills the corresponding
    /// fields in `CapacityConstraints`. If an attribute is missing or cannot be parsed,
    /// numeric fields default to `0`.
    ///
    /// # Arguments
    /// * `node` - A reference to a `roxmltree::Node` representing the `<CapacityConstraints>` element.
    ///
    /// # Returns
    /// A `CapacityConstraints` struct populated with the parsed values.
    ///
    /// # Example
    /// ```
    /// let doc = roxmltree::Document::parse(r#"<Capacity intp="2" max="5" min="1" mode1="H" mode2="A" penalty="10" teamGroups1="3" teamGroups2="2" type="hard"/>"#).unwrap();
    /// let node = doc.root_element();
    /// let capacity = parse_capacity(&node);
    /// assert_eq!(capacity.c_intp, 2);
    /// assert_eq!(capacity.c_type, "hard".to_string());
    /// ```
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

    /// Parses a `<SeparationConstraint>` XML node and converts it into a `SeparationConstraints` struct.
    ///
    /// This function reads the attributes of the given XML node and fills the corresponding
    /// fields in `SeparationConstraints`. If an attribute is missing or cannot be parsed
    /// as a number, it defaults to `0`.
    ///
    /// # Arguments
    /// * `node` - A reference to a `roxmltree::Node` representing the `<SeparationConstraint>` element.
    ///
    /// # Returns
    /// A `SeparationConstraints` struct populated with the parsed values.
    ///
    /// # Example
    /// ```
    /// let doc = roxmltree::Document::parse(r#"<Separation max="3" min="1" penalty="5" teamGroups="2" type="soft"/>"#).unwrap();
    /// let node = doc.root_element();
    /// let separation = parse_separation(&node);
    /// assert_eq!(separation.c_max, 3);
    /// assert_eq!(separation.c_type, "soft".to_string());
    /// ```
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

