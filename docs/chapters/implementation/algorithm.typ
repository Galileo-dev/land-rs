#import "@preview/equate:0.3.1": equate
#import "@preview/showybox:2.0.4": showybox

= Atmospheric Powered Descent Guidance (APDG)
Here, we will focus on solving @apdg, which involves the problem of landing a vehicle through a planetary atmosphere, with only its propulsion system to decelerate and achieve a safe and accurate landing. This phase of flight is the most critical. It is characterised by many dynamic variables, including changes in atmospheric density, aerodynamic forces, and gravity effects, along with a crucial requirement to minimise the use of finite fuel propellant.

== Core Objectives
The main objectives of an @apdg problem are multi-fold and involve trade-offs:

+ *Fuel efficiency:* Minimising the fuel usage is critical as any excess fuel could have been used to increase the payload mass. We can formulate this as maximising the final mass, $m[k_f]$ @Szmuk2016 @Lu2023.

+ *Pinpoint accuracy:* For landing to be useful for a mission, especially for @rlv and science missions, it is critical that a vehicle can land at a specific point with minimal deviation. We can formulate this using a constraint on the final position, $r[k_f]$. We use [0, 0, 0] as the final position. so $r[k_f] = [0, 0, 0]$.

+ *Soft landing:* The vehicle must touchdown while following strict horizontal and vertical landing constraints to prevent damage to the vehicle. @Lu2023

+ *Vertical landing:* For most landing scenarios, the lander must be in a near-vertical orientation during landing. We use a glide-slope constraint to ensure the vehicle stays within a predefined cone-shaped landing corridor. @glide_slope

#figure(
  image("../../assets/glide_slope.png", width: 100%),
  caption: [A planetary landing glideslope cone. Source: @Malyuta2022],
) <glide_slope>

== Key Constraints
@apdg operates within a set of predefined constraints, which can be non-convex and time-varying:

+ *Initial state constraints:* We specify the starting position, velocity, mass, and thrust that align with the conditions of the rocket at inference time.
+ *Final state constraints:* We specify the final position, velocity, and thrust we wish to see at landing.

+ *Control input constraints:*
  - *Thrust limit:* Rocket engines cannot throttle to zero and instead have a minimum and maximum thrust, which results in the minimum thrust constraint being a source of non-convexity.
  - *Thrust vector gimbal limits:* Rocket engines have a gimbal limit or maximum tilt that limits the direction in which the engine can be pointed relative to the rocket body. These constraints are important for managing the control authority of a vehicle.
  - *Thrust magnitude change rate limit:* A limit on how quickly we can throttle up and down the rocket engine, changing the thrust magnitude.

+ *State constraints:*
  - *Glide-Slope constraints:* Have the vehicle stay within a predefined region, typically in the shape of an upside-down cone with the apex at the landing site. Ensuring the vehicle maintains a safe approach and sensor line-of-sight.
  - *Maximum angle of attack:* Limit on the vehicle's rotation to maintain stability.

+ *Free final time:* The stop-distance/time-to-go is often not fixed and must be optimised, adding another layer of convexity.

== Performance indicators
The efficiency of @apdg can be evaluated on the following key performance indicators:
+ *Fuel consumption:* Total amount of fuel propellant used. Measured with $m_f - m_"dry"$
+ *Landing accuracy:* Total deviation from the pre-specified landing position and velocity at touchdown.
+ *Computation efficiency:* The algorithm's time to compute a feasible solution. Most important for on-board, real-time calculations, where a solution must be produced rapidly and reliably or risk the loss of the vehicle and payload.
+ *Robustness:* The degree to which the algorithm adhered to safety margins when faced with uncertainties, e.g. atmospheric effects, actuators imperfections, navigation error and discrepancies in vehicle parameters. Assessed using Monte Carlo simulations.
+ *Constraint Error:* How well does the algorithm satisfy all the constraints throughout the trajectory?
+ *Convergence behaviour:* @sc is an iterative algorithm that requires convergence to find a feasible or near-optimal solution. For @rl, this will refer to the stability of the train process.
+ *Adaptability:* How well can the algorithm adapt to new information or conditions, such as a command to re-target or detect a hazard?

== Implementation

For this @fyp, I implemented *Successive Convexification for Fuel-Optimal
Powered Landing with Aerodynamic Drag
and Non-Convex Constraints* @Szmuk2016, which formulates @apdg as a successive convexification problem using @lc and @sc to handle the non-convex constraints, using a procedure that:
- Make assumptions about the drag and speed profile for the initial trajectory.
- Linearises the non-linear dynamics constraints about a reference trajectory using a first-order Taylor expansion.
- Introduces trust regions and relaxation terms to keep the solution bounded and feasible throughout the convergence process.
- Uses an iterative process by which the problem is repeatedly linearised about a solution obtained from the previous iteration.
- Solve the convexified sub-problems using an @ipm solver.

