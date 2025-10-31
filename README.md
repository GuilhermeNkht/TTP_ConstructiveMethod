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

<div align="center">
<b>Example:</b><br>
<img src="ttpgen/images/ttp.png" alt="Traveling Tournament Problem (TTP)" width="400"/><br>
<i>Figure 1:</i> Left: A valid 4-team TTP tournament. Right: An invalid TTP tournament, with a `noRepeat` violation in column 2, a `maxStreak = 3` violation in column 4, and `doubleRoundRobin` violations in rounds (rows) 2, 3, and 4.
</div>

---

## Features

- Efficient generation of multiple valid schedules.
- Distance computation and statistical analysis.
- Save and load solutions/permutations in JSON format.
- Easy integration with metaheuristic initialization pipelines.
- Optional visualizations (distance histograms).
- Fully reproducible (fixed random seeds).

---

## Technical Stack

| Component | Description                               |
|------------|-------------------------------------------|
| **Language** | Rust                                      |
| **Core Algorithm** | Constructive scheduling via Circle Method |
| **Libraries** |                                           |
|  • roxmltree = "0.20.0" | XML parsing                               |
|  • log = "0.4.28" | Logging framework                         |
|  • env_logger = "0.11.8" | Environment-based logger                  |
|  • chrono = "0.4.42" | Date and time utilities                   |
|  • itertools = "0.14.0" | Advanced iteration/combinatorics tools    |
|  • serde = "1.0.228" | Serialization and deserialization         |
|  • serde_json = "1.0.145" | JSON handling                             |
|  • indicatif = "0.9.0" | Progress bars for CLI                     |
|  • plotters = "0.3.7" | Plotting and visualization                |
|  • rand = "0.9.2" | Random number generation                  |
|  • clap = "4.5.51" | Cli Interface / Comand line argument      |

---

## Getting Started

### Prerequisites

- Rust 1.70+
- cargo for building and running
- librust-yeslogic-fontconfig-sys-dev

### Installation

    git clone <https://github.com/GuilhermeNkht/TTP_ConstructiveMethod>
    cd ttpgen
    cargo build --release

---

## Running

After building, run the executable with:

    cargo run --release --package ttpgen --bin ttpgen -- [OPTIONS]

### Command-line Options

--input <file> : Path to the XML instance file  
--output-solutions <folder> : Directory to save generated solutions  
--output-permutations <folder> : Directory to save generated permutations  
--permutations <n> : Number of random team permutations to generate  
--seed <n> : Random seed for reproducibility  
--log : Enable or disable logging
--save : Enable or disable saving to disk  

---

### Example

cargo run --release --package ttpgen --bin ttpgen --     --input NL12.xml     --output-solutions solutions     --output-permutations permutations     --permutations 10     --seed 31     --log     --save

<div align="center">
<b>Output Example for NL8:</b><br><br>

<b>1. Example of permutation (JSON):</b>  
[outputs/permutations/perms.json](ttpgen/permutations/permutation.json)<br><br>

<b>2. Example of solution (JSON):</b>  
[outputs/solutions/solutions_1.json](ttpgen/solutions/solution_1.json)<br><br>

<b>3. Example of final print in log:</b>  
[outputs/log.txt](ttpgen/log.txt)<br><br>

<b>4. Example of final histogram:</b>  
[outputs/dist_histogram.png](ttpgen/images/dist_histogram.png)<br>
</div>



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
