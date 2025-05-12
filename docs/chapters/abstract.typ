= Abstract

This final year project investigates the use of advanced convex trajectory optimisation strategies in landing @rlv. Percise landing capabilities are essential for lowering launch costs, increasing mission success for moon/Mars landings, and increasing launch cadence. However, it is a challenging problem with various nonlinearities such as aerodynamic drag, thrust bounds and free-final time while minimising fuel consumption.

This project adopts a novel approach using @sc to solve the @apdg problem and evaluates it against state-of-the-art @rl trajectory optimisation approaches. Evaluating the performance trade-offs between the two approaches.

All software was developed end-to-end in Rust, a memory-safe, high-performance programming language designed to provide memory safety guarantees, showcasing Rust as an ideal language for mission-critical avionics systems.

A bespoke @6dof simulation environment was developed in Rust, providing a high-fidelity yet real-time simulation of a @rlv landing scenario.

A custom @socp modelling library was developed and patched into an existing @lp modelling library for more human-readable models. We also developed a raw implementation of @apdg; however, this is not used in the final comparison as it lacks the successive convexification key to the @sc @apdg algorithm.

Methodologically, the project investigates the powered decescent problem as a discrete-time optimal control problem with path and terminal constraints. @apdg is implemented as a convex-optimisation problem which employs linearisation through Taylor series expansion and trust-region safeguarding to ensure convexity. A series of tests was conducted to measure the robustness and computational complexity of the two approaches.

//! Todo(): Add a summary of the conclusion
Conclusion...
