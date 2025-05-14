# land-rs

This project uses cargo workspaces to combine two seperate crates

```
.
├── Cargo.lock
├── Cargo.toml
├── crates
│   ├── gfold_rs                        // Convex optimisation algorithm
│   └── land_sim                        // Simulation of a rocket landing
├── patches
│   └── good_lp+1.13.0.patch            // Patch to add SOCP support to good_lp
├── docs/                               // Documentation for the project written in typst
├── README.md
└── rust-toolchain.toml 
```

To run this project, you need to have the following dependencies installed: 
- Rust & Cargo (with the nightly toolchain)
- Typst (for documentation)

## Installation

To apply the patch to the good_lp crate:
```bash 
cargo patch-crate
```


## Usage
To run the simulation:

```bash
cargo run --release
```

To generate the graphs:

```bash
cargo run --package gfold-rs --example plotting 
```
