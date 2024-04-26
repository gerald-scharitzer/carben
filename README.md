# Carben

Get the carbon intensity of energy systems.

# Rationale

The availability of low-carbon energy varies in space and time.
Varying the production and consumption of energy in space or time changes the total emissions.
Get the carbon intensity of the constituents to optimize the carbon intensity of the system.
Finding a configuration with the lowest emissions is an optimization problem.
If the system is linear, then it can be optimized with linear programming.
More complex systems might require heuristics or quantum computers.
Electric grid nodes can vary their power production (positive sign) and/or consumption (negative sign).
The constraint is production + consumption = 0.
The control system must be fast enough compared to the dynamic of the system to effectively minimize the actual emissions.

# Develop

1. Clone with `git clone https://github.com/gerald-scharitzer/carben.git`
2. Enter with `cd carben`
3. Build with `cargo build`
4. Test with `cargo test [--test name] [-- --nocapture]`
5. Run with `cargo run [-- args]`
6. Document with `cargo doc [--open]`
7. Clean with `cargo clean`
8. Build release with `cargo build --release`

# License

This package is licensed according to the BSD license and its dependencies as follows.

reqwest: tbd
serde: tbd
