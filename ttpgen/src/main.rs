// External crates
use log::info;
use clap::Parser;

// Local modules / crates
use crate::data_set::Rawdata;
use crate::statistics::Statistics;
use cli::Cli;
use solution::Solution;
use xml_manager::XmlManager;

mod xml_manager;
mod data_set;
mod solution;
mod logging;
mod statistics;
mod cli;

fn main() {

    let args = Cli::parse();

    logging::init_logger("log.txt", args.log_enabled);
    info!("Logger initialized");

    info!("{:?}", args);

    info!("Loading instance file");
    let raw_data_set : Rawdata = XmlManager::read_xml(&*args.input);

    info!("Generating traveling distance matrix");
    let traveling_distance_matrix = Solution::generate_traveling_distance_matrix(&raw_data_set);

    info!("Generating permutations");
    let permutations = Solution::generate_random_permutations(&raw_data_set,args.permutations,args.seed,&*args.output_permutations, args.save);

    info!("Generating solutions");
    let (_, distances) = Solution::generate_all_solutions(&raw_data_set, &traveling_distance_matrix, permutations,&*args.output_solutions, args.save);

    Statistics::generate_statistics(&distances);

    info!("Framework execution completed");

}
