// Std library
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::hash::{Hash};
use std::io::BufReader;

// External crates
use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

// Local modules
use crate::data_set::{Rawdata, Team};

/// Saves any serializable data to a json file.
///
/// This function serializes the provided data structure into a json format
/// and writes it to the specified file path. It works with any type
/// that implements `serde::Serialize`.
///
/// # Arguments
/// * `data` - A reference to the data to serialize and save.
/// * `path` - A string slice specifying the file path.
///
/// # Returns
/// A `Result` indicating success (`Ok(())`) or failure (`Err`) with an I/O error.
///
/// # Example
/// ```
/// use serde::Serialize;
/// #[derive(Serialize)]
/// struct Example {
///     id: u32,
///     name: String,
/// }
///
/// let data = Example { id: 1, name: "Test".to_string() };
/// save_to_file(&data, "output/example.json").expect("Failed to save file");
/// ```
pub fn save_to_file<T: Serialize>(data: &T, path: &str) -> std::io::Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, data)?;
    Ok(())
}

/// Represents a single match/game between two teams.
///
/// The `Game` struct stores the home/away status and the opponent's ID.
///
/// # Fields
/// * `home_game` - A boolean indicating if the team is playing at home (`true`) or away (`false`).
/// * `opponent` - The ID of the opponent team.
///
/// # Example
/// ```
/// let match_game = Game {
///     home_game: true,
///     opponent: 5,
/// };
/// println!("Home game: {}, Opponent: {}", match_game.home_game, match_game.opponent);
/// ```
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Game {
    pub home_game: bool,
    pub opponent: i32,
}

/// Represents a set of generated team permutations along with metadata.
///
/// `Permutations` stores multiple random permutations of team IDs,
/// along with the seed used for generation and the instance name. This is useful
/// for saving and later reusing or analyzing the generated permutations.
///
/// # Fields
/// * `seed` - The seed used for generating the permutations.
/// * `instance_name` - The name of the problem instance.
/// * `permutations` - A vector of vectors, where each inner vector represents one permutation of team IDs.
///
/// # Example
/// ```
/// let perms = Permutations {
///     seed: 42,
///     instance_name: "instance_01".to_string(),
///     permutations: vec![
///         vec![0,1,2,3],
///         vec![3,2,1,0],
///     ],
/// };
/// ```
#[derive(Serialize)]
pub struct Permutations {
    pub seed: u64,
    pub instance_name: String,
    pub permutations: Vec<Vec<i32>>,
}

/// A simple wrapper around `ProgressBar` for logging progress.
///
/// # Example
/// ```
/// let progress = ProgressBarLog::new(100);
/// for i in 0..100 {
///     progress.set_message(&format!("Processing item {}", i));
///     progress.inc();
/// }
/// progress.finish();
/// ```
pub struct ProgressBarLog {
    bar: ProgressBar,
}

/// A simple wrapper around `ProgressBar` for logging progress.
///
/// `ProgressBarLog` provides a convenient interface to create and manipulate a progress bar
/// with custom styling and logging messages. This is useful for tracking long-running
/// operations, like generating or evaluating multiple scheduling solutions.
///
/// # Example
/// ```
/// let progress = ProgressBarLog::new(100);
/// for i in 0..100 {
///     progress.set_message(&format!("Processing item {}", i));
///     progress.inc();
/// }
/// progress.finish();
/// ```
impl ProgressBarLog {
    /// Creates a new `ProgressBarLog` with the given total count.
    ///
    /// # Arguments
    /// * `total` - The total number of steps to complete.
    ///
    /// # Returns
    /// A `ProgressBarLog` instance with custom styling.
    pub fn new(total: u64) -> Self {
        let bar = ProgressBar::new(total);
        bar.set_style(
            ProgressStyle::default_bar()
                .template(
                    " [{elapsed_precise}] {bar:40.green/white} {pos}/{len} ({percent}%) | {msg}",
                )
                .progress_chars("%>="),
        );
        Self { bar }
    }