== Problem formulation
This section formulates the @apdg problem in the @sc style and explains the process of turning these into constraints that Clarabel can solve.

@apdg is a continuous-time optimal control problem, with the objective to minimise fuel consumption (maximise the final mass) subject to various vehicle dynamics (3-DOF translational motion, thrust, gravity, drag and point mass), state boundary conditions, and control constraints (glide-slope, minimum/maximum thrust). @Szmuk2016

We remove the non-linearities through @sc:
+ *Discretisation:* The continuous-time problem is discretised over a fixed number of time steps ($N$), which are included as an optimisation variable. The sum of which is the total final time.
+ *Lossless convexification:* non-convex constraints like minimum thrust ($T_min <= norm(T)$) and the vehicle's tilt are reformulated into convex ones by introducing relaxation variables. This relaxed problem will solve the original non-convex problem under certain conditions.
+ *linearisation:* Non-linearities are removed through a first-order Taylor expansion.
+ *Trust-region and relaxation:* Trust-region constraints are added to bound the decision variables between each iteration.
+ *Iterative Refinement:* For each iteration $i > 0$, the previous solution is used as a linearisation point for the next until convergence criteria are met.

We use a thrust vector $T(t)$ as the direction and magnitude of the thrust force for a given timestep $Delta t$.
The goal is to find the optimal thrust vector $T(t)$ that minimises fuel consumption (or maximises final mass) while keeping the rocket on a safe and precise landing trajectory.

=== Algorithm and Equations

// underline the headings
#show heading: it => {
  underline(it.body)
}

#show: equate.with(breakable: true, sub-numbering: false)
#set math.equation(numbering: "(1)", supplement: [Eq.])
#counter(math.equation).update(60)

Assume $k in [0, k_f]$ unless otherwise specified.

#showybox(title: "Problem 4 from " + cite(<Szmuk2016>))[
  === Objective function:

  $ min_(T, Gamma) quad -w_(m,f) m[k_f] + w_(kappa, a, R) norm(kappa_(a, R)) " subject to:" $ <objective_function>

  === Boundary Conditions:

  $
    & m[0] = m_0, space r[0] = r_0, space v[0] = v_0, space T[0] = Gamma_0 hat(n)_0, space Gamma[0] = Gamma_0 \
    & r[k_f] = 0, space v[k_f] = 0, space T[k_f] = Gamma[k_f] hat(n)_f
  $

  === Dynamics:

  $
    & m[k+1] = m[k] - (frac(alpha, 2) (Gamma[k] + Gamma[k+1]) + dot(m)_(b p)) Delta tau && wide k in [0, k_f) \
    & r[k+1] = r[k] + v[k] Delta tau + frac(1, 3) (a[k] + frac(1, 2) a[k+1]) Delta tau^2 && wide k in [0, k_f) \
    & v[k+1] = v[k] + frac(1, 2) (a[k] + a[k+1]) Delta tau && wide k in [0, k_f) \
    & a[k] = frac(1, mu[k]) (T[k] - frac(1, 2) rho S_D C_D s[k] v[k]) + a_R [k] + g &&
  $

  === State Constraints:
  $
    & m_"dry" <= m[k] \
    & norm(r[k]) cos(gamma_(g s)) <= hat(e)_u^T r[k]
  $

  === Control Constraints:
  $
    & norm(T[k]) <= Gamma[k] \
    & 0 <= T_"min" <= Gamma[k] <= T_"max" \
    & Gamma[k] cos(theta_"max") <= hat(e)_u^T T[k] \
    & dot(T)_"min" Delta tau <= Gamma[k+1] - Gamma[k] <= dot(T)_"max" Delta tau && wide k in [0, k_f)
  $

  === SC Modifications:
  $ norm(a_R[k]) <= kappa_(a, R)[k] $
]

