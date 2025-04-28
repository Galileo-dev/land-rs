= Literature Review

== Reusable Rocket and The Importance of Precision Landing
In the past 50 years, autonomous spacecraft have brought humans and satellites to orbit, safely returned humans to Earth, and landed rovers on Mars. Precise landing capability allows missions to land rovers closer to areas of interest @Bonfiglio2011.

Landing an autonomous spacecraft or rocket is challenging—landing within meters of a predetermined target under varying atmospheric conditions is crucial. This precision enables fully reusable rockets, akin to aircraft refuelling and reuse. Commercial companies like SpaceX and Blue Origin have recently demonstrated landings within meters of their targets @blackmore2017.


== Trajectory Optimization Approaches

=== Polynomial guidance
Historically, the Apollo program used polynomial guidance to land astronauts on the moon. Trajectories are represented as polynomials parameterised by time and segmented by phase, enabling missions with very limited computation power, like that of the @agc. However it also requires precise tuning of coefficients @Klumpp1974 and was not fuel-optimal @Ross2004. It's precision was within a few hundred meters of the target making it unsuitable for reusable boosters which require pin-point accuracy and minimal fuel margins.

=== Classical Feedback Laws(ZEM/ZEV)
@zemzev algorithms use analytical methods to provide a closed-form solution that drives the miss-distance and velocity to zero at touchdown. @zemzev is computationally lightweight to compute as it uses simple physics formulas. @zemzev has been extensively used in the past for planetary landers as it's easily to implement and has sufficient accuracy. It works by continuously updating the required acceleration based on the stop-distance/time-to-go (the time until the relative distance to the target is zero) and current state of the vehicle. @zemzev_sample_code

=== Convex Optimization
The paper "G-FOLD: A Real-Time Implementable Fuel Optimal Large Divert Guidance Algorithm for Planetary Pinpoint Landing" @G-FOLD2012 presents a novel convex optimisation approach called @gfold which computes a fuel-optimal trajectory for planetary pinpoint landing which require significant divert requirements. Lossless convexification is used to convert the non-convex problem into a convex one that can be solved using a @ipm solver. @gfold can guaranteed fuel optimality and decent runtime performance.

Experimental flight tests have been carried out through three successful flights of the autonomous "Xombie" vertical lander. The test included a horizontal diverts of 500, 650 and 750 meters, considerably greater than previous landings. These tests showed exceptional accuracy and precision, with a maximum positional deviation of less than one meter from the calculated trajectory, show great promise the fidelity of the algorithm against real-world constraints.

Even though the algorithm lacked the ability to handle atmospheric drag through real-time feedback, it was able to compensate for these unmodeled forces during the test campaign. This research highlights that importance of convex optimisation techniques for solving real-time fuel optimal trajectories that can adapt to unmodeled forces and provide a reliable approach and landing on planetary bodies. These finds lay the foundation for many future papers that utilise much more sophisticated models, enabling more complex but fuel-optimal trajectories in the presence of atmospheric drag.

It is also believed that this algorithm is similar in principle to the algorithm used by SpaceX to land their Falcon 9 1st stages which has been able to prove the real-world effectiveness of booster landing and reuse. The up coming Starship which is expect to be fully reusable (land both first and second stages) is also expected to use a similar convex optimisation algorithm. Convex optimization methods like @gfold are computationally efficient but require accurate models of the vehicle and their environment @G-FOLD2012.
// TODO: add the paper on this

=== Reinforcement Learning
Traditional algorithms like @gfold have demonstrated robust capabilities for solving fuel-optimal trajectories, however recent advances in @drl have shown great promise for solving more complex optimal control problems while being more flexible with greater robust control policies. @drl is known for its ability to learn complex environments while operating in real-time allowing it to adapt to compel dynamics and disturbances @IAFRATE202540.

This section explores next generation algorithms that could rival the performance and robustness of @gfold, ... // TODO: Name other method adding later.
by applying @drl methods to @apdg, including @ppo, @sac, transformer-based agents, imitation learning and hybrid methods that combine traditional control with neural networks, and evaluate them against several metrics:
- Landing accuracy (pinpoint precision)
- Fuel efficiency
- Handing of disturbances
- Complexity and on-board computation requirements
- Training efficiency and sample efficiency

