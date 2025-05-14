= Technology Stack
This chapter describes the various technologies used throughout this project, focusing on the reason for their selection and how they were used. Special priority was given to libraries such as Rust in memory-safe languages.

== Rust

Rust was chosen as a language that offers high-level features with low-level speeds, provides memory safety without a garbage collector and does not allow undefined behaviour @rust_reference_undefined_behavior, making it ideal for safety-critical applications such as running the guidance system of a rocket @Pinho2019.

Usually, problems like the one addressed in this paper would be implemented in languages like Python or C. However, safety-critical systems require heavy verification efforts to ensure safety and correctness. Rust combines a better developer experience with strong safety guarantees and has many modern convenience features. Rust solves many common programming errors through its ownership model and borrow checker. These replace the traditional garbage collector or manual memory management seen in C, without any runtime overhead, as this is all done at compile time. These models also prevent data races and eliminate memory leaks, dangling pointers and array bound violations, all present in low-level languages like C.

The `rustc` compiler can detect many errors that would only show up at runtime for normal languages, therefore increasing reliability and reducing verification workloads, which is critical for rocket landing, as a software failure could mean the vehicle's and its payload loss.

Rust has best-in-class support for concurrent real-time programming, critical for advanced control systems like rockets. With many systems, the concurrency model is designed to ensure thread safety.

Rust's default package manager and build system is known as `cargo`, where libraries are called `crates`.

== Clarabel
Clarabel is a state-of-the-art interior-point solver for convex conic optimisation problems in Rust @Clarabel_2024. This solver is a pure Rust implementation, unlike other solvers available, which utilise importing libraries developed in other unsafe languages, defeating the point of Rust.

Clarabel can solve @lp:pl, @qp:pl and @socp:pl problems. For this project, we are utilising its @socp solver. Clarabel is also the default solver for CVXPY (the most popular Python library for modelling and solving convex optimisation problems), most other solvers are either slow or heavily commercialised. Meanwhile, Clarabel is free, open source, faster, and more robust.

== Patching Good LP

Clarabel does not offer a modelling language, as CVXPY would be used as a high-level interface for Clarabel. However, because of this project's focus, a modelling language was not initially used, and instead, the problem was approached by solving directly in the solver's specification format. Due to the complexity of this problem, this did not last long, as trying to fix issues in the reformulated and decomposed equations was increasingly complex. A bespoke patch was developed and applied to a linear programming library called good_lp, which offers a variety of ways to model and solve linear programming problems; however, it lacked the support for the second-order cone constraint required by the algorithm's implementation @Szmuk2016.

A patch-crate library was used to patch the `good_lp` crate to support the second-order cone constraints.
The general form of the problem is as follows:

$ min c^T x $

$ "Subject to" quad A x = b, quad x in K $

Where:

- $x$ is the decision variable (i.e position, velocity, mass and thrust).
- $c^T x$ is the objective function
- $A x = b$ represents the equality constraints (i.e. vehicle dynamics, fuel mass depletion, glide slope constraints, thrust constraints).
- $K$ represents @soc:pl cone constraints that enforce the thrust direction, minimum glide slope, and acceleration limits.

The @soc:pl are defined as:
$ K^n_S = { v in R^n | v_1 ≥ || v_(2:n) || } $

By applying the patch, I was able to define my constraints in a much more readable format with fewer lines of code required:

==== Before:
```rust
// Velocity dynamics (Equation 66)
  for k in 0..N - 1 {
      for i in 0..3 {
 // Original:
 //      v[k+1] = v[k] + 1/2 * (a[k] + a[k+1]) * dt
 // Rearranged:
 //      v[k+1] - v[k] - (1/2) * a[k] * dt - (1/2) * a[k+1] * dt = 0
          eq!(
              &[
                  (idx_v(k + 1, i), 1.0),
                  (idx_v(k, i), -1.0),
                  (idx_a(k, i), -dt / 2.0),
                  (idx_a(k + 1, i), -dt / 2.0),
              ],
              0.0
          );
      }
  }
```

==== After:
```rust
// Velocity dynamics
// v[k+1] = v[k] + 1/2 * (a[k] + a[k+1]) * dt
for i in 0..3 {
 model.add_constraint(constraint!(
        vars.steps[k + 1].v[i]
            == vars.steps[k].v[i]
                + 0.5 * (vars.steps[k].a[i] + vars.steps[k + 1].a[i]) * settings.dt
    ));
}
```


== Bevy Game Engine
The Bevy game engine is a game engine built in Rust. It is a data-driven engine that utilises entity component systems (ECS) to manage game state. This allows for a better separation of concerns and adds many features and systems while keeping the codebase maintainable. It also allows for easy extensibility and modularity @Anderson2024bevyengine.

=== Bevy ECS
Bevy @ecs is a critical part of the Bevy game engine. It is a robust framework that allows users to separate data (Components), logic (Systems), and entities (Entity IDS) for efficient game logic implementation. Some of the key features of Bevy @ecs are:
- *Entity*: A unique identifier for an object in the game world (e.g. a player, a rocket, a landing pad).
- *Component*: A piece of data attached to entities, defining their properties (e.g. a rocket's position and velocity, rocket rigidbody).
- *System*: This logic manipulates the entities with the specific components (e.g. a system that updates the rocket engine thrust depending on the keyboard input).

#include "../code/ecs.typ"

== Plotters
A pure Rust library for plotting data. This was used after a trajectory was generated to visualise all the data produced by the @sc algorithm and the final output trajectory in 3d.

== Rapier Physics Engine
The Rapier physics engine is built in Rust. It supports 2D and 3d physics (we will use the 3d physics engine to implement a @6dof rocket simulation). Most relevant to this project, Rapier supports joint constraints, collision detection, and rigid body dynamics. This is important as we must simulate the rocket's flight trajectory and engine thrust and control @Crozet2024dimforge.

== Git
Git is an easy-to-use version control system that allows for keeping track of changes to the codebase. It was used in combination with GitHub to allow for easy syncing and cloud backups of the codebase during development. It has many useful features for collaboration, but due to this individual project, these features were not used and will not be discussed here.

== Typst
This document was written using Typist, a markup-based typesetting system. It has many of the same LaTeX features but is much more user-friendly and easier to learn than LaTeX. It also has a rich plugin ecosystem that allows easy customisation, such as the code blocks and maths information boxes used in this document. It has speedy PDF compilation thanks to its pure Rust implementation.

== Conclusion
The technology stack was chosen to deliver high performance, strong safety guarantees, and a modular architecture allowing easy extensibility. Laying a foundational framework for the future development of safety-critical systems.
