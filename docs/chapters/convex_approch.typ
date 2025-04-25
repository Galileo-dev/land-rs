#import "@preview/equate:0.3.1": equate
#import "@preview/showybox:2.0.4": showybox

= Convex Optimisation

For this @fyp I implemented *Successive Convexification for Fuel-Optimal
Powered Landing with Aerodynamic Drag
and Non-Convex Constraints* @Szmuk2016 which formulates @apdg as a successive convexification problem using @sc to handle the non-convex constraints, using a procedure that:
- Make assumptions about the drag and speed profile for the initial trajectory.
- Linearises the non-linear dynamics constraints about a reference trajectory using a first-order Taylor expansion.
- Introduces trust-region and relaxation terms to keep the solution bounded and feasible throughout the convergence process.
- Uses an iterative process by which the problem is repeatedly linearised about a solution obtained from the previous iteration.
- Solve the convexified sub-problemsw using an @ipm solver.


== Problem Formulation
This section formulates the @apdg problem in the @sc style and explains the process of turning these into contraints that Clarabel can solve.

We remove the non-linearities through @sc:
1. *Lossless convexification:* introduces auxiliary variables to linearise the constraints.
2. *linearisation:* Non linearities are removed through the use of a first-order Taylor expansion.
3. *Trust-region and relaxation:* Trust-region contraints are added to bound the decision variables between each iteration.
4. *Iterative Refinement:* For each iteration $i > 0$ the previous solution is used as a linearisation point for the next, until some convergence criteria is met.

We use a thrust vector $T(t)$ as the direction and magnitude of the thrust force for a given timestep $Delta t$.
The goal is to find the optimal thrust vector $T(t)$ that minimises fuel consumption (or maximises final mass) while keeping the rocket on a safe and precise landing trajectory.

== Successive Convexification for @apdg

// underline the headings
#show heading: it => {
  underline(it.body)
}


#set enum(numbering: "a)")
+ Assume $k in [0, k_f]$ unless otherwise specified.
+

#showybox(title: "Problem 4 from " + cite(<Szmuk2016>))[
  #show: equate.with(breakable: true, sub-numbering: false)
  #set math.equation(numbering: "(1)", supplement: [Eq.])
  #counter(math.equation).update(60)


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

#pagebreak()

== Code Review

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
