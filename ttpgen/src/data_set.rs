/// All raw data parsed from the TTP XML file,
/// including teams, slots, distances, and constraints.

#[derive(Clone, Debug)]
pub struct Rawdata {
    /// The name of the instance.
    pub instance_name: String,
    /// List of teams in the tournament.
    pub teams: Vec<Team>,
    /// List of time slots or rounds.
    pub slots: Vec<Slot>,
    /// List of distances between teams.
    pub distances: Vec<Distance>,
    /// Capacity constraints.
    pub capacity_constraints: Vec<CapacityConstraints>,
    /// Separation constraints.
    pub separation_constraints: Vec<SeparationConstraints>,
}
impl Rawdata {}

/// Represents the travel distance between two teams.
#[derive(Clone, Debug)]
pub struct Distance {
    /// Distance value.
    pub dist: i32,
    /// First team.
    pub team1: i32,
    /// Second team.
    pub team2: i32,
}
impl Distance {
    /// Creates a new Distance with default values.
    pub fn new() -> Distance {
        Distance {
            dist: 0,
            team1: 0,
            team2: 0,
        }
    }
}

/// Represents a team in the tournament.
#[derive(Clone, Debug)]
pub struct Team {
    /// Team ID.
    pub id: i32,
    /// League ID or division.
    pub league: i32,
    /// Name of the team.
    pub name: String,
    /// Team group or category.
    pub team_groups: i32,
}
impl Team {
    /// Creates a new Team with default values.
    pub fn new() -> Team {
        Team {
            id: 0,
            league: 0,
            name: "Null".to_string(),
            team_groups: 0,
        }
    }
}

/// Represents a time slot or round in the tournament.
#[derive(Clone, Debug)]
pub struct Slot {
    /// Slot ID.
    pub id: i32,
    /// Name or label of the slot.
    pub name: String,
}

impl Slot {
    /// Creates a new Slot with default values.
    pub fn new() -> Slot {
        Slot {
            id: 0,
            name: "Null".to_string(),
        }
    }
}

/// Represents capacity constraints for the tournament.
#[derive(Clone, Debug)]
pub struct CapacityConstraints {
    /// Internal parameter.
    pub c_intp: i32,
    /// Maximum value.
    pub c_max: i32,
    /// Minimum value.
    pub c_min: i32,
    /// Mode type 1 (single char).
    pub c_mode1: char,
    /// Mode type 2 (string).
    pub c_mode2: String,
    /// Penalty value.
    pub c_penalty: i32,
    /// First team group affected.
    pub c_team_groups1: i32,
    /// Second team group affected.
    pub c_team_groups2: i32,
    /// Type of constraint.
    pub c_type: String,
}
impl CapacityConstraints {
    /// Creates a new CapacityConstraints instance with default values.
    pub fn new() -> CapacityConstraints {
        CapacityConstraints {
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
#[derive(Clone, Debug)]
pub struct SeparationConstraints {
    /// Maximum value.
    pub c_max: i32,
    /// Minimum value.
    pub c_min: i32,
    /// Penalty value.
    pub c_penalty: i32,
    /// Team group affected by the constraint.
    pub c_team_groups: i32,
    /// Type of constraint.
    pub c_type: String,
}
impl SeparationConstraints {
    /// Creates a new SeparationConstraints instance with default values.
    pub fn new() -> SeparationConstraints {
        SeparationConstraints {
            c_max: 0,
            c_min: 0,
            c_penalty: 0,
            c_team_groups: 0,
            c_type: "Null".to_string(),
        }
    }
}

