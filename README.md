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

We implemented a **constructive method based on the Circle Method** (Kirkman, 1847) to generate many valid initial schedules for TTP. The algorithm runs in \(O(n)\) time for \(n\) teams, and by adjusting parameters, it can produce up to \(2n \cdot n!\) different schedules. The main goal is practical: **provide metaheuristics with a large pool of feasible solutions right from the start**.

---

## Problem Overview

The **Traveling Tournament Problem (TTP)** is about scheduling a double round-robin tournament for an even number of teams, each team plays every other team twice (home and away), while respecting three main rules:

1. **Maximum Streak (`maxStreak`)**: No team plays more than *n* consecutive home or away games.  
2. **No Repeats (`noRepeat`)**: Teams cannot face the same opponent in consecutive rounds.  
3. **Double Round-Robin (`doubleRoundRobin`)**: Each team plays only one game per round.

---

## Implementation Details

### Methodology

We adapted the **Circle Method** for single round-robin tournaments to ensure all constraints are met. By tweaking parameters, the method can produce many valid schedules systematically, which is useful for initializing evolutionary algorithms.

### Time Complexity

- \(O(n)\) for generating a single valid schedule.  
- Up to \(2n \cdot n!\) different schedules through parameter variation.

---

## Technical Stack

| Component | Description |
|------------|-------------|
| **Language** | Rust |
| **Core Algorithm** | ---------- |
| **Libraries** | `----`, `----`, `----` |
| **Usage Context** | Providing initial population for metaheuristics |

---

## Getting Started

### Prerequisites

