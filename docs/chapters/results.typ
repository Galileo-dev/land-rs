#import "@preview/showybox:2.0.4": showybox

= Results

The @sc algorithm was run using the Rust implementation with the modified modelling library to define constraints and variables. The Following figures show the optimisation process results, which gave us a feasible trajectory.

#figure(
  image("../assets/trajectory_chart.png", width: 100%),
  caption: [*Three-dimensional trajectory produced by the last successive convexification iteration.* The dots along the
    trajectory indicate discretisation points, and the lines intersecting the trajectory at the discretisation points represent scaled commanded thrust vectors.],
) <trajectory_chart>


#figure(
  image("../assets/pos_vel_chart.png", width: 100%),
  caption: [*Up, east, and north components of the positions and velocities.* This figure shows a componentwise representation
    of the positions and velocities of the trajectory shown in Figure 1. The hop manoeuvre is evident in the up-position plot at the top left],
) <pos_vel_chart>

#figure(
  image("../assets/thrust_mass_chart.png", width: 100%),
  caption: [*Thrust profile of the converged trajectory.* The top left plot shows the vehicle's vacuum thrust profile and the variables Γ and minimum and maximum thrust constraints. In the top right plot, the thrust magnitude rate is shown along with its minimum and maximum bounds. The bottom left plot shows the commanded thrust vector's tilt angle and the 15° maximum tilt limit. Lastly, the bottom right plot shows the azimuth of the thrust vector.],
) <thrust_mass_chart>

#figure(
  image("../assets/mass_chart.png", width: 100%),
  caption: [*Vehicle mass as a function of time.* The dashed lines at the top and bottom of the figures represent the initial mass and the dry mass of the vehicle, respectively.],
) <mass_chart>

#figure(
  image("../assets/convergence_chart.png", width: 100%),
  caption: [*Iteration history of position, velocity, and thrust.* Each plot shows the quantity $log max_k delta x_i [k]$ at each SC iteration for which $i > 0$.],
) <convergence_chart>

#figure(
  image("../assets/relaxation_convergence_chart.png", width: 100%),
  caption: [*Iteration history of the SC relaxation term.* The figure shows the maximum value of $||a_R||$ over all $k in [0, k_f]$ for each SC iteration.],
) <relaxation_convergence_chart>

This chapter presents and analyses the results of the @sc algorithm developed for this project. Due to the limited timeframe given for this final year project, the provided results are primarily focused on the

This chapter aims to conclude this FYP by presenting the results of the algorithm implemented. Due to the significant time constraints imposed during this FYP, these results are not extensive and do not show real-world performance, as I only had enough time to implement reference trajectories. Later, I will discuss adding a guidance component. The results presented here are the results of running the convex optimisation algorithms on varying initial conditions and constraints. For the @drl comparison, I will use another paper that has already implemented a comparable @drl approach.

We used the same algorithm parameters as @Szmuk2016. The aim was to see if our implementation matched the original algorithm we implemented, given the same parameters.

#showybox(
  title: "Algorithm Parameters ",
  columns(2)[

    #table(
      columns: (auto, auto, auto),
      align: center,
      [*Parameter*], [*Value*], [*Units*],
      $P$, "1.0", $ "kg/m"^3 $,
      $P_("amb")$, "100", $ "kPa" $,
      $g_0$, "9.807", $ "m/s"^2 $,
      $g$, $[-g_0, 0, 0]^T$, $ "m/s"^2 $,
      $m_("dry")$, "10,000", $ "kg" $,
      $A_("nozzle")$, "0.5", $ "m"^2 $,
      $I_("sp")$, "300", $ "s" $,
      $T_("min,vac")$, "100", $ "kN" $,
      $T_("max,vac")$, "250", $ "kN" $,
      $dot(T)_("min")$, "-100", $ "kN/s" $,
      $dot(T)_("max")$, "100", $ "kN/s" $,
      $theta_("max")$, "15", $ "deg" $,
      $gamma_("gs")$, "80", $ "deg" $,
      $S_D$, "10", $ "m"^2 $,
      $C_D$, "1.0", $$,
    )

    #colbreak()


    #table(
      columns: (auto, auto, auto),
      align: center,
      [*Parameter*], [*Value*], [*Units*],
      $t_("f,s")$, "15", $ "s" $,
      $m_0$, "15,000", $ "kg" $,
      $r_0$, $[500, 500, 0]^T$, $ "m" $,
      $v_0$, $[-50, 0, 50]^T$, $ "m/s" $,
      $Gamma_("0,vac")$, "175", $ "kN" $,
      $hat(n)_0$, $[0, 0, 1]^T$, $$,
      $hat(n)_f$, $[1, 0, 0]^T$, $$,
      $N$, "30", $$,
      $n_("SC")$, "10", $$,
      $w_("m,f")$, "1.0", $$,
      $w_(eta, Delta t)$, "0.0001", $$,
      $w_(eta, T)$, "0.0001", $$,
      $w_(kappa, a, R)$, "100", $$,
    )
  ],
)


== generated trajectory overview

@trajectory_chart shows that the optimiser generated a bang-coast-bang profile, meaning first starts at the set intial $-50"m/s"$ then zeros that out in the first $approx 5$ seconds, it then performs a translational burn to bring it closer to the landing target, finally it performs the landing burn touching down at < 0.2m/s. The vehicle's mass starts at 15000kg with 5,000kg of propellant and quickly consumes $approx 1950$ (39%) of the propellant in the $approx$29 seconds of the flight.


== Runtime performance
We achieved a runtime of about 460ms for the @sc algorithm running on a Ryzen 7 5800x3d. In comparison, @drl with @ppo achieved a runtime performance of 2.5ms @Xue2023.

