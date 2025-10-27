use crate::data_set::Rawdata;
use std::collections::HashMap;
use std::fs::File;
use log::{info};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

/// Represents a solution to the Traveling Tournament Problem (TTP).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Solution {
    /// A matrix representing the schedule of games:
    /// - rows: slots (rounds)
    /// - col: teams
    pub solution: Vec<Vec<Game>>,
}

/// Represents a single game in the schedule.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    /// Indicates whether the game is at home (true) or away (false)
    pub home_game: bool,
    /// Opponent team ID
    pub opponent: i32,
}

impl Solution {
    /// Generates the traveling distance matrix for all teams.
    ///
    /// # Arguments
    /// * `data` - A reference to `Rawdata` containing teams and distance information.
    ///
    /// # Returns
    /// A 2D vector where `matrix[i][j]` represents the distance from team i to team j.
    pub fn generate_traveling_distance_matrix(data: &Rawdata) -> Vec<Vec<i32>> {
        let mut traveling_distance_matrix = vec![vec![0i32; data.teams.len()]; data.teams.len()];

        for distance in &data.distances {
            traveling_distance_matrix[distance.team1 as usize][distance.team2 as usize] = distance.dist;
        }

        traveling_distance_matrix
    }

    /// Generates an initial schedule using a circle round-robin approach twice.
    ///
    /// # Arguments
    /// * `data` - A reference to `Rawdata`.
    ///
    /// # Returns
    /// A `Solution` representing a feasible initial schedule.
    pub fn generate_initial_solution(data: &Rawdata) -> Solution {
        let mut solution_matrix = Solution {
            solution: vec![
                vec![
                    Game { home_game: false, opponent: -1 };
                    data.teams.len()
                ];
                data.slots.len()
            ],
        };

        let num_slots = 2 * (data.teams.len() - 1);
        let mut teams: Vec<usize> = (0..data.teams.len()).collect();

        // First half of the round-robin schedule
        for round in 0..(data.teams.len() - 1) {
            for i in 0..(data.teams.len() / 2) {
                let team_a = teams[i];
                let team_b = teams[data.teams.len() - 1 - i];
                solution_matrix.solution[round][team_a] = Game { home_game: true, opponent: team_b as i32 };
                solution_matrix.solution[round][team_b] = Game { home_game: false, opponent: team_a as i32 };
            }
            teams[1..].rotate_right(1);
        }

        // Second half of the round-robin schedule (reverse home/away)
        for round in (data.teams.len() - 1)..num_slots {
            for i in 0..(data.teams.len() / 2) {
                let team_a = teams[i];
                let team_b = teams[data.teams.len() - 1 - i];
                solution_matrix.solution[round][team_a] = Game { home_game: false, opponent: team_b as i32 };
                solution_matrix.solution[round][team_b] = Game { home_game: true, opponent: team_a as i32 };
            }
            teams[1..].rotate_right(1);
        }

        solution_matrix
    }

    pub fn generate_all_solutions(data: &Rawdata,traveling_distance_matrix: &Vec<Vec<i32>>, path: &str) -> Vec<Solution>{

        let mut solutions: Vec<Solution> = Vec::new();

        let teams = &data.teams;
        let mut id_solution = 0;
        for direction in [true, false]  {
            for fixed_team in 0..data.teams.len() {
                for perm in teams.iter().permutations(teams.len()) {
                    id_solution = id_solution + 1;
                    let ids: Vec<i32> = perm.iter().map(|team| team.id).collect();
                    println!("perm: {:?}", ids);
                    let mut temporary_rawdata = data.clone();
                    temporary_rawdata.teams = perm.into_iter().cloned().collect();
                    let temporary_solution = Solution::generate_florian_solution(&temporary_rawdata, fixed_team, direction);
                    let (distance, cap_constraints, sep_constraints, round_robin_respect) = Solution::evaluate_solution(&data,&traveling_distance_matrix,&temporary_solution);
                    info!("Solution: \n{}Distance: {},\nCapacity Constraints: {},\nSeparation Constraints: {}, \nRound Robin Respect: {}", Solution::solution_to_string(&temporary_solution, &data),distance,cap_constraints,sep_constraints,round_robin_respect);
                    solutions.push(temporary_solution);
                    let file = File::create(format!("{}/solutions_{}.json", path, id_solution)).unwrap();
                    serde_json::to_writer_pretty(file, &solutions).unwrap();
                }
            }
        }

        solutions
    }

