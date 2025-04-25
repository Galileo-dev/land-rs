= Problem Analysis

== Reusable Rocket and The Importance of Precision Landing
In the past 50 years, autonomous spacecraft have brought humans and satellites to orbit, safely returned humans to Earth, and landed rovers on Mars. Precise landing capability allows missions to land rovers closer to areas of interest @Bonfiglio2011.

Landing an autonomous spacecraft or rocket is challenging—landing within meters of a predetermined target under varying atmospheric conditions is crucial. This precision enables fully reusable rockets, akin to aircraft refuelling and reuse. Commercial companies like SpaceX and Blue Origin have recently demonstrated landings within meters of their targets @blackmore2017.


== Problem Analysis
When a vehicle enters an atmosphere, it encounters several challenges:
- *Friction and Heating:* Most reentry energy is dissipated through friction, leading to extreme heating requiring a heat shield @blackmore2017.
- *Drag:* Significant drag forces affect the vehicle. For example, the Falcon 9 reusable rocket experiences up to 6g deceleration @blackmore2017.
- *High Winds:* Wind speeds can reach 160 km/h causing it to veer off course without real-time feedback. @blackmore2017.
- *Communication Blackouts:* Ionized air can cause temporary communication losses, as experienced by Apollo 13's 6-minute blackout @blackmore2017.
- *Radiation:* High radiation levels impact onboard flight computers and electronics @blackmore2017.

=== Small Margin for Error
The first landing attempt must succeed; failure means vehicle destruction on impact. Carrying extra fuel for a second attempt is primarily infeasible. Large rocket engines struggle to throttle down to hover and require continuous propellant to maintain altitude. Most large rockets don't have a low enough minimum throttle, so, during landing, the rocket will have a @twr above zero. When the velocity reaches zero, the rocket will start going back up @blackmore2017.

=== Hardware Limits
A successful guidance system must compute divert trajectories without exceeding hardware capabilities or safety constraints. Large rocket engines have thrust constraints preventing hovering, requiring continuous descent to minimize propellant usage @blackmore2017.



== Trajectory Optimization Approaches
- *Polynomial guidence:* Used in the apollo program to land astornauts on the moon, trajectories are represented as polynomials parameterised by time, and while it is very efficent to compute enabling missions with very limited computation power, like that of the @agc,it also require precise coefficeients that are precomputed and stored @Klumpp1974 However it is not fuel optimal @Ross2004.

- *Convex Optimization:* Used by SpaceX to land their Falcon9 1st stage and the new Starship 1st and 2nd stage. Convex optimization methods like @gfold are computationally efficient but require accurate models of the vehicle and their environment @G-FOLD2012.

- *Reinforcement Learning:* Reinforcement learning methods like @rl are more robust but require extensive training and are computationally expensive @FURFARO2020156.

The main challange involves solving the fuel-optimal @apdg problem in real-time as it's impossible to precompute the optimal trajectory for every possible scenario and especially with atmosphereic disturbances like wind. Convex optimisation and Reinforcement learning

=== G-FOLD Algorithm
The @gfold algorithm operates through lossless convexification, enabling real-time computation of fuel-optimal trajectories @G-FOLD2012. While current implementations use Python to generate C code with CVXGEN @Mattingley2012, this mixed-language approach can introduce unsafe assumptions that might crash programs during critical phases like rocket descent.

@gfold currently solves @pdg, but @apdg is needed for bodies with atmospheres like Earth and Mars. Real-time @apdg solutions are achievable with runtimes around 0.6s on flight hardware through successive convexification @ChenYushu2023AFAf.


=== Reinforcement Learning Algorithms
@rl approaches combine classical ZEM/ZEV methods with @rl to improve fuel efficiency and constraint handling. While classical ZEM/ZEV algorithms work for precision landing, they struggle with thrust limits and glide slope constraints. Modern @rl solutions include:

- *Actor-Critic Models:* These update parameters based on the lander state, allowing expansion to complex environments. Training in simulated Mars conditions helps encode terrain avoidance and glide slope constraints @FURFARO2020156.

- *Proximal Policy Optimization (PPO):* This closed-loop method maps thrust commands adaptively, handling uncertainties in mass, gravity, and disturbances. "Shaping rewards" guide trajectories toward soft pinpoint landings @GAUDET20201723.

- *Deep Classification and Regression Networks:* DCRNG combines classification and regression networks, achieving 96.5% success for bang-bang profiles and 91.6% for singular profiles @Wang2023.

== Conclusion
While @rl solutions require extensive training, they deliver efficient runtime performance and greater adaptability. In contrast, @gfold offers more optimal fuel consumption but is more sensitive to model inaccuracies. A balanced approach may yield the best performance for missions requiring both resilience and precision.

== Proposed Solution
I propose a modular Rust library implementing @gfold using Clarabel.rs for second-order cone solving and a python library implement a @rl approach using Pytorch. These libraries will target real-time performance under atmospheric conditions and be hosted on GitHub.

I've opted for Rust, a language that offers high-level features with low-level speeds, provides memory safety without a garbage collector and doesn't allow undefined behaviour @rust_reference_undefined_behavior, making it ideal for safety-critical applications such as running the guidance system of a spacecraft @Pinho2019.
