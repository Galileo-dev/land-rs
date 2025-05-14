= Literature Review
This chapter provides an overview of the existing methods for trajectory generation, both traditional techniques like polynomial guidance and ZEM/ZEV, and modern approaches including convex optimisation (G-FOLD, SCvx) and next-generation Deep Reinforcement Learning approaches (PPO, SAC).

== Introduction
"Trajectory generation is the computation of a multidimensional temporal state and control signal that satisfies a set of specifications while optimising key mission objectives" - @Malyuta2022.

Algorithms like @gfold and @sc have demonstrated robust capabilities for solving fuel-optimal trajectories. However, recent advances in @drl have shown great promise for solving more complex optimal control problems while being more flexible with greater robust control policies. @drl is known for its ability to learn complex environments while operating in real-time, allowing it to adapt to complex dynamics and disturbances @IAFRATE202540.

This section explores next-generation algorithms that could rival the performance and robustness of the current state-of-the-art methods like @gfold, @sc and @ppo.
By applying @drl methods to @apdg, including @ppo, @sac, transformer-based agents, imitation learning and hybrid methods that combine traditional control with neural networks, and evaluate them against several metrics:
- Landing accuracy (pinpoint precision)
- Fuel efficiency
- Handing of disturbances
- Complexity and on-board computation requirements
- Training efficiency and sample efficiency

We organise this discussion by reviewing the need for autonomous landing rockets, then traditional trajectory optimisation approaches, discussing modern @drl approaches, and finally comparing the two, including summary tables and diagrams. The goal is to provide insights into how deep learning can outperform convex optimisation methods in autonomous rocket landing.

== Motivation

In the past 50 years, autonomous spacecraft have brought humans and satellites to orbit, safely returned humans to Earth, and landed rovers on Mars. Precise landing capability allows missions to land rovers closer to areas of interest @Bonfiglio2011.
Landing an autonomous spacecraft or rocket is challenging; landing within meters of a predetermined target under varying atmospheric conditions is even more difficult but crucial @MARS2020. This precision enables a new kind of rocket to be developed; fully reusable rockets, akin to aircraft refuelling and reuse. Generating optimal trajectories on board the vehicle is not just desirable but necessary, as it is not always possible to remotely control the spacecraft in real-time, e.g. Mars landing scenario @SanMartin2013 @Steltzner2014 @Way2007. Vehicles must be able to autonomously land and have their own robust and adaptive decision-making capabilities. Failure to generate and follow an optimal trajectory can result in losing the vehicle, payload and even human life. A reliable and robust landing system is a key factor in maintaining public trust and the complete safety of astronauts, which is necessary for future human spaceflight missions to be approved by regulatory bodies and to be successful @Stein2003.

Commercial companies like Spacex and Blue Origin have recently demonstrated landings within meters of their targets @blackmore2017 @Szmuk2016, and Spacex is now making successful Falcon9 booster landings a routine event during all launches.

For Mars 2020, a combination of @trn and @lvs successfully landed Perseverance *just 5m* from its aimpoint inside a crater covered with landing hazards, the new system significantly outperformed the mission requirements. It was a key factor for immediate access to valuable geological samples @MARS2020. A smaller landing ellipse allows a site choice to be driven by geology rather than engineering constraints. Jezero crater was chosen based on geological science merit once the ellipse was below 10x10km @GRANT2018106.

Precision landing enables a critical turning point in space exploration and payload delivery costs. @rlv:pl significantly reduce the operational and refurbishment costs, and significantly improve the economic viability of space flight. Reusability focuses on reducing the costs of the vehicle's recovery, inspection and refurbishment, mainly with high wear components like engine, avionics and tanks. Precision landing significantly drives down turnaround time as the logistics of recovery and refurbishment are reduced considerably or eliminated @webb2016economics.

