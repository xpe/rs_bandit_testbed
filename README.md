# Multi-Armed Bandit Test Bed

This code reproduces Section 2.3 from "Reinforcement Learning: An Introduction" by Sutton & Barto, Second Edition. I am grateful to Richard Sutton for sharing his knowledge of reinforcement learning with such a wonderful textbook.

## Components

This code has two main parts:

* Experiment-running and data generation: written in Rust
* Data visualization: written in Python

## Running

Use the three scripts in 'bin' to generate three figures:

* Figure 2.2: `bin/fig_2_2`
* Figure 2.4: `bin/fig_2_4`
* Figure 2.5: `bin/fig_2_5`

Each script will output files to the 'results' directory.

## Setup

I recommend creating a Conda environment named 'plot' with the correct dependencies:

```sh
conda create -n plot python=3
conda activate plot
conda install matplotlib numpy pandas
```
