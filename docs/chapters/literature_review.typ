= Literature Review
"Trajectory generation is the computation of a multidi-
mensional temporal state and control signal that satisfies a
set of specifications while optimising key mission objectives-
tives"-@Malyuta2022.


Traditional algorithms like @gfold have demonstrated robust capabilities for solving fuel-optimal trajectories. However, recent advances in @drl have shown great promise for solving more complex optimal control problems while being more flexible with greater robust control policies. @drl is known for its ability to learn complex environments while operating in real-time, allowing it to adapt to complex dynamics and disturbances @IAFRATE202540.

This section explores next-generation algorithms that could rival the performance and robustness of the current state-of-the-art methods like @gfold, @sc and @ppo.
By applying @drl methods to @apdg, including @ppo, @sac, transformer-based agents, imitation learning and hybrid methods that combine traditional control with neural networks, and evaluate them against several metrics:
- Landing accuracy (pinpoint precision)
- Fuel efficiency
- Handing of disturbances
- Complexity and on-board computation requirements
- Training efficiency and sample efficiency

We organise this discussion by reviewing traditional trajectory optimisation approaches, discussing modern @drl approaches, and finally comparing the two, including summary tables and diagrams. The goal is to provide insights into how deep learning can outperform convex optimisation methods in autonomous rocket landing.

== Traditional Trajectory Optimisation Approaches
Before discussing @drl methods, we will review the more traditional approaches, such as those used during the Apollo program, landing on other planets and those used in the Spacex Falcon 9 and Starship programs to achieve @rlv:pl.

=== Polynomial guidance
Historically, the Apollo program used polynomial guidance to land astronauts on the moon. Trajectories are represented as polynomials parameterised by time and segmented by phase, enabling missions with minimal computation power like the @agc. However, it also requires precise tuning of coefficients @Klumpp1974 and was not fuel-optimal @Ross2004. Its precision was within a few hundred meters of the target, making it unsuitable for reusable boosters requiring pinpoint accuracy and minimal fuel margins.

=== Classical Feedback Laws(ZEM/ZEV)
@zemzev algorithms use analytical methods to provide a closed-form solution that drives the miss-distance and velocity to zero at touchdown. @zemzev is computationally lightweight to compute as it uses simple physics formulas. @zemzev has been extensively used for planetary landers as it is easy to implement and has sufficient accuracy. It works by continuously updating the required acceleration based on the stop-distance/time-to-go (the time until the relative distance to the target is zero) and current state of the vehicle, however @zemzev assumes perfect knowledge of the vehicle's stop-distance/time-to-go, which is not possible for more complex scenarios like @apdg and can not account for more complex constraints (e.g. thrust limits or glide-slopes. An example reference implementation of @zemzev in Python is available: @zemzev_sample_code. Recent methods have aimed to make @zemzev more adaptive by tuning its parameters with machine learning techniques to outperform its limitations @FURFARO2020156. For @FURFARO2020156, the authors propose a method to use an actor-critic @rl algorithm to adjust @zemzev guidance gains in

=== Convex Optimisation
The trajectory generation problem is almost always non-convex, which means it is difficult to solve efficiently, and there is no guaranteed solution. Through discretisation and reformulation, we can solve the trajectory generation problem using a convex optimiser. Depending on the exact formulation and discretisation method, the finite-dimensional optimisation problem may be a non-linear (which is non-convex) optimisation problem (NLP). NLP optimisation is again not efficient to solve. More importantly, there is no guarantee that a solution exists or does not exist @boyd2004convex.

The paper "G-FOLD: A Real-Time Implementable Fuel Optimal Large Divert Guidance Algorithm for Planetary Pinpoint Landing" @G-FOLD2012 presents a novel convex optimisation approach called @gfold which computes a fuel-optimal trajectory for planetary pinpoint landing, which requires significant divert requirements. Lossless convexification is used to convert the non-convex problem into a convex one that can be solved using an @ipm solver. @gfold can guarantee fuel optimality and decent runtime performance.

Experimental flight tests have been conducted through three successful autonomous "Xombie" vertical lander flights. The test included a horizontal distance of 500, 650 and 750 meters, considerably greater than previous landings. These tests showed exceptional accuracy and precision, with a maximum positional deviation of less than one meter from the calculated trajectory, showing great promise for the fidelity of the algorithm against real-world constraints @ScharfDaniel2017.