    /// # Description
    /// Generates a solution using Florian's strategy with alternating home/away games.
    ///
    /// # Arguments
    /// * `data` - Reference to the problem instance (`Rawdata`).
    /// * `fixed_team` - Index of the team that is fixed during rotations.
    /// * `upward` - Boolean flag controlling the alternation of rotations.
    ///
    /// # Returns
    /// A [`Solution`] representing a valid initial double round-robin schedule,
    /// without any constraints.
    ///
    /// # Example
    /// let data = load_instance("example.dat");
    /// let sol = Solution::generate_florian_solution(&data, 0, true);
    /// Solution::print_solution(&sol, &data);
    pub fn generate_florian_solution(data: &Rawdata, fixed_team: usize, upward: bool) -> Solution {

        info!(
            "Starting Florian's construction for {} teams | Fixed team: {} | Pattern: {}",
            data.teams.len(),
            fixed_team,
            if upward { "Upward direction" } else { "Downward direction" }
        );

        let mut solution_matrix = Solution {
            solution: vec![
                vec![
                    Game { home_game: false, opponent: -1 };
                    data.teams.len()
                ];
                data.slots.len()
            ],
        };

        let mut teams: Vec<usize> = data
            .teams
            .iter()
            .enumerate()
            .map(|(_, team)| team.id as usize)
            .collect();


        let fixed_team = teams.remove(fixed_team);
        teams.push(fixed_team);

        for round in 0..2 * (data.teams.len() - 1) {
            info!("Round: {}", round);
            info!("Teams before rotation: {:?}", teams);
            for i in 0..(data.teams.len() / 2) {
                let team_a = teams[i];
                let team_b = teams[data.teams.len() - 1 - i];
                let home_first = (round % 2 == 0) == upward;

                if home_first {
                    solution_matrix.solution[round][team_a] = Game { home_game: true, opponent: team_b as i32 };
                    solution_matrix.solution[round][team_b] = Game { home_game: false, opponent: team_a as i32 };
                } else {
                    solution_matrix.solution[round][team_a] = Game { home_game: false, opponent: team_b as i32 };
                    solution_matrix.solution[round][team_b] = Game { home_game: true, opponent: team_a as i32 };
                }
                info!(
                "Pairing: Team {} vs Team {} | {} is home",
                team_a,
                team_b,
                if home_first { team_a } else { team_b }
                );
            }
            let fixed_team = teams.remove(teams.len()-1);
            teams.rotate_right(1);
            teams.push(fixed_team);
            info!("Teams after rotation: {:?}", teams);
        }

        info!("Final solution for {} teams | Fixed team: {} | Pattern: {}",
            data.teams.len(),
            fixed_team,
            if upward { "Upward direction" } else { "Downward direction" });

        solution_matrix
    }

    /// Converts the schedule into a readable string format.
    ///
    /// # Arguments
    /// * `solution_matrix` - A reference to a [`Solution`] containing the schedule.
    /// * `data` - A reference to [`Rawdata`] providing team names and IDs.
    ///
    /// # Returns
    /// A [`String`] representing the formatted schedule.
    pub fn solution_to_string(solution_matrix: &Solution, data: &Rawdata) -> String {
        let mut output = String::new();

        output.push_str(&format!("{:>8}", ""));
        for team_id in 0..data.teams.len() {
            output.push_str(&format!("{:>8}", format!("{}:{}", data.teams[team_id].name, data.teams[team_id].id)));
        }
        output.push('\n');

        for (slot_id, row) in solution_matrix.solution.iter().enumerate() {
            output.push_str(&format!("{:>8}", format!("Slot:{}", slot_id)));
            for game in row {
                output.push_str(&format!("{:>8}", format!("{}{}", game.opponent, if game.home_game { "H" } else { "A" })));
            }
            output.push('\n');
        }

        output
    }

