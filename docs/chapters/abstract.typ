= Abstract

This final year project investigates the use of advanced trajectory optimisation strategies in landing @rlv:pl. An @rlv is a launch vehicle performing the same job as a standard rocket, getting a payload into a specific orbit or another planetary body. However, it's unique in its ability to land with precision safely. Precise landing capabilities are essential for lowering launch costs, increasing mission success for moon/Mars landings, and increasing launch cadence. However, it is a very challenging problem with many nonlinearities and uncertainties such as aerodynamic drag, thrust bounds and free-final time while minimising fuel consumption.

This project adopts a novel approach using @sc to solve the @apdg problem and evaluates it against state-of-the-art @rl trajectory optimisation approaches. Assess the performance trade-offs between the two methods.

All software was developed end-to-end in Rust, a memory-safe, high-performance, systems programming language designed to provide memory safety guarantees, showcasing Rust as an ideal language for mission-critical avionics systems.

A bespoke @6dof simulation environment was developed in Rust, providing a high-fidelity yet real-time simulation of a @rlv landing scenario, that simulates the rocket's physics, simple drag, and thrust models. The simulation is designed to be extensible and modular, allowing new models and features to be added easily due to its use of @ecs.

A custom @socp modelling library was developed and patched into an existing @lp modelling library for more human-readable models that could be automatically reformulated into the specification expected by the convex solver. A direct raw implementation was initially developed of @apdg; however, this is not used in the final comparison as it lacks the successive convexification key to the @sc @apdg algorithm.

The project investigates the powered descent problem as a discrete-time optimal control problem with path and terminal constraints. @apdg is implemented as a convex-optimisation problem which employs linearisation through Taylor series expansion and trust-region safeguarding to ensure convexity. A series of tests was conducted to measure the robustness and computational complexity of the two approaches.

Some of the key findings from this investigation show a trade-off between @sc and @drl; however, each method shows distinct advantages under different operational conditions.
