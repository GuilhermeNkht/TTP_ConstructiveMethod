use xml_manager::XmlManager;
use solution::Solution;
use crate::data_set::Rawdata;

mod xml_manager;
mod data_set;
mod solution;
mod logging;



fn main() {

    logging::init_logger("log.txt");

    let raw_data_set : Rawdata = XmlManager::read_xml("NL4.xml");
    let traveling_distance_matrix = Solution::generate_traveling_distance_matrix(&raw_data_set);

    let initial_solution = Solution::generate_initial_solution(&raw_data_set);
    let (result, cap_constraints, sep_constraints, round_robin_respect) = Solution::evaluate_solution(&raw_data_set,&traveling_distance_matrix,&initial_solution);

    println!("{}", Solution::solution_to_string(&initial_solution, &raw_data_set));

    println!("Distance: {}", result);
    println!("Constraints: {}, {}, {}", cap_constraints, sep_constraints, round_robin_respect);

    let florian_solution = Solution::generate_florian_solution(&raw_data_set,0, true);
    let (result_florian, cap_constraints_florian, sep_constraints_florian, round_robin_respect_florian) = Solution::evaluate_solution(&raw_data_set,&traveling_distance_matrix,&florian_solution);

    println!("{}", Solution::solution_to_string(&florian_solution, &raw_data_set));

    println!("Distance: {}", result_florian);
    println!("Constraints: {}, {}, {}", cap_constraints_florian, sep_constraints_florian,round_robin_respect_florian);

    Solution::generate_all_solutions(&raw_data_set, &traveling_distance_matrix,"Solutions");

}
