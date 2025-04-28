

= Methodology

== Approach
I propose a modular Rust library implementing @gfold using Clarabel.rs for second-order cone solving and a python library implement a @rl approach using Pytorch. These libraries will target real-time performance under atmospheric conditions and be hosted on GitHub.

I've opted for Rust, a language that offers high-level features with low-level speeds, provides memory safety without a garbage collector and doesn't allow undefined behaviour @rust_reference_undefined_behavior, making it ideal for safety-critical applications such as running the guidance system of a spacecraft @Pinho2019.

- Implement the @gfold algorithm with optimisations from Fast Algorithm for Onboard Atmospheric Powered Descent Guidance @ChenYushu2023AFAf using Rust with Clarabel.rs as our convex solver. This will allow for real-time performance and memory safety, vital for onboard computations during rocket descent.

- Implement a @rl approach using Pytorch to compare the performance and adaptability with the @gfold algorithm.

- Use the Bevy game engine for its ECS (Entity Component System) @Anderson2024bevyengine and Rapier physics engine @Crozet2024dimforge to develop a basic simulation environment; this will involve a fundamental rocket entity that can be controlled through various inputs such as nozzle angle, thrust and reaction control thrusters while providing altitude, displacement and velocity metrics to the guidance algorithms.
  - Potentially add disturbances and noisy sensor data to the simulation environment to test real-world behaviour of the algorithms and sensor filtering techniques.

- Incorporate atmospheric variables into the simulation, such as wind shear, air density fluctuations, and drag, to test the algorithm's capabilities and robustness.

- Define performance metrics such as landing accuracy, computational load and fuel efficiency to determine the effectiveness of each algorithm.

== Ethical and Practical Considerations
- This project involves no human participants; hence, no ethics study is needed.

- Computational efficiency is essential as the algorithm must run in real-time during rocket descent and be corrected on the fly. This can be challenging for hardware-constrained systems like flight computers, which typically use radiation-hardened hardware much slower than today's processors.

- Create edge cases to test against, such as wind and potential mass shifts, to mimic real-world conditions and test reactions by different algorithms.

== Task Completed

- [x] Implementation of the simulation environment using Bevy for creating a basic wireframe visualisation for landing trajectories and Rapier for realistic physics simulations.

- [x] Implementation of the @gfold algorithm, focusing on a modular design and adherence to real-time constraints

- [x] Develop a communication protocol between the simulation environment and the algorithms to allow for real-time two-way communication while decoupling the simulation from the algorithms.

- [x] Add @socp to an existing Rust linear programming library to make modeling constraints easier.

- [ ] Advanced atmospheric simulation integration of the simulation environment to truly test the robustness of the algorithms against real-world

- [ ] Integration of the @gfold algorithm with simulation environment to allow for testing against different scenarios.

- [ ] Conduct a detailed comparative analysis between @gfold and the @rl approach on pre-defined parameters like fuel efficiency and adaptability.

- [x] Extensive documentation and reporting through a final year project report and code documentation.