Even though the algorithm could not handle atmospheric drag through real-time feedback, it could compensate for these unmodeled forces during the tests. This research highlights the importance of convex optimisation techniques for solving real-time fuel-optimal trajectories that can adapt to unmodeled forces and provide a reliable approach for landing on planetary bodies. These findings lay the foundation for many future papers that utilise much more sophisticated models, enabling more complex but fuel-optimal trajectories in the presence of atmospheric drag.

=== Deep Reinforcement Learning
Guidance and control are separate modules where guidance computes the optimal trajectory. Control manages the rocket to follow the computed trajectory by using sensor feedback to adhere closely to the computed trajectory.
@drl offers a unified approach where a single neural network agent handles guidance and control. Instead of computing a whole trajectory for each initial condition, the agent learns to make input states (e.g. acceleration, velocity, position, rotation) to control actions (e.g. throttle and nozzle pitch and yaw) by optimising expected reward through trial and error. We will review several novel @rl based algorithms for landing rockets with pinpoint accuracy and analyse how they achieve their results.

==== On-Policy Policy Gradient Methods (@ppo)

@ppo is a policy gradient reinforcement learning method popular for continuous control problems. @ppo imperatively updates and improves a stochastic policy by gradient ascent and a clipped surrogate objective function to limit the update size between each step to avoid instability. In a rocket landing scenario, @ppo has been shown to land rockets with near-pinpoint accuracy successfully.

- *Landing Accuracy*: @ppo can achieve near-pinpoint landings if the reward strongly penalises position errors. In a 3-DOF simulation task, a @ppo policy can bring the vehicle on the pad within a few meters. This shows that @ppo alone cannot match the sub-meter accuracy of a finely tuned classical controller.

- *Fuel Efficiency*: A minimisation fuel reward can be incorporated into a @ppo policy, but reaching an actual fuel-optimal landing similar to @gfold is challenging. It was found that a 6-DOF @ppo agent uses 60% of the fuel used by @sc @IAFRATE202540. Suggesting a well-trained RL agent can significantly outperform state-of-the-art methods in fuel efficiency.

- *Handling Disturbances*: During training, random disturbances (wind gusts, noisy sensors, centre-of-mass shifts, etc) were injected into the environment to help the agent generalise to real-world scenarios. The controller achieved low position and velocity errors in experiments under different unmodeled dynamics and disturbances. It almost matches that of a disturbance-free scenario. @ppo:pl closed-loop policy can react to continuous state deviations, providing the disturbance rejection of a classical feedback controller. However, an RL policy could be unstable and fail catastrophically if the disturbance/condition is far outside its training. Training with a broad set of scenarios is crucial, or domain randomisation is needed to achieve a robust policy.

- *Computational Complexity*: @rl has minimal runtime complexity, where the inference is a neural network forward pass. A real-world small network of about a few hundred neurons with an optional @lstm @Xue2023: running such a network on flight hardware is high-speed (the Imitation Learning + PPO-LSTM policy ran at 2.5 ms on a Jetson TX2 @Xue2023). Another group achieved inference time of 10ms on a ZU9E embedded platform (Zynq UltraScale+ ZU9EG). The main trade-off with @rl is the offline training time and cost, which require significant compute resources, used to simulate many landing scenarios often on GPU clusters @IAFRATE202540. Once a @ppo policy is trained, it is a small, fixed function that can be evaluated on board.

- *Training*: @ppo requires reward shaping to learn sensible behaviours and goal-oriented policies for tasks like landing. It is difficult for @ppo to learn if a sparse reward is used (only giving a reward on successful touchdown), as random exploration rarely finds a feasible landing trajectory. It was found that intermediate rewards (e.g. angle-of-attack, reducing velocity, etc) were effective. @IAFRATE202540 found that two-phase training was most effective, first the agent was guided towards a heuristic descent profile, then a second phase where the trajectory was refined for fuel optimality, significantly improving convergence.

// === Soft Actor-Critic Methods (@sac)
//todo

== Conclusion
//TODO(): Add tables and diagrams

Landing an autonomous rocket is a complex task requiring optimal control and machine learning. State-of-the-art @drl methods have demonstrated successful and reliable landing with pinpoint accuracy and high efficiency, rivalling the performance of methods like @gfold and @sc. Using imitation learning, @ppo
// and @sac
has been able to learn effective descent guidance from expert trajectories. More importantly, @drl has shown great runtime performance and can run on embedded platforms, respond to disturbances, and simplify the overall control architecture by combining guidance and control decisions into a single neural network.

