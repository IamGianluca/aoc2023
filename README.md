# AOC 2023 Solutions

This repository contains my solutions to the [Advent Of Code 2023](https://adventofcode.com/2023) challenge. All solutions are written in Rust. 

# Installation

First, let's download the codebase:

```bash
git pull git@github.com:IamGianluca/aoc2023.git
```

We can build the software with all optimizations with:

```bash
cargo build --release
```

Then, we can run any of the binaries included in the project:

```bash
target/release/<binary_name>
```

# Directory Structure

The solutions for each day in the AOC 2023 challenge are in the `src/bin/` directory. Each day has its own `.rs` file. 

# Development

You can run all unit tests with:

```bash
cargo test
```

Or specify which binary you want to target:

```bash
cargo test --bin <binary_name>
```

To profile any of the binaries: 

```bash
perf record -g --call-graph dwarf target/release/<binary_name>
perf report
```