== Trajectory Optimisation Approaches
Before discussing @drl methods, we will review the more traditional approaches, such as those used during the Apollo program, landing on other planets. Then we will discuss those used in the Spacex Falcon 9 and Starship programs to achieve the success they have. Finally, we will discuss the next-generation methods that are being developed and have shown excellent runtime performance and computational efficiency.

=== Polynomial guidance
Historically, the Apollo program used polynomial guidance to land astronauts on the moon. Trajectories are represented as polynomials parameterised by time and segmented by phase, enabling missions with minimal computation power like the @agc. However, it also requires precise tuning of coefficients @Klumpp1974 and was not fuel-optimal @Ross2004. Its precision was within a few hundred meters of the target, making it unsuitable for reusable boosters requiring pinpoint accuracy and minimal fuel margins.

=== Classical Feedback Laws(ZEM/ZEV)
@zemzev algorithms use analytical methods to provide a closed-form solution that drives the miss-distance and velocity to zero at touchdown. @zemzev is computationally lightweight to compute as it uses simple physics formulas. @zemzev has been extensively used for planetary landers as it is easy to implement and has sufficient accuracy. It continuously updates the required acceleration based on the stop-distance/time-to-go (the time until the relative distance to the target is zero) and the vehicle's current state. However, @zemzev assumes perfect knowledge of the vehicle's stop-distance/time-to-go, which is impossible for more complex scenarios like @apdg and can not account for more complex constraints (e.g. thrust limits or glide-slopes). An example reference implementation of @zemzev in Python is available: @zemzev_sample_code. Recent methods have aimed to make @zemzev more adaptive by tuning its parameters with machine learning techniques to outperform its limitations @FURFARO2020156. For @FURFARO2020156, the authors propose using an actor-critic @rl algorithm to adjust @zemzev guidance gains in real-time, resulting in a hybrid guidance law that was still lightweight but could be more flexible under varying conditions. Such an approach retains @zemzev:pl reliability but improves performance when the dynamics or constraints are outside the normal assumptions.

=== Convex Optimisation
The trajectory generation problem is almost always non-convex, which means it is difficult to solve efficiently, and there is no guaranteed solution. Algorithms like @gfold and @sc show that, through techniques like discretisation and @lc, we can solve the trajectory generation problem using a convex optimiser. Depending on the exact formulation and discretisation method, the finite-dimensional optimisation problem may be a non-linear (which is non-convex) optimisation problem (NLP). NLP optimisation is again not efficient to solve. More importantly, there is no guarantee that a solution exists or does not exist @boyd2004convex.

The paper "G-FOLD: A Real-Time Implementable Fuel Optimal Large Divert Guidance Algorithm for Planetary Pinpoint Landing" @G-FOLD2012 presents a novel convex optimisation approach called @gfold developed at NASA's Jet Propulsion Laboratory (JPL), which computes a fuel-optimal trajectory for planetary pinpoint landing, which requires significant divert requirements. Lossless convexification is used to convert the non-convex problem into a convex one that can be solved using an @ipm solver. @gfold can guarantee fuel optimality and decent runtime performance.

Experimental flight tests have been conducted through three successful autonomous "Xombie" vertical lander flights. The test included a horizontal distance of 500, 650 and 750 meters, considerably greater than previous landings. These tests showed exceptional accuracy and precision, with a maximum positional deviation of less than one meter from the calculated trajectory, showing great promise for the fidelity of the algorithm against real-world constraints @ScharfDaniel2017.

Even though the algorithm could not handle atmospheric drag through real-time feedback, it could compensate for these unmodeled forces during the tests. This research highlights the importance of convex optimisation techniques for solving real-time fuel-optimal trajectories that can adapt to unmodeled forces and provide a reliable approach for landing on planetary bodies. These findings lay the foundation for many future papers that utilise much more sophisticated models and techniques, enabling more complex but fuel-optimal trajectories in the presence of atmospheric drag.