#showybox(title: "Problem 5 from " + cite(<Szmuk2016>))[
  #show: equate.with(breakable: true, sub-numbering: false)
  #set math.equation(numbering: "(1)", supplement: [Eq.])
  #counter(math.equation).update(74)

  === Objective Function:

  $
    min_(Delta t, T, Gamma) -w_(m,f) m[k_f] + w_(eta, Delta t) eta_(Delta t) + w_(eta, T) norm(eta_T) + w_(kappa, a, R) norm(kappa_(a,R))
  $

  === Boundary Conditions:

  $
    & m[0] = m_0, space r[0] = r_0, space v[0] = v_0, space T[0] = Gamma_0 hat(n)_0, space Gamma[0] = Gamma_0 \
    & r[k_f] = 0, space v[k_f] = 0, space T[k_f] = Gamma[k_f] hat(n)_f
  $

  === Dynamics:

  $
    & Psi[k] eq.delta [ Delta t quad Psi_m^T [k] quad Psi_Gamma^T [k] quad Psi_v^T [k] quad Psi_T^T [k] quad Psi_(a, R)^T [k] ]^T && wide k in [0, k_f) \
    & f_m (Psi[k]) eq.delta -[ frac(alpha, 2) (Gamma[k] + Gamma[k+1]) + dot(m)_(b p) ] Delta t && wide k in [0, k_f) \
    & f_r (Psi[k]) eq.delta v[k] Delta t + frac(1, 3) (a[k] + frac(1, 2) a[k+1]) Delta t^2 && wide k in [0, k_f) \
    & f_v (Psi[k]) eq.delta frac(1, 2) (a[k] + a[k+1]) Delta t && wide k in [0, k_f) \
    & m[k+1] = m[k] + f_m(Psi_(i-1)[k]) + (partial f_m) / (partial Psi) |_(Psi_(i-1) [k]) delta Psi_i [k] && wide k in [0, k_f) \
    & r[k+1] = r[k] + f_r(Psi_(i-1)[k]) + (partial f_r) / (partial Psi) |_(Psi_(i-1) [k]) delta Psi_i [k] && wide k in [0, k_f) \
    & v[k+1] = v[k] + f_v(Psi_(i-1)[k]) + (partial f_v) / (partial Psi) |_(Psi_(i-1) [k]) delta Psi_i [k] && wide k in [0, k_f) \
    & a[k] = frac(1, m[k]) (T[k] + D[k]) + a_R [k] + g
  $

  === State Constraints:
  $
    & m_"dry" <= m[k] \
    & norm(r[k]) cos(gamma_(g s)) <= hat(e)_u^T r[k]
  $

  === Control Constraints:
  $
    & norm(T[k]) <= Gamma[k] \
    & 0 <= T_"min" <= Gamma[k] <= T_"max" \
    & Gamma[k] cos(theta_"max") <= hat(e)_u^T T[k] \
    & dot(T)_"min" Delta t <= Gamma[k+1] - Gamma[k] <= dot(T)_"max" Delta t && wide k in [0, k_f)
  $

  === SC Modifications:
  $
    & delta Delta t_i^2 <= eta_(Delta t) \
    & delta T_i^T [k] delta T_i [k] <= eta_T[k] \
    & norm(a_R[k]) <= kappa_(a, R) [k]
  $
]

// Stop underlining the headings
#show heading: it => {
  it.body
}

#showybox(title: "Algorithm 1 from " + cite(<Szmuk2016>))[
  #set enum(numbering: "a)")
  + Specify vehicle and environmental parameters (e.g. $m_"dry", theta_"max", P_"amb"$, etc.), boundary conditions (e.g. $r_0, Gamma_0, m_0$, etc.), and algorithm parameters (e.g. $N, n_"sc", w_(m,f)$, etc.).
  + Specify a time of flight guess, $t_(f,s)$, and compute $Delta tau$ using @time_of_flight_guess.
    #counter(math.equation).update(54)
    $ Delta tau = t_(f,s) / k_f $ <time_of_flight_guess>
    #counter(math.equation).update(94)
  + Compute mass and speed profiles for first iteration using @mass_profile and @speed_profile, for integers $k in [0, k_n]$.
    $ mu[k] = ( (k_n - k) / k_n ) m_0 + ( k / k_n ) m_"dry" $ <mass_profile>
    $ s[k] = ( (k_n - k) / k_n ) norm(v_0) + ( k / k_n ) norm(v_f) $ <speed_profile>
  + Solve Problem 4 using $Delta tau$, $mu$, and $s$ to obtain trajectory $cal(T)_0$.
  + For $i = 1, 2, ..., n_"sc" - 1$
    #set enum(numbering: "a)")
  + Solve Problem 5, linearizing about trajectory $cal(T)_(i-1)$, and obtaining trajectory $cal(T)_i$.
  + Exit if $cal(T)_i$ is within some acceptable tolerance of $cal(T)_(i-1)$, or if $i = n_"sc" - 1$.
    #set enum(numbering: "1.") // Reset outer numbering
]

#pagebreak()

== Conclusion

The results of the @sc Formulation show that the algorithm can guarantee convergence to a feasible solution while satisfying various safety constraints. This provides a baseline against which to compare @drl methods.