    /// Increments the progress bar by one step.
    pub fn inc(&self) {
        self.bar.inc(1);
    }

    #[allow(dead_code)]
    /// Finishes the progress bar, marking it as complete.
    pub fn finish(&self) {
        self.bar.finish();
    }

    #[allow(dead_code)]
    /// Sets a custom message to display alongside the progress bar.
    ///
    /// # Arguments
    /// * `msg` - The message string to display.
    pub fn set_message(&self, msg: &str) {
        self.bar.set_message(msg);
    }
}

/// Represents a solution for the scheduling problem.
///
/// Each `Solution` contains an `id` identifying the solution,
/// and a `solution` matrix where `solution[slot][team]`
/// stores a `Game` indicating the opponent and if the game is home or away.
///
/// # Fields
/// * `id` - A unique identifier for the solution.
/// * `solution` - A 2D vector of `Game` instances representing the schedule matrix.
///
/// # Example
/// ```
/// let solution = Solution {
///     id: 1,
///     solution: vec![vec![Game { home_game: true, opponent: 2 }]],
/// };
/// println!("Solution ID: {}", solution.id);
/// ```
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Solution {
    pub id: i32,
    pub solution: Vec<Vec<Game>>,
}

impl Solution {

    /// Creates a new, empty `Solution` instance initialized with default game values.
    ///
    /// This function constructs a solution matrix where each slot and team position
    /// is filled with a `Game`:
    /// - `home_game` is set to `false`
    /// - `opponent` is set to `-1` (indicating: no opponent assigned yet)
    ///
    /// # Arguments
    /// * `data` - A reference to the `Rawdata` structure containing teams and slots
    ///   information. The size of the solution matrix is derived from:
    ///   - `data.teams.len()`
    ///   - `data.slots.len()`
    ///
    /// # Returns
    /// A `Solution` struct with:
    /// - `id` initialized to `-1`
    /// - `solution` initialized as a `slots x teams` matrix filled with default games.
    ///
    /// # Example
    /// ```
    /// let data = Rawdata::generate_example();
    /// let solution = Solution::new(&data);
    /// assert_eq!(solution.solution.len(), data.slots.len());
    /// assert_eq!(solution.solution[0].len(), data.teams.len());
    /// ```
    pub fn new(data: &Rawdata) -> Solution {
        Solution {
            id: -1,
            solution: vec![
                vec![
                    Game {
                        home_game: false,
                        opponent: -1
                    };
                    data.teams.len()
                ];
                data.slots.len()
            ],
        }
    }

    /// Generates a traveling distance matrix based on the distance in `Rawdata`.
    ///
    /// This function constructs a 2D matrix where each cell `(i, j)` represents the
    /// distance traveled from team `i` to team `j`. The matrix is initialized with
    /// zeros and populated using the `distances` list contained inside `Rawdata`.
    ///
    /// # Arguments
    /// * `data` - A reference to the `Rawdata` structure containing team distance
    ///   relationships. `data.distances` is expected to list distances between pairs
    ///   of teams.
    ///
    /// # Returns
    /// A 2D vector (`Vec<Vec<i32>>`) where:
    /// - The row index corresponds to the origin team
    /// - The column index corresponds to the destination team
    /// - Each cell contains the travel distance between them
    ///
    /// # Example
    /// ```
    /// let data = Rawdata::generate_example();
    /// let distance_matrix = generate_traveling_distance_matrix(&data);
    ///
    /// println!("Distance: {}", distance_matrix[0][2]);
    /// ```
    pub fn generate_traveling_distance_matrix(data: &Rawdata) -> Vec<Vec<i32>> {
        let mut traveling_distance_matrix = vec![vec![0i32; data.teams.len()]; data.teams.len()];

        for distance in &data.distances {
            traveling_distance_matrix[distance.team1 as usize][distance.team2 as usize] =
                distance.dist;
        }

        traveling_distance_matrix
    }

