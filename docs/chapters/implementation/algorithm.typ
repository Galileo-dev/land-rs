#import "@preview/equate:0.3.1": equate
#import "@preview/showybox:2.0.4": showybox

== Convex Optimisation

=== Background

Convex optimisation aims to minimise a convex objective function over a set convex of convex constraints. The expresiveness of this technique allows it to handle a wide range of control problems, and has some very appealing properties @boyd2004convex:
- The local solution is also the global solution.
- A problem can be verified to be infeasible or unbounded when a feasible solution does not exist.
- The runtime complexity is polynomial in the problem size
- Algorithms can self-initalise, eliminating the need for an expert initial guess.

This makes it safer than other techniques for autonomous rocket landing, where a failure of the algorithm to converge could result in a catastrophic failure or in the case of human spaceflight, a loss of life.

=== Implementation

For this @fyp I implemented *Successive Convexification for Fuel-Optimal
Powered Landing with Aerodynamic Drag
and Non-Convex Constraints* @Szmuk2016 which formulates @apdg as a successive convexification problem using @lc and @sc to handle the non-convex constraints, using a procedure that:
- Make assumptions about the drag and speed profile for the initial trajectory.
- Linearises the non-linear dynamics constraints about a reference trajectory using a first-order Taylor expansion.
- Introduces trust-regions and relaxation terms to keep the solution bounded and feasible throughout the convergence process.
- Uses an iterative process by which the problem is repeatedly linearised about a solution obtained from the previous iteration.
- Solve the convexified sub-problemsw using an @ipm solver.

#showybox(
  title: "Convex Set ",
  columns(2)[

    A set $C$ is convex if and only if it contains the line segment that connects any two points in the set.

    $ x, y in C => [x, y]_theta in C $

    for all $theta in [0, 1]$, where $[x, y]_theta eq.delta (1 - theta)x + theta y$. Convexity is also preserved when sets intersect as longs as the intersecting sets are convex.

    #colbreak()

    #figure(
      image("../../assets/convex_set.png", width: 80%),
      caption: [*Convex set example.*],
    ) <convex_set>
  ],
)

#showybox(
  title: "Convex Function ",
  columns(2)[
    A function $f: RR^n -> RR$ is convex if and only if its domain $"dom" f$ is a convex set and $f$ lies below the line segment connecting any two points:

    $ x, y in "dom" f => f([x, y]_theta) <= [f(x), f(y)]_theta $

    for all $theta in [0, 1]$.

    #colbreak()

    #figure(
      image("../../assets/convex_function.png", width: 80%),
      caption: [*Convex function example.*],
    ) <convex_function>
  ],
)

#showybox(
  title: "Convex problem ",
  columns(2)[
    A convex optimisation is problem is the maximisation of a convex function subject to convex constraints that aim to restrict the search space:
    $
      & max_(x in RR^n) f_0(x) \
      & "s.t." & f_i(x) <= 0, & i=1,...,n_("ineq,") \
      && g_i(x) = 0, & i=1,...,n_("eq,")
    $
    where $f_0: RR^n -> RR$ is a convex cost function, $f_i: RR^n -> RR$ are convex inequality constraints, and $g_i: RR^n -> RR$ are convex equality constraints.
  ],
)
== Problem Formulation
This section formulates the @apdg problem in the @sc style and explains the process of turning these into contraints that Clarabel can solve.

We remove the non-linearities through @sc:
1. *Lossless convexification:* introduces auxiliary variables to linearise the constraints.
2. *linearisation:* Non linearities are removed through the use of a first-order Taylor expansion.
3. *Trust-region and relaxation:* Trust-region contraints are added to bound the decision variables between each iteration.
4. *Iterative Refinement:* For each iteration $i > 0$ the previous solution is used as a linearisation point for the next, until some convergence criteria is met.

We use a thrust vector $T(t)$ as the direction and magnitude of the thrust force for a given timestep $Delta t$.
The goal is to find the optimal thrust vector $T(t)$ that minimises fuel consumption (or maximises final mass) while keeping the rocket on a safe and precise landing trajectory.

// TOOD: add an explaination of convex optimisation from https://ieeexplore.ieee.org/stamp/stamp.jsp?tp=&arnumber=9905530

=== Successive Convexification for @apdg

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

== Code Implementation

Rust is well equipped to handle convex optimisation tasks; a very prominent library, *Clarabel*, is written entirely in pure Rust; this is also used as the default solver for *CVXPY*, a well-regarded Python library for modelling and solving convex optimisation problems.

=== Patching

Clarabel does not offer a modelling language and, as such, requires much reformulation of the problem to fit the specification of the solver. Instead, a patch was applied to a linear programming library, which offers a variety of ways to model and solve linear programming problems; however, it lacked the support for the second-order cone constraint required by the above algorithm @Szmuk2016.

`patch-crate` was used to patch the `good_lp` @good_lp_patch crate to add support for the second-order cone constraints.
The general form of the problem is as follows:

$ min c^T x $

$ "Subject to" quad A x = b, quad x in K $

where:

- $x$ is the decision variable (i.e position, velocity, mass and thrust).
- $c^T x$ is the objective function
- $A x = b$ represents the equality constraints (i.e. vehicle dynamics, fuel mass depletion, glide slope constraints, thrust constraints).
- $K$ represents @soc:pl cone constraints that enforce the thrust direction, minimum glide slope, acceleration limits.

The @soc are defined as:
$ K^n_S = { v in R^n | v_1 ≥ || v_(2:n) || } $

== Results

#figure(
  image("../../assets/trajectory_chart.png", width: 80%),
  caption: [*Three-dimensional trajectory produced by the last successive convexification iteration.* The dots along the
    trajectory indicate discretisation points, and the lines intersecting the trajectory at the discretisation points represent scaled com-
    manded thrust vectors.],
) <trajectory_chart>


#figure(
  image("../../assets/pos_vel_chart.png", width: 80%),
  caption: [*Up, east, and north components of the positions and velocities.* This figure shows a componentwise representation
    of the positions and velocities of the trajectory shown in Figure 1. The hop maneuver is evident in the up-position plot at the top
    left],
) <pos_vel_chart>

#figure(
  image("../../assets/thrust_mass_chart.png", width: 80%),
  caption: [*Thrust profile of the converged trajectory.* In the top left plot, the vehicle’s vacuum thrust profile is shown, along
    with the variable Γ, and the minimum and maximum thrust constraints. In the top right plot, the thrust magnitude rate is shown
    along with its minimum and maximum bounds. The bottom left plot shows the tilt angle of the commanded thrust vector, as well
    as the 15° maximum tilt limit. Lastly, the bottom right plot shows the azimuth of the thrust vector.],
) <thrust_mass_chart>

#figure(
  image("../../assets/mass_chart.png", width: 80%),
  caption: [*Vehicle mass as a function of time.* The dashed lines at the top and bottom of the figures represent the initial mass
    and the dry mass of the vehicle, respectively.],
) <mass_chart>

#figure(
  image("../../assets/convergence_chart.png", width: 80%),
  caption: [*Iteration history of position, velocity, and thrust.* Each plot shows the quantity $log max_k delta x_i [k]$ at each SC iteration for which $i > 0$.],
) <convergence_chart>

#figure(
  image("../../assets/relaxation_convergence_chart.png", width: 80%),
  caption: [*Iteration history of the SC relaxation term.* The figure shows the maximum value of $||a_R||$ over all $k in [0, k_f]$ for each SC iteration.],
) <relaxation_convergence_chart>