    /// Checks all constraints for the provided solution.
    ///
    /// # Arguments
    /// * `data` - Reference to `Rawdata` containing constraints.
    /// * `solution_matrix` - Reference to `Solution` to check.
    ///
    /// # Returns
    /// Tuple containing:
    /// 1. Number of capacity constraint violations
    /// 2. Number of separation constraint violations
    /// 3. Boolean indicating if round-robin constraints are respected
    pub fn check_constraints(data : &Rawdata, solution_matrix : &Solution) -> (i32,i32,bool) {

        let num_slots = solution_matrix.solution.len();
        let num_teams = solution_matrix.solution[0].len();
        let mut capacity_constraints = 0;
        let mut separation_constraints = 0;
        let mut round_robin_respect = true;

        // Capacity Constraints:

        for constraint in &data.capacity_constraints {
            for team in 0..num_teams {
                for start_slot in 0..=num_slots - constraint.c_intp as usize {
                    let count = solution_matrix.solution[start_slot..start_slot + constraint.c_intp as usize]
                        .iter()
                        .filter(|slot| {
                            let game = &slot[team];
                            match constraint.c_mode1 {
                                'H' => game.home_game,
                                'A' => !game.home_game,
                                _ => false,
                            }
                        })
                        .count();

                    if count < constraint.c_min as usize || count > constraint.c_max as usize {
                        capacity_constraints += 1;
                    }
                }
            }
        }

        // Separation Constraints:

        for constraint in &data.separation_constraints {

            for team in 0..num_teams {

                let mut last_slot_vs: Vec<Option<usize>> = vec![None; num_teams];

                for slot in 0..num_slots {
                    let game = &solution_matrix.solution[slot][team];
                    let opponent = game.opponent as usize;

                    if let Some(last) = last_slot_vs[opponent] {
                        let distance = slot - last;

                        if distance <= constraint.c_min as usize || distance > constraint.c_max as usize {
                            separation_constraints += 1;
                        }
                    }

                    last_slot_vs[opponent] = Some(slot);
                }
            }
        }

        // Round robin constraints

        let mut match_count: HashMap<(usize, usize), i32> = HashMap::new();

        for slot in 0..num_slots {
            for home_team in 0..num_teams {
                let away_team = solution_matrix.solution[slot][home_team].opponent;

                let key = if home_team < away_team as usize {
                    (home_team, away_team as usize)
                } else {
                    (away_team as usize, home_team)
                };

                *match_count.entry(key).or_insert(0) += 1;
            }
        }

        for ((_, _), count) in &match_count {
            if *count > 4 {
                round_robin_respect = false;
            }
        }

        (capacity_constraints, separation_constraints, round_robin_respect)
    }

    /// Evaluates the total traveling distance for a solution.
    ///
    /// # Arguments
    /// * `traveling_distance_matrix` - Matrix of distances between teams.
    /// * `solution_matrix` - Reference to `Solution`.
    ///
    /// # Returns
    /// Total traveling distance as `i32`.
    pub fn evaluate_objective(traveling_distance_matrix : &Vec<Vec<i32>>, solution_matrix : &Solution) -> i32{

        let num_slots = solution_matrix.solution.len();
        let num_teams = solution_matrix.solution[0].len();
        let mut total_distance = 0;

        for team in 0..num_teams {
            for slot in 0..num_slots {
                let game = &solution_matrix.solution[slot][team];
                if !game.home_game && game.opponent != -1 {
                    let opponent = game.opponent as usize;
                    total_distance += traveling_distance_matrix[team][opponent];
                }
            }
        }

        total_distance
    }

    /// Evaluates the complete solution including constraints and objective function.
    ///
    /// # Arguments
    /// * `data` - Reference to `Rawdata` containing teams and constraints.
    /// * `traveling_distance_matrix` - Computed traveling distance matrix.
    /// * `solution_matrix` - Reference to `Solution`.
    ///
    /// # Returns
    /// Tuple containing:
    /// 1. Objective value (total distance)
    /// 2. Capacity constraint violations
    /// 3. Separation constraint violations
    /// 4. Boolean indicating if round-robin constraints are respected
    pub fn evaluate_solution(
        data: &Rawdata,
        traveling_distance_matrix: &Vec<Vec<i32>>,
        solution_matrix: &Solution,
    ) -> (i32, i32, i32, bool) {
        let (cap_constraints, sep_constraints, round_robin_respect) = Self::check_constraints(data, solution_matrix);
        let result = Self::evaluate_objective(traveling_distance_matrix, solution_matrix);
        (result, cap_constraints, sep_constraints, round_robin_respect)
    }
}