    #[allow(dead_code)]
    /// Checks if a list of `Solution` objects contains duplicates.
    ///
    /// This function iterates over all solutions and attempts to insert each one into a
    /// `HashSet`. If insertion fails for any solution, it means a duplicate exists,
    /// and the function returns `true`.
    ///
    /// # Arguments
    /// * `solutions` - A reference to a vector of `Solution` instances to be checked.
    ///
    /// # Returns
    /// * `true` if one or more duplicates are found.
    /// * `false` if all solutions are unique.
    ///
    /// # Requirements
    /// The `Solution` type must implement:
    /// - `Hash`
    /// - `Eq`
    ///
    /// # Example
    /// ```
    /// let solutions = load_solutions("output/solutions/");
    /// if has_duplicate_solutions(&solutions) {
    ///     println!("Duplicate.");
    /// } else {
    ///     println!("No duplicates.");
    /// }
    /// ```
    pub fn has_duplicate_solutions(solutions: &Vec<Solution>) -> bool {
        let mut seen = HashSet::new();

        for sol in solutions {
            if !seen.insert(sol) {
                return true;
            }
        }
        false
    }

    #[allow(dead_code)]
    /// Loads all solution files from a directory and returns them as a vector of `Solution`.
    ///
    /// This function scans the directory for files whose names follow the pattern
    /// `solutions_*.json`. Each file is opened, deserialized into a `Solution`,
    /// and collected into a vector. After loading, the solutions are sorted in ascending
    /// order based on their `id` field.
    ///
    /// # Arguments
    /// * `path` - A string slice representing the directory to search for solution files.
    ///
    /// # Returns
    /// A vector of `Solution` objects loaded from the directory.
    ///
    /// # Panics
    /// This function will panic if:
    /// - The directory cannot be read.
    /// - A file cannot be opened.
    /// - A JSON file cannot be deserialized into a `Solution`.
    ///
    /// # Example
    /// ```
    /// let solutions = load_solutions("output/solutions/");
    /// println!("Loaded {} solutions", solutions.len());
    ///
    /// if let Some(first) = solutions.first() {
    ///     println!("First solution ID: {}", first.id);
    /// }
    /// ```
    pub fn load_solutions(path: &str) -> Vec<Solution> {
        let mut all_solutions = Vec::new();

        let entries = fs::read_dir(path).expect("Error opening directory");

        for entry in entries {
            let entry = entry.expect("Error at path");
            let path = entry.path();

            if path.is_file() {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    if filename.starts_with("solutions_") && filename.ends_with(".json") {
                        let file = File::open(&path).expect("Error opening file");
                        let reader = BufReader::new(file);

                        let solution: Solution =
                            from_reader(reader).expect("Error deserializing JSON");

                        all_solutions.push(solution);
                    }
                }
            }
        }

