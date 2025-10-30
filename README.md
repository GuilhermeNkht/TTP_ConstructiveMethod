# A Constructive Method to Build Many Valid Initial Solutions for the Traveling Tournament Problem

**Authors:**  
Guilherme Nakahata (AIST, University of Tsukuba)  
Florian Richoux (AIST, Japan)  
Daan van den Berg (Vrije Universiteit Amsterdam, Netherlands)  
Claus Aranha (University of Tsukuba, Japan)

---

## Abstract

This repository contains the code and materials for our paper:  
**“A Constructive Method to Build Many Valid Initial Solutions for the Traveling Tournament Problem (TTP)”**.

We implemented a **constructive method based on the Circle Method** (Kirkman, 1847) to generate many valid initial schedules for the TTP.  
The algorithm runs in O(n) time for n teams.  
By adjusting parameters, it can produce up to 2n · n! different schedules.  

The main goal is to provide metaheuristics with a large pool of feasible solutions from the start.

---

## Problem Overview

The Traveling Tournament Problem (TTP) schedules a double round-robin tournament for an even number of teams, where each team plays every other team twice (home and away), while respecting the following constraints:

1. Maximum Streak (maxStreak) – No team plays more than n consecutive home or away games.  
2. No Repeats (noRepeat) – Teams cannot face the same opponent in consecutive rounds.  
3. Double Round-Robin (doubleRoundRobin) – Each team plays only one game per round.

---

## Features

- Efficient generation of multiple valid schedules
- Distance computation and statistical analysis
- Save and load solutions/permutations in JSON format
- Easy integration with metaheuristic initialization pipelines
- Optional visualizations (distance histograms)
- Fully reproducible (fixed random seeds)

---

## Technical Stack

| Component | Description |
|------------|-------------|
| **Language** | Rust |
| **Core Algorithm** | Constructive scheduling via Circle Method |
| **Libraries** | |
|  • roxmltree = "0.20.0" | XML parsing |
|  • log = "0.4.28" | Logging framework |
|  • env_logger = "0.11.8" | Environment-based logger |
|  • chrono = "0.4.42" | Date and time utilities |
|  • itertools = "0.14.0" | Advanced iteration/combinatorics tools |
|  • serde = "1.0.228" | Serialization and deserialization |
|  • serde_json = "1.0.145" | JSON handling |
|  • indicatif = "0.9.0" | Progress bars for CLI |
|  • plotters = "0.3.7" | Plotting and visualization |
|  • rand = "0.9.2" | Random number generation |

---

## Getting Started

### Prerequisites

- Rust 1.70+
- cargo for building and running

### Installation

    git clone <repository_url>
    cd ttpgen
    cargo build --release

---

## Running

After building, run the executable with:

    cargo run --release --package ttpgen --bin ttpgen -- [OPTIONS]

### Command-line Options


TODO:
//--input <file> : Path to the XML instance file  
//--output-solutions <folder> : Directory to save generated solutions  
//--output-permutations <folder> : Directory to save generated permutations  
//--permutations <n> : Number of random team permutations to generate  
//--seed <n> : Random seed for reproducibility  
//--no-save : Disable saving solutions to disk  
//--log <true|false> : Enable or disable logging

---

### Example

TODO:
//cargo run --release --package ttpgen --bin ttpgen --   --input NL12.xml   --output-solutions output --output-permutations output --permutations 10   --seed 42
//This command generates 10 random solutions using the constructive method, evaluates their total travel distances, and saves them to the output/ folder.

---

## Features Summary

- Multiple random permutations: Generate different solutions for the same instance.  
- Travel distance evaluation: Computes total travel distance for each schedule.  
- Statistics: Mean, median, variance, standard deviation, min/max, quartiles.  
- Plotting: Create histograms of travel distances.  
- Logging: Optional detailed logs for analysis.  
- JSON Output: Solutions and permutations are reproducible and portable.

---

## Example Usage in Code

let raw_data_set : Rawdata = XmlManager::read_xml("NL8.xml");

let traveling_distance_matrix = Solution::generate_traveling_distance_matrix(&raw_data_set);

let permutations = Solution::generate_random_permutations(&raw_data_set,10000,2025,"permutations");

let (_, distances) = Solution::generate_all_solutions(&raw_data_set, &traveling_distance_matrix, permutations,"solutions");

Statistics::generate_statistics(&distances);

---

## Citation

If you use this code or ideas from this work, please cite this work (to be added).

---

## License

This project is distributed under the XXXXX License.  
See the LICENSE file for details.
