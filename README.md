# ❄️ Andrei's 2023 Advent of Code ❄️

[![Build Status](https://github.com/AndreiBarsan/2023-advent-of-code/actions/workflows/aoc-ci-build.yml/badge.svg)](https://github.com/AndreiBarsan/2023-advent-of-code/actions/workflows/aoc-ci-build.yml)

## Learning Goals

- [ ] Finish the first 12 days for Rust practice
- [ ] Use tch in at least 5 problems
- [ ] Dockerized CI with tch support for fun
- [ ] Actually understand Rust-y implementations of cyclic data structures

## Running the Code

The following instructions set up Torch support, albeit without GPU by default. They are geared towards Apple Silicon, though they should work OK on x86 as well.

 1. Set up [Cargo](https://doc.rust-lang.org/rust-by-example/cargo.html)
 2. Enable `nightly` Rust with `rustup`, since the project needs features currently only on nightly, like benchmarking support.
 3. Set up a Python Anaconda environment and activate it.
 4. Install PyTorch in this environment: `conda install pytorch::pytorch torchvision torchaudio -c pytorch`
    * Do not enable `LIBTORCH_USE_PYTORCH`.

 5. Now you can finally build and run problems:
```
cargo run --release --bin <XX_problem>
```

Special thanks to [this repo](https://github.com/ssoudan/tch-m1) for a simple example of running Torch and its Rust bindings!

## See Also

- [My Advent of Code 2022 Solutions](https://github.com/AndreiBarsan/2022-advent-of-code/)
- [My Advent of Code 2021 Solutions](https://github.com/AndreiBarsan/2021-advent-of-code/)
