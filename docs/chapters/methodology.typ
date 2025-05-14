

= Methodology

This FYP aims to investigate the performance of advanced convex trajectory optimisation algorithms for @apdg and compare their performance and robustness to @drl algorithms that promise to offer more flexible and adaptive control policies, while being more computationally efficient.

== Research Approach
My research was focused on practical and iterative development, focusing on implementing an algorithm that can be used in real-time, on board the guidance computer of a rocket. I chose to implement a successive Convexification algorithm that offered a balance between performance and robustness. I found that not much preexisting code had been open-sourced, so I developed my own open-sourced bespoke library to implement @gfold using @sc described in @Szmuk2016.

A Rust library implementing @gfold was developed using Clarabel.rs to solve @soc constraints based on @Szmuk2016. These libraries target real-time performance under atmospheric conditions.

== Data Collection
Data was collected from both the algorithm and the simulation environment. The algorithm and environment generated data outputs that capture the key metrics. This data allowed for a detailed comparative analysis of the algorithms' outputs.
== Completed Tasks

- [x] Implement the @gfold algorithm based on "Successive Convexification for Fuel-Optimal Powered Landing with Aerodynamic Drag and Non-Convex Constraints" @Szmuk2016 using Rust with Clarabel.rs for solving the @soc constraints, ensuring real-time performance and memory safety, vital for onboard computations during vehicle descent.

- [x] Use the Bevy game engine for its ECS (Entity Component System) @Anderson2024bevyengine and Rapier physics engine @Crozet2024dimforge to develop a basic simulation environment; this will involve a fundamental rocket entity that can be controlled in real-time e.g. nozzle angle, thrust, pitch and yaw while providing altitude, velocity and acceleration metrics to the guidance algorithms.

- [x] Define performance metrics such as landing accuracy, computational efficiency, fuel efficiency and robustness to atmospheric disturbances, to evaluate the robustness of each algorithm.

- [x] Conducted initial research on the @gfold algorithm and convex optimisation by reviewing research papers. Stanford EE364A for a deeper dive @boyd_vandenberghe_ee364a.

- [x] Implement the simulation environment using Bevy to create a basic wireframe visualisation for landing trajectories and Rapier for realistic physics simulations.

- [x] Setup communication between the simulation environment and the algorithms to allow for real-time two-way communication while decoupling the simulation from the algorithms.

- [x] Added @socp to an existing Rust linear programming library to simplify modelling constraints.

- [x] Extensive documentation and reporting through a final year project report and code documentation.

- [x] Conducted a detailed comparative analysis between @gfold and the @rl approach on pre-defined parameters like fuel efficiency and adaptability.\

- [x] Generate trajectories based on the rocket settings and configuration inside the simulation environment.

- [x] Draw the generated reference trajectories inside the simulation environment to visualise for debugging and analysis.

- [ ] Integration of the @gfold algorithm with a simulation environment to allow for testing against different scenarios.

- [ ] Incorporate atmospheric variables into the simulation, such as wind shear, air density fluctuations, and drag, to test the algorithm's capabilities and robustness.
