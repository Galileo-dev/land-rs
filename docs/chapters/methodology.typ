= Methodology

== Approach
- Implement the G-FOLD algorithm using the Rust programming language with Clarabel.rs for solving convex optimisation problems. This allows for real-time performance and maintaining memory safety, vital for onboard computations during rocket descent.

- Use the Bevy game engine for its ECS (Entity Component System) @Anderson2024bevyengine and Rapier physics engine @Crozet2024dimforge to develop a model of real-world physics; this will involve a fundamental rocket entity that can be controlled through various inputs such as nozzle angle, thrust and reaction control thrusters while providing altitude, displacement and velocity metrics to the guidance algorithms.

- Incorporate atmospheric variables into the simulation, such as wind shear, air density fluctuations, and drag, to test the algorithm's capabilities and robustness.

- Define performance metrics such as landing accuracy, computational load and fuel efficiency to determine the effectiveness of each algorithm.

== Ethical and Practical Considerations
- This project involves no human participants; hence, no ethics study is needed.

- Computational efficiency is essential as the algorithm must run in real-time during rocket descent and be corrected on the fly. This can be challenging for hardware-constrained systems like flight computers, which typically use radiation-hardened hardware much slower than today's processors.

- Create edge cases to test against, such as wind and potential mass shifts, to mimic real-world conditions and test reactions by different algorithms.

== Development Plan
1. Initial implementation of the G-FOLD algorithm, focusing on modular design and adherence to real-time constraints

2. Initial implementation of the simulation environment using Bevy for creating a basic wireframe visualisation for landing trajectories and Rapier for realistic physics simulations.

3. Integration of the G-FOLD algorithm with simulation environment to allow for testing against different scenarios.

4. Conduct a detailed comparative analysis between G-FOLD and the reinforcement learning approach on pre-defined parameters like fuel efficiency and adaptability.

== Extras
- Extensive documentation and reporting through a final year project report and code documentation.

- Implement a CI/CD pipeline to automate the testing and notify of potential regressions during pull requests.
