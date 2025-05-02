

= Methodology

== Introduction
The goal of this fyp is to investigate the use of advanced convex trajectory optimisation algorithms for the use of landing autonomous rockets with pin-point accuracy and compare to compare it's performance and robustness vs next-gen deep reinforcement learning algorithms that promise to offer more flexible and adaptive control policies.

== Research Approach
My research was focused on practical development of an algorithm that can be used in a real-time, onboard the guidence computer of a rocket. I choose to implement a sucessive Convexification algorithm that offered a balance between performance and robustness. I found that not my many prexisting code had been open-sourced and so I decided to develop my own bespoke library for the implementation of @gfold.

a Rust library implementing @gfold was developed using Clarabel.rs for solviing @soc constraints base on @Szmuk2016. These library targets real-time performance under atmospheric conditions.

Rust was choosen as a language that offers high-level features with low-level speeds, provides memory safety without a garbage collector and doesn't allow undefined behaviour @rust_reference_undefined_behavior, making it ideal for safety-critical applications such as running the guidance system of a rocket @Pinho2019.

- Implement the @gfold algorithm with optimisations from Fast Algorithm for Onboard Atmospheric Powered Descent Guidance @ChenYushu2023AFAf using Rust with Clarabel.rs as our convex solver. This will allow for real-time performance and memory safety, vital for onboard computations during rocket descent.

- Implement a @rl approach using Pytorch to compare the performance and adaptability with the @gfold algorithm.

- Use the Bevy game engine for its ECS (Entity Component System) @Anderson2024bevyengine and Rapier physics engine @Crozet2024dimforge to develop a basic simulation environment; this will involve a fundamental rocket entity that can be controlled in real-time e.g. nozzle angle, thrust, pitch and yaw while providing altitude, velocity and acceleration metrics to the guidance algorithms.

- Incorporate atmospheric variables into the simulation, such as wind shear, air density fluctuations, and drag, to test the algorithm's capabilities and robustness.

- Define performance metrics such as landing accuracy, computational load and fuel efficiency to determine the effectiveness of each algorithm.

== Data Collection
- Data was collected from the both the algorithm and the simulation environment this data allows for a detailed comparative analysis between the algorithms output's and the how it performs in real-world conditions (simulated).

== Task Completed

- [x] Conducted initial research on the @gfold algorithm and convex optimization by reviewing research papers. (Due to current gaps in understanding, I have enrolled in Stanford EE364A for a deeper dive @boyd_vandenberghe_ee364a.)

- [x] Implementation of the simulation environment using Bevy for creating a basic wireframe visualisation for landing trajectories and Rapier for realistic physics simulations.

- [x] Implementation of the @gfold algorithm, focusing on a modular design and adherence to real-time constraints

- [x] Develop a communication protocol between the simulation environment and the algorithms to allow for real-time two-way communication while decoupling the simulation from the algorithms.

- [x] Add @socp to an existing Rust linear programming library to make modeling constraints easier.

- [ ] Advanced atmospheric simulation integration of the simulation environment to truly test the robustness of the algorithms against real-world

- [ ] Integration of the @gfold algorithm with simulation environment to allow for testing against different scenarios.

- [ ] Conduct a detailed comparative analysis between @gfold and the @rl approach on pre-defined parameters like fuel efficiency and adaptability.

- [x] Extensive documentation and reporting through a final year project report and code documentation.
