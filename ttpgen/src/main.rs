// External crates
use log::info;

// Local modules / crates
use crate::data_set::Rawdata;
use crate::statistics::Statistics;
use solution::Solution;
use xml_manager::XmlManager;

mod xml_manager;
mod data_set;
mod solution;
mod logging;
mod statistics;

fn main() {

    logging::init_logger("log.txt");
    info!("Logger initialized");

    info!("Loading instance file");
    let raw_data_set : Rawdata = XmlManager::read_xml("NL8.xml");

    info!("Generating traveling distance matrix");
    let traveling_distance_matrix = Solution::generate_traveling_distance_matrix(&raw_data_set);

    info!("Generating permutations");
    let permutations = Solution::generate_random_permutations(&raw_data_set,10000,2025,"permutations");

    info!("Generating solutions");
    let (_, distances) = Solution::generate_all_solutions(&raw_data_set, &traveling_distance_matrix, permutations,"solutions");

    Statistics::generate_statistics(&distances);

    info!("Framework execution completed");

}
