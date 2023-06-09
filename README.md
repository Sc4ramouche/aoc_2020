# [Advent of Code 2020](https://adventofcode.com/2020/)

Solving puzzles, learning Rust.

## Learnings

### Rust modules

It took me some time to understand how to properly split the code into modules in Rust. Those two articles were helpful:

- [Rust Module System Explained](https://aloso.github.io/2021/03/28/module-system.html)
- [Rust: Project structure example step by step](https://dev.to/ghost/rust-project-structure-example-step-by-step-3ee)

### Hash Set

- Day first puzzle resembles the [Two Sum problem](https://leetcode.com/problems/two-sum/). At first I implemented it using the `HashMap`, then I realised I only really need the key. Which lead me to discovering the `HashSet` collection type.
