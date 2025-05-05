= Problem Analysis

The main problem to address is the generation of a dynamically feasible trajectory for spacecraft and @rlv:pl to land on a specific target safely, while remaining fuel-optimal.

When a vehicle enters an atmosphere, it encounters several challenges:
- *Friction and Heating:* Most reentry energy is dissipated through friction, leading to extreme heating requiring a heat shield @blackmore2017.
- *Drag:* Significant drag forces affect the vehicle. For example, the Falcon 9 reusable rocket experiences up to 6g deceleration @blackmore2017.
- *High Winds:* Wind speeds can reach 160 km/h causing it to veer off course without real-time feedback. @blackmore2017.
- *Communication Blackouts:* Ionized air can cause temporary communication losses, as experienced by Apollo 13's 6-minute blackout @blackmore2017.
- *Radiation:* High radiation levels impact onboard flight computers and electronics @blackmore2017.


== Trajectory Definition
A trajectory is defined as a temporal state and control signal over time @Malyuta2022. Being "dynamically feasible" means the trajectory must strictly satisfy the vehicle's dynamics (e.g. equations of motion and actuator limits) at every point in time. Onboard trajectory generation is crucial as communication is not always possible.


== Non-convexity
As mentioned in the introduction, the trajectory generation problem is almost always non-convex, This makes it difficult to efficently and accurately solve a trajectory generation problem, however we can apply a systematic approach to handle these non-convexitie and generate a feasible trajectory using a convex solver. The two main methods are:

- *Lossless Convexification*: involves reformulating the non-convex problem a convex one through variable substitution and "lifting" of the control inputs into a higher-dimensional space @Malyuta2022. Most importantly the new problem is globally optimal of the original non-convex problem. This is why it's called "lossless convexification (LCvx)", and it can be solve with a single call to a convex solver @Malyuta2022
== Small Margin for Error
The first landing attempt must succeed; failure means vehicle destruction on impact. Carrying extra fuel for a second attempt is primarily infeasible. Large rocket engines struggle to throttle down to hover and require continuous propellant to maintain altitude. Most large rockets don't have a low enough minimum throttle, so, during landing, the rocket will have a @twr above zero. When the velocity reaches zero, the rocket will start going back up @blackmore2017.

== Hardware Limits
A successful guidance system must compute divert trajectories without exceeding hardware capabilities or safety constraints. Large rocket engines have thrust constraints preventing hovering, requiring continuous descent to minimize propellant usage @blackmore2017.
