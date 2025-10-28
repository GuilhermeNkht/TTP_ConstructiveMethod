use xml_manager::XmlManager;
use solution::Solution;
use crate::data_set::Rawdata;
use log::{info};
use crate::statistics::Statistics;

mod xml_manager;
mod data_set;
mod solution;
mod logging;
mod statistics;

fn main() {

    logging::init_logger("log.txt", false);
    info!("Logger initialized");

    info!("Loading instance file");
    let raw_data_set : Rawdata = XmlManager::read_xml("NL12.xml");

    info!("Generating traveling distance matrix");
    let traveling_distance_matrix = Solution::generate_traveling_distance_matrix(&raw_data_set);

    // let (solutions, distances) = Solution::generate_all_solutions(&raw_data_set, &traveling_distance_matrix,"solutions");

    let loaded = Solution::load_solutions("solutions/");
    println!("{}", Solution::has_duplicate_solutions(&loaded));

    let sum = Solution::generate_distances(loaded, &raw_data_set, &traveling_distance_matrix);

    let total: i128 = sum.iter().sum();

    println!("Sum: {}", total);

    Statistics::generate_statistics(&sum);


    // let initial_solution = Solution::generate_initial_solution(&raw_data_set);
    // let (result, cap_constraints, sep_constraints, round_robin_respect) = Solution::evaluate_solution(&raw_data_set,&traveling_distance_matrix,&initial_solution);

    // info!("\n{}", Solution::solution_to_string(&initial_solution, &raw_data_set));
    // info!("Distance: {}", result);
    // info!("Constraints: {}, {}, {}", cap_constraints, sep_constraints, round_robin_respect);

    // let florian_solution = Solution::generate_florian_solution(&raw_data_set,0, true);
    // let (result_florian, cap_constraints_florian, sep_constraints_florian, round_robin_respect_florian) = Solution::evaluate_solution(&raw_data_set,&traveling_distance_matrix,&florian_solution);

    // info!("\n{}", Solution::solution_to_string(&florian_solution, &raw_data_set));
    // info!("Distance: {}", result_florian);
    // info!("Constraints: {}, {}, {}", cap_constraints_florian, sep_constraints_florian,round_robin_respect_florian);

    info!("Framework execution completed");

}
