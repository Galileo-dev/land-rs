= Literature Review

== Reusable Rocket and The Importance of Precision Landing
In the past 50 years, autonomous spacecraft have brought humans and satellites to orbit, safely returned humans to Earth, and landed rovers on Mars. Precise landing capability allows missions to land rovers closer to areas of interest @Bonfiglio2011.

Landing an autonomous spacecraft or rocket is challenging—landing within meters of a predetermined target under varying atmospheric conditions is crucial. This precision enables fully reusable rockets, akin to aircraft refuelling and reuse. Recently, commercial companies like SpaceX and Blue Origin have demonstrated landings within meters of their targets @blackmore2017.

== G-FOLD Algorithm
The G-FOLD algorithm addresses these challenges through lossless convexification, enabling real-time computation of fuel-optimal trajectories @GFOLD-foundation. While current implementations use Python to generate C code with CVXGEN @Mattingley2012, this mixed-language approach can introduce unsafe assumptions that might crash programs during critical phases like rocket descent.

G-FOLD currently solves powered descent guidance (PDG), but atmospheric powered descent guidance (APDG) is needed for bodies with atmospheres like Earth and Mars. Real-time APDG solutions are achievable with runtimes around 0.6s on flight hardware @ChenYushu2023AFAf.

== Problem Analysis
When a vehicle enters an atmosphere, it encounters several challenges:
- *Friction and Heating:* Most reentry energy is dissipated through friction, leading to extreme heating and necessitating a heat shield.
- *Drag:* Significant drag forces affect the vehicle.
- *High Winds:* Wind speeds can reach up to 160 km/h.
- *Communication Blackouts:* Ionized air can cause temporary communication losses, as experienced by Apollo 13's 6-minute blackout.
- *Radiation:* High radiation levels impact onboard flight computers and electronics @blackmore2017.

=== Small Margin for Error
The first landing attempt must succeed; failure means vehicle destruction on impact.

=== Hardware Limits
A successful guidance system must compute divert trajectories without exceeding hardware capabilities. Large rocket engines have thrust constraints preventing hovering, requiring continuous descent to minimize propellant usage @blackmore2017.

== Reinforcement Learning Algorithms
Reinforcement learning approaches combine classical ZEM/ZEV methods with RL to enhance fuel efficiency and constraint handling. While classical ZEM/ZEV algorithms work for precision landing, they struggle with thrust limits and glide slope constraints. Modern RL solutions include:

- *Actor-Critic Models:* These update parameters based on lander state, allowing expansion to complex environments. Training in simulated Mars conditions helps encode terrain avoidance and glide slope constraints @FURFARO2020156.

- *Proximal Policy Optimization (PPO):* This closed-loop method maps thrust commands adaptively, handling uncertainties in mass, gravity, and disturbances. "Shaping rewards" guide trajectories toward soft pinpoint landings @GAUDET20201723.

- *Deep Classification and Regression Networks:* DCRNG combines classification and regression networks, achieving 96.5% success for bang-bang profiles and 91.6% for singular profiles @Wang2023.

== Conclusion
While RL solutions require extensive training, they deliver efficient runtime performance and adaptability. In contrast, G-FOLD offers optimal fuel consumption but is more sensitive to model inaccuracies. A balanced approach may yield the best performance for missions requiring both resilience and precision.

== Proposed Solution
I propose a modular Rust library implementing G-FOLD using Clarabel.rs for second-order cone solving. This library will target real-time performance under atmospheric conditions and serve as a foundation for future trajectory planning projects, hosted on GitHub @rust_reference_undefined_behavior @Pinho2019.

Rust offers an ideal combination of high-level features with low-level performance, memory safety without garbage collection, and freedom from undefined behavior—crucial for spacecraft guidance systems.