        all_solutions.sort_by_key(|s| s.id);
        all_solutions
    }

    #[allow(dead_code)]
    /// Calculates the total traveling distances for a list of solutions.
    ///
    /// This function iterates over each solution, evaluates it using the provided
    /// traveling distance matrix, and collects the total distances into a vector.
    ///
    /// # Arguments
    /// * `solutions` - A vector of `Solution` instances to evaluate.
    /// * `data` - A reference to the `Rawdata` containing teams and constraints.
    /// * `traveling_distance_matrix` - A reference to a 2D vector where `matrix[i][j]` represents
    ///   the distance from team `i` to team `j`.
    ///
    /// # Returns
    /// A vector of `i128` where each element represents the total traveling distance
    /// of the corresponding solution.
    ///
    /// # Example
    /// ```
    /// let data = Rawdata::generate_example();
    /// let distance_matrix = vec![vec![0,5,7], vec![5,0,3], vec![7,3,0]];
    /// let solutions = vec![Solution::generate_example(), Solution::generate_example()];
    /// let distances = generate_distances(solutions, &data, &distance_matrix);
    /// println!("All distances: {:?}", distances);
    /// ```
    pub fn generate_distances(
        solutions: Vec<Solution>,
        data: &Rawdata,
        traveling_distance_matrix: &Vec<Vec<i32>>,
    ) -> Vec<i128> {
        let mut all_distances: Vec<i128> = Vec::new();

        for solution in solutions {
            let (distance, _, _, _) =
                Solution::evaluate_solution(data, traveling_distance_matrix, &solution);

            all_distances.push(distance as i128);
        }

        all_distances
    }

    /// Logs a solution's schedule and its evaluation metrics.
    ///
    /// This function prints a representation of the solution,
    /// including the total traveling distance, capacity, round-robin and separation
    /// constraint violations, It also returns the total distance.
    ///
    /// # Arguments
    /// * `solution` - A reference to the `Solution` to log.
    /// * `data` - A reference to the `Rawdata` containing teams and constraints.
    /// * `traveling_distance_matrix` - A reference to a 2D vector where `matrix[i][j]` represents
    ///   the distance from team `i` to team `j`.
    ///
    /// # Returns
    /// The total traveling distance (`i32`) of the solution.
    ///
    /// # Example
    /// ```
    /// let data = Rawdata::generate_example();
    /// let solution = Solution::generate_example();
    /// let distance = Solution::log_solution(&solution, &data, &vec![vec![0,5,7], vec![5,0,3], vec![7,3,0]]);
    /// println!("Total distance: {}", distance);
    /// ```
    fn log_solution(
        solution: &Solution,
        data: &Rawdata,
        traveling_distance_matrix: &Vec<Vec<i32>>,
    ) -> i32 {
        let (distance, cap_constraints, sep_constraints, round_robin_respect) =
            Solution::evaluate_solution(data, traveling_distance_matrix, solution);

        let solution_str = Solution::solution_to_string(solution, data);
        info!(
            "Solution:\n{}\nDistance: {}\nCapacity Constraints: {}\nSeparation Constraints: {}\nRound Robin Respect: {}",
            solution_str, distance, cap_constraints, sep_constraints, round_robin_respect
        );

        distance
    }

    /// Generates a complete solution for a given team permutation using Florian's method.
    ///
    /// This function clones the input `Rawdata`, applies the given team permutation, and
    /// generates a round-robin schedule using `generate_florian_solution`. The resulting
    /// solution is assigned the provided ID.
    ///
    /// # Arguments
    /// * `data` - A reference to the `Rawdata` containing the original teams, traveling_distance_matrix and constraints.
    /// * `perm` - A reference to a vector of `Team` representing the ordered permutation of teams.
    /// * `fixed_team` - The index of the team to remain fixed during the method rotations.
    /// * `upward` - If `true`, the home/away pattern follows an upward direction, otherwise downward.
    /// * `id` - The unique ID to assign to the generated solution.
    ///
    /// # Returns
    /// A `Solution` struct representing the generated schedule with the specified ID.
    ///
    /// # Example
    /// ```
    /// let data = Rawdata::generate_example();
    /// let perm = data.teams.clone();
    /// let solution = generate_solution(&data, &perm, 0, true, 1);
    /// println!("{}", solution_to_string(&solution, &data));
    /// ```
    fn generate_solution(
        data: &Rawdata,
        perm: &Vec<Team>,
        fixed_team: usize,
        upward: bool,
        id: i32,
    ) -> Solution {
        let mut temporary_data = data.clone();
        temporary_data.teams = perm.clone();
        let mut solution = Solution::generate_florian_solution(&temporary_data, fixed_team, upward);
        solution.id = id;

        solution
    }

    /// Generates a set of unique random permutations of the team IDs.
    ///
    /// This function takes the list of teams from `Rawdata` and generates the requested number of
    /// unique permutations. Each permutation is randomized and stored in a `Vec<i32>`.
    ///
    /// # Arguments
    /// * `data` - A reference to the `Rawdata` struct containing the list of teams.
    /// * `number_permutation` - A reference to an `i32` specifying how many unique permutations
    ///   should be generated.
    ///
    /// # Returns
    /// A vector of vectors (`Vec<Vec<i32>>`), where each inner vector is a unique permutation
    /// of the team IDs.
    ///
    /// # Example
    /// ```
    /// let data = Rawdata::generate_example();
    /// let permutations = generate_random_permutations(&data, &5);
    /// ```
    pub fn generate_random_permutations(
        data: &Rawdata,
        number_permutations: i32,
        seed: u64,
        path: &str, save: bool,
    ) -> Vec<Vec<i32>> {
        let team_ids: Vec<i32> = data.teams.iter().map(|t| t.id).collect();

        let mut rng = StdRng::seed_from_u64(seed);
        let mut permutations: HashSet<Vec<i32>> = HashSet::new();

        while permutations.len() < number_permutations as usize {
            let mut perm = team_ids.clone();
            perm.shuffle(&mut rng);
            permutations.insert(perm);
        }

        let vec_perm: Vec<Vec<i32>> = permutations.into_iter().collect();

        if save {
            let permutations_to_save = Permutations {
                seed,
                instance_name: data.instance_name.clone(),
                permutations: vec_perm.clone(),
            };
            save_to_file(&permutations_to_save, &format!("{}/permutation.json", path)).unwrap();
        }

        vec_perm
    }

    /// Generates all possible solutions for a given team permutation using Florian's method,
    /// evaluates their distances, and optionally saves them to disk.
    ///
    /// This function iterates over all possible combinations of fixed teams and home/away patterns
    /// (upward/downward) for a given permutation of teams. Each generated solution is evaluated
    /// using the traveling distance matrix, logged, and optionally saved as JSON.
    ///
    /// # Arguments
    /// * `data` - A reference to the `Rawdata` containing teams, slots, and constraints.
    /// * `traveling_distance_matrix` - A reference to a 2D vector where `matrix[i][j]` represents
    ///   the distance from team `i` to team `j`.
    /// * `permutation` - A vector of vect of team IDs representing the order in which teams are considered.
    /// * `path` - A string slice representing the directory path where solutions will be saved if `SAVE_ENABLED` is true.
    ///
    /// # Returns
    /// A tuple `(solutions, all_distances)`:
    /// - `solutions` (Vec<Solution>): all generated solution matrices.
    /// - `all_distances` (Vec<i128>): total traveling distance for each solution.
    ///
    /// # Panics
    /// This function may panic if saving a solution to file fails.
    ///
    /// # Example
    /// ```
    /// let data = Rawdata::generate_example();
    /// let distance_matrix = vec![vec![0,5,7], vec![5,0,3], vec![7,3,0]];
    /// let permutation = vec![0,1,2];
    /// let (solutions, distances) = generate_all_solutions(&data, &distance_matrix, permutation, "output");
    /// println!("Solutions length {}", solutions.len());
    /// println!("Distances: {:?}", distances);
    /// ```
    pub fn generate_all_solutions(
        data: &Rawdata,
        traveling_distance_matrix: &Vec<Vec<i32>>,
        permutation: Vec<Vec<i32>>,
        path: &str,
        save: bool,
    ) -> (Vec<Solution>, Vec<i128>) {
        let mut solutions: Vec<Solution> = Vec::new();
        let mut all_distances: Vec<i128> = Vec::new();

        let mut id_solution = 0;

        let total_perms = 2 * data.teams.len() * permutation.len();

        // Create progress bar
        let progress = ProgressBarLog::new(total_perms as u64);

        for team in permutation {
            let teams_ordered: Vec<Team> = team
                .iter()
                .filter_map(|id| data.teams.iter().find(|t| t.id == *id))
                .cloned()
                .collect();

            // Log the permutation
            info!("Permutation: {:?}", team);

            for direction in [true, false] {
                for fixed_team in 0..data.teams.len() {
                    id_solution = id_solution + 1;

                    // Generate solution
                    let temporary_solution = Solution::generate_solution(
                        &data,
                        &teams_ordered,
                        fixed_team,
                        direction,
                        id_solution,
                    );

                    // Log solution details
                    let distance_solution = Solution::log_solution(
                        &temporary_solution,
                        &data,
                        &traveling_distance_matrix,
                    );

                    // Store the solution and the distance
                    solutions.push(temporary_solution.clone());
                    all_distances.push(distance_solution as i128);

                    // Save to file
                    if save {
                        save_to_file(
                            &temporary_solution,
                            &format!("{}/solution_{}.json", path, id_solution),
                        )
                        .unwrap();
                    }

                    // Update bar inc
                    progress.inc();
                }
            }
        }

        (solutions, all_distances)
    }

    /// Generates a schedule using Florian's method construction.
    ///
    /// This function constructs a round-robin schedule fixing a team. The `upward`
    /// flag determines the pattern of home/away assignments for the first match
    /// of each pairing.
    ///
    /// # Arguments
    /// * `data` - A reference to `Rawdata` containing team information.
    /// * `fixed_team` - The index of the team to remain fixed during rotations.
    /// * `upward` - If `true`, the home team assignment follows an upward pattern; otherwise downward.
    ///
    /// # Returns
    /// A `Solution` struct with the scheduled matches for all slots and teams.
    ///
    /// # Example
    /// ```
    /// let data = Rawdata::generate_example();
    /// let solution = generate_florian_solution(&data, 0, true);
    /// println!("{}", solution_to_string(&solution, &data));
    /// ```
    pub fn generate_florian_solution(data: &Rawdata, fixed_team: usize, upward: bool) -> Solution {
        info!(
            "Starting Florian's construction for {} teams | Fixed team: {} | Pattern: {}",
            data.teams.len(),
            fixed_team,
            if upward {
                "Upward direction"
            } else {
                "Downward direction"
            }
        );

        let mut solution_matrix = Solution::new(&data);

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
                    solution_matrix.solution[round][team_a] = Game {
                        home_game: true,
                        opponent: team_b as i32,
                    };
                    solution_matrix.solution[round][team_b] = Game {
                        home_game: false,
                        opponent: team_a as i32,
                    };
                } else {
                    solution_matrix.solution[round][team_a] = Game {
                        home_game: false,
                        opponent: team_b as i32,
                    };
                    solution_matrix.solution[round][team_b] = Game {
                        home_game: true,
                        opponent: team_a as i32,
                    };
                }

                info!(
                    "Pairing: Team {} vs Team {} | {} is home",
                    team_a,
                    team_b,
                    if home_first { team_a } else { team_b }
                );
            }

            let fixed_team = teams.remove(teams.len() - 1);
            teams.rotate_right(1);
            teams.push(fixed_team);
            info!("Teams after rotation: {:?}", teams);
        }

        info!(
            "Final solution for {} teams | Fixed team: {} | Pattern: {}",
            data.teams.len(),
            fixed_team,
            if upward {
                "Upward direction"
            } else {
                "Downward direction"
            }
        );

        solution_matrix
    }

    /// Converts a `Solution` matrix into a formatted string representation.
    ///
    /// This function generates a human-readable string showing the schedule of all teams
    /// for each slot. Each cell shows the opponent ID followed by `H` for a home game or
    /// `A` for an away game. The output also includes team names and IDs as headers.
    ///
    /// # Arguments
    /// * `solution_matrix` - A reference to the `Solution` containing the schedule.
    /// * `data` - A reference to the `Rawdata` struct containing team information.
    ///
    /// # Returns
    /// A `String` representing the formatted solution.
    ///
    /// # Example
    /// ```
    /// let data = Rawdata::generate_example();
    /// let solution = Solution::generate_example();
    /// let output_str = solution_to_string(&solution, &data);
    /// println!("{}", output_str);
    /// ```
    /// Example output:
    /// ```text
    /// Id: 1
    ///          ATL:0    NYM:1    PHI:2
    /// Slot:0    1H       2A       0H
    /// Slot:1    2H       0A       1H
    /// ```
    pub fn solution_to_string(solution_matrix: &Solution, data: &Rawdata) -> String {
        let mut output = String::new();
        output.push_str(&format!("Id: {}\n", solution_matrix.id));

        output.push_str(&format!("{:>8}", ""));
        for team_id in 0..data.teams.len() {
            output.push_str(&format!(
                "{:>8}",
                format!("{}:{}", data.teams[team_id].name, data.teams[team_id].id)
            ));
        }
        output.push('\n');

        for (slot_id, row) in solution_matrix.solution.iter().enumerate() {
            output.push_str(&format!("{:>8}", format!("Slot:{}", slot_id)));
            for game in row {
                output.push_str(&format!(
                    "{:>8}",
                    format!(
                        "{}{}",
                        game.opponent,
                        if game.home_game { "H" } else { "A" }
                    )
                ));
            }
            output.push('\n');
        }

        output
    }

    /// Checks all constraints for a solution, including capacity, separation, and round-robin.
    ///
    /// 1. **Capacity constraints**: Verifies for each team, within the specified interval (`c_intp`)
    ///    of consecutive slots, the number of home or away games falls within
    ///    the minimum (`c_min`) and maximum (`c_max`) allowed.
    ///
    /// 2. **Separation constraints**: Ensures that matches between two teams respect the minimum and maximum
    ///    separation distances defined by each constraint.
    ///
    /// 3. **Round-robin constraints**: Checks that no pair of teams plays against each other more than 4 times (2 pairs of game).
    ///
    /// # Arguments
    /// * `data` - A reference to the `Rawdata` containing teams and constraints.
    /// * `solution_matrix` - A reference to the `Solution` with the scheduled games.
    ///
    /// # Returns
    /// A tuple `(capacity_violations, separation_violations, round_robin_respected)`
    /// - `capacity_violations` (i32): total number of capacity constraint violations.
    /// - `separation_violations` (i32): total number of separation constraint violations.
    /// - `round_robin_respected` (bool): true if all pairs of teams respect the round-robin.
    ///
    /// # Example
    /// ```
    /// let data = Rawdata::generate_example();
    /// let solution = Solution::generate_example();
    /// let (cap_viol, sep_viol, rr_ok) = check_constraints(&data, &solution);
    /// println!("Capacity violations: {}, Separation violations: {}, Round-robin ok: {}", cap_viol, sep_viol, rr_ok);
    /// ```
    fn check_constraints(data: &Rawdata, solution_matrix: &Solution) -> (i32, i32, bool) {
        let num_slots = solution_matrix.solution.len();
        let num_teams = solution_matrix.solution[0].len();
        let mut capacity_constraints = 0;
        let mut separation_constraints = 0;
        let mut round_robin_respect = true;

        // Capacity Constraints:

        for constraint in &data.capacity_constraints {
            for team in 0..num_teams {
                for start_slot in 0..=num_slots - constraint.c_intp as usize {
                    let count = solution_matrix.solution
                        [start_slot..start_slot + constraint.c_intp as usize]
                        .iter()
                        .filter(|slot| {
                            let game = &slot[team];
                            match constraint.c_mode1 {
                                'A' => game.home_game,
                                'H' => !game.home_game,
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

                        if distance <= constraint.c_min as usize
                            || distance > constraint.c_max as usize
                        {
                            separation_constraints += 1;
                        }
                    }

                    last_slot_vs[opponent] = Some(slot);
                }
            }
        }

        // Round-robin constraints

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

        (
            capacity_constraints,
            separation_constraints,
            round_robin_respect,
        )
    }

    /// Calculates the total traveling distance for all teams in a given solution.
    ///
    /// This function iterates over all teams and all slots in the solution. For each team,
    /// it tracks the current location and adds the distance to the next game location.
    /// Home games do not require traveling, while away games add the distance to the opponent's location.
    ///
    /// # Arguments
    /// * `traveling_distance_matrix` - A reference to a 2D vector where `matrix[i][j]` represents
    ///   the distance from team `i` to team `j`.
    /// * `solution_matrix` - A reference to the `Solution` containing the schedule of games
    ///   for all slots and teams.
    ///
    /// # Returns
    /// The total traveling distance for all teams (i32).
    ///
    /// # Example
    /// ```
    /// let distance_matrix = vec![vec![0, 5, 7], vec![5, 0, 3], vec![7, 3, 0]];
    /// let total = evaluate_objective(&distance_matrix, &solution);
    /// println!("Total traveling distance: {}", total);
    /// ```
    fn evaluate_objective(
        traveling_distance_matrix: &Vec<Vec<i32>>,
        solution_matrix: &Solution,
    ) -> i32 {
        let num_slots = solution_matrix.solution.len();
        let num_teams = solution_matrix.solution[0].len();
        let mut total_distance = 0;

        for team in 0..num_teams {
            let mut current_location = team;
            for slot in 0..num_slots {
                let game = &solution_matrix.solution[slot][team];
                let next_location = if game.home_game {
                    team
                } else {
                    game.opponent as usize
                };
                total_distance += traveling_distance_matrix[current_location][next_location];
                current_location = next_location;
            }
        }

        total_distance
    }

    /// Evaluates a given solution by calculating the total traveling distance and checking constraints.
    ///
    /// This function combines the distance evaluation and constraint checks for a solution.
    /// It returns the total traveling distance, the total violations of capacity constraints,
    /// the total violations of separation constraints, and a boolean indicating if the
    /// round-robin structure is respected.
    ///
    /// # Arguments
    /// * `data` - A reference to the `Rawdata` struct containing teams, slots, and constraints.
    /// * `traveling_distance_matrix` - A reference to a 2D vector where `matrix[i][j]` represents
    ///   the distance from team `i` to team `j`.
    /// * `solution_matrix` - A reference to the `Solution` containing the schedule of games
    ///   for all slots and teams.
    ///
    /// # Returns
    /// A tuple `(total_distance, capacity_violations, separation_violations, round_robin_respected)`
    /// - `total_distance` (i32): total traveling distance for all teams.
    /// - `capacity_violations` (i32): total penalty for capacity constraints violations.
    /// - `separation_violations` (i32): total penalty for separation constraints violations.
    /// - `round_robin_respected` (bool): true if the round-robin structure is respected.
    ///
    /// # Example
    /// ```
    /// let data = Rawdata::generate_example();
    /// let distance_matrix = vec![vec![0,5,7], vec![5,0,3], vec![7,3,0]];
    /// let solution = Solution::generate_example();
    /// let (total_distance, cap_viol, sep_viol, rr_ok) = evaluate_solution(&data, &distance_matrix, &solution);
    /// ```
    pub fn evaluate_solution(
        data: &Rawdata,
        traveling_distance_matrix: &Vec<Vec<i32>>,
        solution_matrix: &Solution,
    ) -> (i32, i32, i32, bool) {
        let (cap_constraints, sep_constraints, round_robin_respect) =
            Self::check_constraints(data, solution_matrix);
        let result = Self::evaluate_objective(traveling_distance_matrix, solution_matrix);
        (
            result,
            cap_constraints,
            sep_constraints,
            round_robin_respect,
        )
    }
}
