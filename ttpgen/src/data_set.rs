use serde::{Serialize, Deserialize};

/// All raw data parsed from a TTP XML instance.
///
/// Contains all information necessary to generate solutions,
/// including teams, slots, distances, and constraints.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rawdata {
    /// Name of the instance.
    pub instance_name: String,
    /// List of teams participating in the tournament.
    pub teams: Vec<Team>,
    /// List of time slots or rounds.
    pub slots: Vec<Slot>,
    /// Pair travel distances between teams.
    pub distances: Vec<Distance>,
    /// Capacity constraints for the tournament.
    pub capacity_constraints: Vec<CapacityConstraints>,
    /// Separation constraints for the tournament.
    pub separation_constraints: Vec<SeparationConstraints>,
}

/// Represents the travel distance between two teams.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Distance {
    /// Distance value between two teams.
    pub dist: i32,
    /// First team ID.
    pub team1: i32,
    /// Second team ID.
    pub team2: i32,
}

impl Distance {
    /// Creates a new Distance with default values (0).
    pub fn new() -> Self {
        Self {
            dist: 0,
            team1: 0,
            team2: 0,
        }
    }
}

/// Represents a team in the tournament.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Team {
    /// Unique team ID.
    pub id: i32,
    /// League or division ID.
    pub league: i32,
    /// Name of the team.
    pub name: String,
    /// Team group or category.
    pub team_groups: i32,
}

impl Team {
    /// Creates a new Team with default values.
    pub fn new() -> Self {
        Self {
            id: 0,
            league: 0,
            name: "Null".to_string(),
            team_groups: 0,
        }
    }
}

/// Represents a time slot or round in the tournament.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Slot {
    /// Slot ID.
    pub id: i32,
    /// Name or label of the slot.
    pub name: String,
}

impl Slot {
    /// Creates a new Slot with default values.
    pub fn new() -> Self {
        Self {
            id: 0,
            name: "Null".to_string(),
        }
    }
}

/// Represents capacity constraints for the tournament.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapacityConstraints {
    /// Interval parameter
    pub c_intp: i32,
    /// Maximum allowed occurrences.
    pub c_max: i32,
    /// Minimum required occurrences.
    pub c_min: i32,
    /// Mode type 1 ('A', 'H').
    pub c_mode1: char,
    /// Mode type 2 (string).
    pub c_mode2: String,
    /// Penalty value for violation.
    pub c_penalty: i32,
    /// First affected team group.
    pub c_team_groups1: i32,
    /// Second affected team group.
    pub c_team_groups2: i32,
    /// Type of constraint (description).
    pub c_type: String,
}

impl CapacityConstraints {
    /// Creates a new CapacityConstraints instance with default values.
    pub fn new() -> Self {
        Self {
            c_intp: 0,
            c_max: 0,
            c_min: 0,
            c_mode1: 'N',
            c_mode2: "Null".to_string(),
            c_penalty: 0,
            c_team_groups1: 0,
            c_team_groups2: 0,
            c_type: "Null".to_string(),
        }
    }
}

/// Represents separation constraints for the tournament.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SeparationConstraints {
    /// Maximum allowed distance between occurrences.
    pub c_max: i32,
    /// Minimum required distance between occurrences.
    pub c_min: i32,
    /// Penalty value for violation.
    pub c_penalty: i32,
    /// Team group affected by the constraint.
    pub c_team_groups: i32,
    /// Type of constraint (description).
    pub c_type: String,
}

impl SeparationConstraints {
    /// Creates a new SeparationConstraints instance with default values.
    pub fn new() -> Self {
        Self {
            c_max: 0,
            c_min: 0,
            c_penalty: 0,
            c_team_groups: 0,
            c_type: "Null".to_string(),
        }
    }
}
