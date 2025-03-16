#import "@preview/equate:0.3.1": equate
#show: equate.with(breakable: true, sub-numbering: true)
#set math.equation(numbering: "(1.1)")

= Convex Optimisation Approach

== Introduction
In this paper we implement and evaluate the paper Convex optimization solutions versus @rl solutions. For the convex optimization solutions this paper will implement Fast Algorithm for Onboard Atmospheric Powered Descent Guidance @ChenYushu2023AFAf an algorithm developed to efficiently compute the trajectories of a rocket during descent and course correct it to an optimal landing trajectory while considering various constraints from previous papers. It does this through exploiting the sparsity in the problem structure of @socp with successive convexification @Szmuk2016. The core problem is to account for the high computational complexity of @apdg, which must be solved onboard a spacecraft in real-time to allow it to correct for deviations from the planned optimal trajectory.

== SOCP Problem Formulation
// The paper outlines the problem Formulation as minimising the fuel consumption while keeping safe and precise landing trajectories while under aerodynamic forces.
// The problem is formulated using @socp.

The goal is to find the optimal thrust vector $T(t)$ that minimises the fuel consumption while keeping the rocket on a safe and precise landing trajectory. The general form of the problem is as follows:

$ min c^T x $

$ "Subject to" quad A x = b, quad x in K $

where:

- $x$ is the decision variable (i.e position, velocity, mass and thrust).
- $c^T x$ is the objective function
- $A x = b$ represents the equality constraints (i.e. vehicle dynamics, fuel mass depletion, glide slope constraints, thrust constraints).
- $K$ represents @soc:pl cone constraints that enforce the thrust direction, minimum glide slope, acceleration limits.

The @soc are defined as:

$ K^n_S = { v in R^n | v_1 ≥ || v_(2:n) || } $

This @soc ensure that constraints like the thrust vector directions and non-linear aerodynamic drag are satisfied.

== Full convex optimization problem (@apdg)

The full convex optimization problem for the @apdg is as follows:

$ min integral_0^t_f || T(t) || d t $ <objective_function>

- Minimise the fuel consumption by minimising the thrust magnitude over time.

=== Vehicle Dynamics (Point-Mass Model)

$ dot(r) = v, quad dot(v) = frac(T, m) + g + D_a $ <vehicle_dynamics>

$ dot(m) = - (|| T ||) / (I_"sp" g_0) $ <mass_dynamics>

- Position $r$ and Velocity $v$ follows Newtonian mechanics.
- g is gravity vector $g = (0, 0, -g)$ and $D_a$ is aerodynamic drag.
- m is the mass of the vehicle, and decreases as fuel is consumed.

=== Thrust Constraints (SOC)

$ T_min ≤ || T(t) || ≤ T_max $ <thrust_constraints>

- Ensures the min and max thrust is within the engine limits.

=== Glide-Slope Constraint (SOC)

$ frac(r_z, sqrt(r_x^2 + r_y^2)) ≥ tan(θ_"gs"^max) $ <glide_slope_constraint>

- Prevents unsafe angles of attack.

=== Thrust direction constraint (SOC)

$ || T - hat(e)_z || ≤ tan(θ_T^max) T_z $ <thrust_direction_constraint>

- Limits the angle of the thrust vector.

=== Mass and Fuel Constraints

$ m_"dry" ≤ m(t) ≤ m_0 $ <mass_constraints>

- Ensures the vehicle has enough fuel to land.

=== Final Conditions

$ r(t_f) = r_"final", quad v(t_f) = 0 $ <final_conditions>

- Ensure the vehicle lands at the target location with zero velocity.
