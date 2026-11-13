# land-rs

Convex-optimisation rocket powered-landing guidance (APDG / successive
convexification) with a 6-DoF Bevy/Rapier flight simulation.

```
.
├── Cargo.toml
├── crates
│   ├── gfold_rs                        // Convex optimisation algorithm
│   └── land_sim                        // Simulation of a rocket landing
├── patches
│   └── good_lp+1.13.0.patch            // Patch to add SOCP support to good_lp
├── docs/                               // Documentation for the project written in typst
├── flake.nix                           // Nix devshell + docs package
├── rust-toolchain.toml                 // Pinned nightly toolchain
└── README.md
```

## Requirements

- Nix with flakes (`nix-command`, `flakes` experimental features), **or**
- Rust nightly (see `rust-toolchain.toml`), `cargo patch-crate`, and Typst.

The flake devshell provides the pinned nightly toolchain, `cargo patch-crate`,
Typst, rust-analyzer and the system libraries Bevy needs (Wayland/X11, Vulkan,
ALSA), and applies the good_lp patch automatically on first entry.

```bash
nix develop
```

## Installation

The good_lp crate is patched to add second-order-cone constraint support
(`soc_constraint!`). Inside the devshell this is automatic; otherwise run:

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

To build the thesis document:

```bash
nix build .#docs   # produces result/land-rs.pdf
```