@gfold was later extended to a @3dof and @6dof with @sc to handle full vehicle dynamics, including atmospheric drag @Szmuk2016 @Szmuk_2018. Convex solvers can guarantee a near-minimum fuel usage and precise landing accuracy under modelled dynamics and constraints. However, they must employ a separate guidance controller to follow their open-loop reference trajectory and require all constraints to be convexified. If enough guidance error (positional deviations due to wind) is accumulated, the trajectory may not be feasible anymore and must be re-solved.

=== Deep Reinforcement Learning
Guidance and control are separate modules where guidance computes the optimal trajectory. Control manages the rocket to follow the calculated trajectory by using sensor feedback to adhere closely to the computed trajectory.
@drl offers a unified approach where a single neural network agent handles guidance and control. Instead of computing a whole trajectory for each initial condition, the agent learns to map input states (e.g. acceleration, velocity, position, rotation) to control actions (e.g. throttle and nozzle pitch and yaw) by optimising expected reward through trial and error. We will review several novel @rl based algorithms for landing rockets with pinpoint accuracy and analyse how they achieve their results.

==== On-Policy Policy Gradient Methods

@ppo is a policy gradient reinforcement learning method popular for continuous control problems. @ppo imperatively updates and improves a stochastic policy by gradient ascent and a clipped surrogate objective function to limit the update size between each step to avoid instability. In a rocket landing scenario, @ppo has been shown to land rockets with near-pinpoint accuracy successfully.

- *Landing Accuracy*: @ppo can achieve near-pinpoint landings if the reward strongly penalises position errors. In a 3-DOF simulation task, a @ppo policy can bring the vehicle on the pad within a few meters. This shows that @ppo alone cannot match the sub-meter accuracy of a finely tuned classical controller.

- *Fuel Efficiency*: A minimisation fuel reward can be incorporated into a @ppo policy, but reaching an actual fuel-optimal landing similar to @gfold is challenging. It was found that a 6-DOF @ppo agent uses 60% of the fuel used by @sc @IAFRATE202540. Suggesting a well-trained RL agent can significantly outperform state-of-the-art methods in fuel efficiency.

- *Handling Disturbances*: During training, random disturbances (wind gusts, noisy sensors, centre-of-mass shifts, etc) were injected into the environment to help the agent generalise to real-world scenarios. The controller achieved low position and velocity errors in experiments under different unmodeled dynamics and disturbances. It almost matches that of a disturbance-free scenario. @ppo:pl closed-loop policy can react to continuous state deviations, providing the disturbance rejection of a classical feedback controller. However, an RL policy could be unstable and fail catastrophically if the disturbance/condition is far outside its training. Training with a broad set of scenarios is crucial, or domain randomisation is needed to achieve a robust policy.

- *Computational Complexity*: @rl has minimal runtime complexity, where the inference is a neural network forward pass. A real-world small network of about a few hundred neurons with an optional @lstm @Xue2023: running such a network on flight hardware is high-speed (the Imitation Learning + PPO-LSTM policy ran at 2.5 ms on a Jetson TX2 @Xue2023). Another group achieved inference time of 10ms on a ZU9E embedded platform (Zynq UltraScale+ ZU9EG). The main trade-off with @rl is the offline training time and cost, which require significant compute resources, used to simulate many landing scenarios often on GPU clusters @IAFRATE202540. Once a @ppo policy is trained, it is a small, fixed function that can be evaluated on board.

- *Training*: @ppo requires reward shaping to learn sensible behaviours and goal-oriented policies for tasks like landing. It is difficult for @ppo to learn if a sparse reward is used (only giving a reward on successful touchdown), as random exploration rarely finds a feasible landing trajectory. It was found that intermediate rewards (e.g. angle-of-attack, reducing velocity, etc) were effective. @IAFRATE202540 found that two-phase training was most effective, first the agent was guided towards a heuristic descent profile, then a second phase where the trajectory was refined for fuel optimality, significantly improving convergence.

