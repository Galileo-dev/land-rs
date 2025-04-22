= Abstract

This Final Year Project investigates use of advanced convex trajectory optimisation strategies for use in landing @rlv. Percise landing capabilities is essential for lowering launch costs, increasing mission success for missions such as moon/mars landings and for increasing launch cadence, yet it is a very challenging problem with various nonlinearities such as aerodynamic drag, thrust bounds and free-final time while minimising fuel consumption.

This project adopts a novel approach using successive convexification to solve the @apdg problem and evaluates it against state-of-the-art @rl trajectory optimisation approaches and evaluates the performance trade-offs between the two approaches in both robustness and computational complexity

All software was developed end-to-end in Rust, a memory-safe, high-performance programming language designed to provide memory safety guarantees, showcasing Rust as an ideal language for mission-critical avionics systems.

A bespoke @6dof simulation environment was developed in Rust, providing a high-fidelity yet real-time simulation of a @rlv landing.

A custom @socp modeling library was developed and patched into an existing @lp modeling library for more human-readable models. We also developed a raw implementation of @apdg however this used in the final comparison as it lacks the successive convexification that is key to the @apdg algorithm.

Methodologically, the project investigates the powered decesent problem as a descrete-time optimal control problem with path and terminal constraints. @apdg is implemented as a convex-optimisation problem which employs linearisation through taylor series expansion and trust-region safeguarding to ensure convexity.

//! Todo(): Add summary of the conclusion
