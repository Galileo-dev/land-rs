= Convex Optimisation Approach

== Introduction
The Fast Algorithm for Onboard Atmospheric Powered Descent Guidance @ChenYushu2023AFAf is an algorithm developed to efficiently compute the trajectories of a rocket during descent to course correct it to an optimal landing trajectory while considering various constraints. It does this through second order cone programming (SOCP) which is a convex optimisation problem. We will implement this algorithm in this paper and compare it against the RL method. The core problem is to account for the high computational complexity of @apdg, which must be solved onboard a spacecraft in real-time.

== SOCP Problem Formulation
The paper outlines the problem Formulation as minimising the fuel consumption while keeping safe and precise landing trajectories while under aerodynamic forces.
The problem is formulated as a Second Order Cone Programming (SOCP) optimisation.

The goal is to find the optimal thrust vector $A(t)$ that minimises the fuel consumption while keeping the rocket on a safe and precise landing trajectory. The SOCP formulation is as follows:
== Decision Variables
- *Position vector*: $r(t) = (r_x, r_y, r_z)$ (m)
- *Velocity vector*: $v(t) = (v_x, v_y, v_z)$ (m/s)
- *Acceleration vector*: $a(t) = (a_x, a_y, a_z)$ (m/s²)
- *Thrust vector*: $T(t) = (T_x, T_y, T_z)$ (N)
- *Vehicle mass*: $m(t)$ (kg)
- *Final landing time*: $t_f$ (s)

== Objective Function (Minimise fuel consumption)

Minimize fuel consumption:

$min integral(0, t_f, abs(T(t)), t)$

Discretized form:


$min sum_(k=1)^N
  & abs(T_k) * Delta t_k$

where $N$ is the number of time steps and $Delta t_k$ is the step duration.

== Constraints

=== 1. Vehicle Dynamics (Point-Mass Model)

Motion follows Newton’s second law:

$dot(r) = v$

$dot(v) = (T / m) + g + D_a$

where gravity is:

$g = (0, 0, -g)$

and aerodynamic drag:

$D_a = - 1 / 2 * C_D * S_"ref" * ρ_0 * exp(-c_ρ * r_y) * abs(v) * v$

with:

- *$C_D$*: Drag coefficient
- *$S_"ref"$*: Reference area
- *$ρ_0$*: Atmospheric density at sea level
- *$c_ρ$*: Atmospheric density decay rate with altitude

=== 2. Mass Dynamics (Fuel Consumption)

Rocket fuel consumption:

$dot(m) = - abs(T) / (I_"sp" * g_0)$

where:

- *$I_"sp"$*: Specific impulse (s)
- *$g_0$*: Standard gravitational acceleration (m/s²)

=== 3. Thrust Constraints

Engine limits:

$T_min ≤ abs(T) ≤ T_max$

Thrust pointing constraint:

$cos(θ_T,max) ≤ (T ⋅ (-g)) / (abs(T) * abs(g))$

=== 4. Glide-Slope Constraint (Controlled Landing)

Controlled descent angle:

$abs(r_⊥) / r_z ≤ tan(θ_"gs")$

where *$r_l$* is the horizontal position component.

=== 5. Boundary Conditions

At *t_f*:

- *Final position*: $r(t_f) = r_"target"$
- *Final velocity*: $v(t_f) = 0$
- *Landing thrust alignment*: $T(t_f) ⋅ g ≥ 0$
