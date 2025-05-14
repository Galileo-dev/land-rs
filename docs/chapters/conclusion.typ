= Conclusion
This @fyp report summarises the progress in comparing convex optimisation and @rl approaches for autonomous rocket landing. We have discussed the development progress of the simulation environment, the mathematical formulation of the convex optimisation approach and outlining the development plan.

Time was limited when developing this FYP. There are some features I wanted to implement, but due to time constraints, they could not be completed.

Here, I will outline improvements that I would make to the current implementation if I had more time:

- Implement a guidance component to have the simulated rocket follow the generated reference trajectory. I have provided below a potential architecture for this guidance controller, and given more time, would have implemented this controller.

- Use #link("https://github.com/eliotbo/bevy_plot")[bevy_plot] to plot the metrics of the algorithm and simulation in real-time.

- Implement @ChenYushu2023AFAf "A Fast Algorithm for Onboard Atmospheric Powered Descent Guidance" to improve the performance of the convex solver.

- Add improvements from @Szmuk_2018 "Successive Convexification for 6-Dof Mars Rocket Powered Landing with Free-Final-Time", which is a more recent algorithm from the same authors as the currently implemented algorithm @Szmuk2016


== Potential Guidance Controller
Once we have a suitable trajectory, we must implement a controller to follow this trajectory. A guidance controller is used within the vehicle's system alongside the trajectory optimisation algorithm, serving as a central controller that takes in the computed fuel-optimal trajectory in the form of thrust vectors as inputs and outputs actionable commands. It distributes these control inputs to the appropriate system, e.g. thrusters, pitch, yaw, etc. These systems have control loops that manage their actuators through a dedicated PID controller. Each PID controller activates its respective actuator, and feedback from the sensors is used to update the PID controller and adjust its output appropriately to reach the desired actuator state.

=== Model Predictive Control (MPC)
@mpc is the process of applying a reference trajectory to a real-world system. It has been used in many domains over the last few decades.

=== Guidance Controller Diagram
#figure(
  image("../assets/control_architecture.png", width: 80%),
  caption: [A typical architecture for a guidance controller. Source: @Malyuta2022],
) <pos_vel_chart>


#include "../code/guidance_controller_diagram.typ"

=== Conclusion
Using a convex optimised trajectory with a guidance control loop, we could ensure that the physical vehicle follows the computed trajectory with minimal deviations, robustly and safely, filling in the gap between a theoretically optimal trajectory and actual hardware action.