=== Off-policy method
Off-policy reinforcement learning leverages learning from previous experiences that may have used an older or different policy. Allowing it to learn from a broader range of experiences, off-policy, particularly maximises reward while also maximising its entropy/exploration, resulting in a robust stochastic policy. Off-policy methods can be more sample-efficient than on-policy @ppo methods, due to their use of a replay buffer. This is more useful for environments that are expensive to simulate. Most notable algorithms include @ddpg, @td3 and @sd3 @Li2023. Each method improves upon the previous, solving a common issue of value function estimation biases, convergence and stability problems. We will discuss the results of each technique from the tests conducted by @Li2023:

- *Landing Accuracy:*
  - *DDPG:* Policy convergence is unstable, and the algorithm frequently fails to achieve a precise landing. Making it unsuitable for real-world applications by any means.
  - *TD3:* It is more stable than DDPG, but it is still inaccurate and frequently fails to land within the targeted zone. The algorithm underestimates the value function, which causes it to follow a conservative sub-optimal trajectory.
  - *SD3:* Shows the most promising accuracy. All landings within the 10m x 10m target zone. A softmax operator was used to reduce the bias of @td3, allowing for more stable convergence and higher landing accuracy.

- *Fuel Efficiency:*
  - *DDPG:* Often resulted in inefficient trajectories that increase fuel usage significantly compared to other methods due to overestimating the Q-values.
  - *TD3:* Improves in comparison to DDPG, often using less fuel due to a more stable trajectory, still conservative and underestimation occasionally resulted in a higher than optimal fuel consumption.
  - *SD3:* Shows significant improvements over the other two methods, with a smooth and accurate policy that results in an optimal descent profile minimising the fuel usage.

- *Handling Disturbances:*
  - *DDPG:* Highly sensitive, which leads to an unstable trajectory with unreliable landing results, with a not robust policy due to the unstable Q-value estimations.

  - *TD3:* Better and more robust than DDPG, could handle some moderate disturbances; however, it significantly struggles with larger, unmoddled variations due to the convervative policy decision caused by value underestimation.

  - *SD3* displays the most robust performance, softmax smoothing significantly improved the value function estimation accuracy. It significantly enhanced the agent's adaptability to real-time uncertainties such as aerodynamic forces, gravitational forces, Earth's rotation, etc.

- *Computational Complexity:*

  - *DDPG:* Like most RL algorithms, it is computationally efficient as it only involves a forward pass of the neural network.
  - *TD3:* More computationally complex due to its use of a twin Q-network and smoothing operation, but still relatively low inference time, usable for real-time on-board execution.
  - *SD3:* Increase computational complexity due to its use of a softmax operation during updates; however, inference times are still suitable for real-time on-board execution.

- *Training:*
  - *DDPG:* Careful tuning is required and tends to display unstable learning curves. Hyperparameter tuning and tial and error reward shaping are needed to achieve relatively slow convergence, yet it fails to stabilise.
  - *TD3:* More stable, mainly due to its delayed policy updates and policy smoothing, resulting in a more consistent improvement, but is slower to converge due to more conservative updates
  - *SD3:* highly stable and efficient during training, integrating a softmax value estimation has improved both convergence speed and stability. The policy converges with fewer training iterations and less manual tuning than DDPG and TD3.

Overall, @sd3 significantly advances off-policy reinforcement learning. It's an accurate and stable policy that could be a promising alternative to methods like convex optimisation in a real-world scenario @Li2023.


== Conclusion

Landing an autonomous rocket is a complex task requiring optimal control and machine learning. State-of-the-art @drl methods have demonstrated successful and reliable landing with pinpoint accuracy and high efficiency, rivalling the performance of methods like @gfold and @sc. Using imitation learning, @ppo
has been able to learn effective descent guidance from expert trajectories. More importantly, @drl has shown great runtime performance and can run on embedded platforms, respond to disturbances, and simplify the overall control architecture by combining guidance and control decisions into a single neural network.
