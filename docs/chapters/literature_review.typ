== Reusable Rocket and The Importance of Precision Landing
In the past 50 years, autonomous spacecraft have brought humans and satellites to orbit, safely returned humans to Earth, and landed rovers on Mars. Precise landing capability allows missions to land rovers closer to areas of interest @Bonfiglio2011.

Landing an autonomous spacecraft or rocket is challenging—landing within meters of a predetermined target under varying atmospheric conditions is crucial. This precision enables fully reusable rockets, akin to aircraft refuelling and reuse. Commercial companies like SpaceX and Blue Origin have recently demonstrated landings within meters of their targets @blackmore2017.


== Trajectory Optimization Approaches

=== Polynomial guidance
Most prominent during the Apollo program to land astronauts on the moon. Trajectories are represented as polynomials parameterised by time, and is very efficient to compute. Enabling missions with very limited computation power, like that of the @agc. However it also requires precise coefficients that are precomputed and stored @Klumpp1974 and it is not fuel-optimal @Ross2004.

=== Convex Optimization
The paper "G-FOLD: A Real-Time Implementable Fuel Optimal Large Divert Guidance Algorithm for Planetary Pinpoint Landing" @G-FOLD2012 presents a novel convex optimisation approach called @gfold which computes a fuel-optimal trajectory for planetary pinpoint landing which require significant divert requirements. lossless convexification is used to convert the non-convex problem into a convex one that can be solved using a @ipm solver.

Experimental flight tests have been carried out through three successful flights of the autonomous "Xombie" vertical lander. The test included a horizontal diverts of 500, 650 and 750 meters, considerably greater than previous landings. These tests showed exceptional accuracy and precision, with a maximum positional deviation of less than one meter from the calculated trajectory, show great promise the fidelity of the algorithm against real-world constraints.

Even though the algorithm lacked the ability to handle atmospheric drag through real-time feedback, it was able to compensate for these unmodeled forces during the test campaign. This research highlights that importance of convex optimisation techniques for solving real-time fuel optimal trajectories that can adapt to unmodeled forces and provide a reliable approach and landing on planetary bodies. These finds lay the foundation for many future papers that utilise much more sophisticated models, enabling more complex but fuel-optimal trajectories in the presence of atmospheric drag.

It is also believed that this algorithm is similar in principle to the algorithm used by SpaceX to land their Falcon 9 1st stages which has been able to prove the real-world effectiveness of booster landing and reuse. The up coming Starship which is expect to be fully reusable (land both first and second stages) is also expected to use a similar convex optimisation algorithm. Convex optimization methods like @gfold are computationally efficient but require accurate models of the vehicle and their environment @G-FOLD2012.
// TODO: add the paper on this

- *Reinforcement Learning:* Reinforcement learning methods like @rl are more robust but require extensive training and are computationally expensive @FURFARO2020156.

The main challenge involves solving the fuel-optimal @apdg problem in real-time as it's impossible to precompute the optimal trajectory for every possible scenario and especially with atmospheric disturbances like wind. Convex optimisation and Reinforcement learning

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
