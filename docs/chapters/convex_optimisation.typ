#import "@preview/equate:0.3.1": equate
#import "@preview/showybox:2.0.4": showybox

= Convex Optimisation

== Background

Convex optimisation aims to minimise a convex objective function over a set of convex constraints. The expressiveness of this technique allows it to handle a wide range of control problems, and has some very appealing properties @boyd2004convex:
- The local solution is also the global solution.
- A problem can be verified to be infeasible or unbounded when a feasible solution does not exist.
- The runtime complexity is polynomial in the problem size
- Algorithms can self-initialise, eliminating the need for an expert initial guess.

This makes it safer than other techniques for autonomous rocket landing, where a failure of the algorithm to converge could result in a catastrophic failure or, in the case of human spaceflight, a loss of life.


== Convex Optimisation Fundamentals
This section will explore and provide a surface-level understanding of convex optimisation, and is mainly based on teachings from @boyd2004convex and @Malyuta2022.

#showybox(
  title: "Convex Set ",
  columns(2)[

    A set $C$ is convex if and only contains the line segment that connects any two points in the set.

    $ x, y in C => [x, y]_theta in C $

    for all $theta in [0, 1]$, where $[x, y]_theta eq.delta (1 - theta)x + theta y$. Convexity is also preserved when sets intersect as long as the intersecting sets are convex.

    #colbreak()

    #figure(
      image("../assets/convex_set.png", width: 80%),
      caption: [*Convex set example.*],
    ) <convex_set>
  ],
)

#showybox(
  title: "Convex Function ",
  columns(2)[
    A function $f: RR^n -> RR$ is convex if and only if its domain $"dom" f$ is a convex set and $f$ lies below the line segment connecting any two points:

    $ x, y in "dom" f => f([x, y]_theta) <= [f(x), f(y)]_theta $

    For all $theta in [0, 1]$.

    #colbreak()

    #figure(
      image("../assets/convex_function.png", width: 80%),
      caption: [*Convex function example.*],
    ) <convex_function>
  ],
)

#showybox(
  title: "Convex problem ",
  columns(2)[
    A convex optimisation problem is the maximisation of a convex function subject to convex constraints that aim to restrict the search space:
    $
      & max_(x in RR^n) f_0(x) \
      & "s.t." & f_i(x) <= 0, & i=1,...,n_("ineq,") \
      && g_i(x) = 0, & i=1,...,n_("eq,")
    $
    where $f_0: RR^n -> RR$ is a convex cost function, $f_i: RR^n -> RR$ are convex inequality constraints, and $g_i: RR^n -> RR$ are convex equality constraints.
  ],
)
