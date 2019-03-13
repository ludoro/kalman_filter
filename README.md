## Introduction


## Files

High level overview of files:

* main.rs : you can find an example of the working code.
* lib.rs : contains the unit-tests.
* surface.rs : contains the declaration of points, planes and rectangles structs.
* functions.rs : contains translation and rotation functions, checking functions




## How to use + plus requirements

Compiler: rustc 1.33.0
Cargo: 1.33.0

Dependencies:
nalgebra = "0.17.2"
approx = "0.3.1"
num-traits = "0.2"

The code is run using cargo.
The "project" is created running cargo new #project_name --bin

In terminal:

For testing: cargo test
For building: cargo build
For running: cargo run

If maximum performance is required, it is best to use the flag --release when building.

Compilation will take more time but the code will run faster.

## Things to improve
Should have used more generics functions, but I wanted to keep it simple for a
first implementation.
