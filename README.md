# Introduction

## Library
Matrix operations can get nasty very quickly, that is why I decided to use the
library "nalgebra" for matrix related stuff.

For the entries of the rotation matrix, I took heavy inspiration from here:
https://math.stackexchange.com/questions/1167717/transform-a-plane-to-the-xy-plane

(I could also have used the function .rotate() by nalgebra)

The inverse of the rotation matrix does not actually require any calculations
as it is just the transpose thanks to linear algebra theory.
The library has the .transpose() function too.

The choice of the library is something that should take more time because
there are a lot of them. However the majority are poorly documented, while nalgebra has
a really well structured documentation.

## Tollerance
I declared a global variable (static in rust) called EPS = 10^{-6} which is the
the tollerance used in the calculations.
When dealing with numerical equalities, it is important to use such a variable because
it will be hard to have perfect equality using f64 variables.


# Files

High level overview of files:

* main.rs : you can find an example of the working code.
* lib.rs : contains the unit-tests.
* surface.rs : contains the declaration for points, planes and rectangles structs.
* functions.rs : contains translation and rotation functions, checking functions.




# How to use + requirements

The code is run using cargo.
The "project" is created running cargo new #project_name --bin

In terminal:

For testing: cargo test

Compiler: rustc 1.33.0

Cargo: 1.33.0

Dependencies:

nalgebra = "0.17.2"

approx = "0.3.1"

num-traits = "0.2"

The dependencies must be listed in the cargo.toml file.
With the first build everything will be installed and compiled, pretty neat!



For building: cargo build

For running: cargo run

If maximum performance is required, it is best to use the flag --release when building.

Compilation will take more time but the code will run faster.

# Things to improve
I should have used more generics types, but I wanted to keep it simple for a
first implementation.
