= Guidance Controller
Once we have a suitable trajectory, we must implement a controller to follow this trajectory. A guidance controller is used within the vehicle's system alongside the trajectory optimisation algorithm, serving as a central controller that takes in the computed fuel-optimal trajectory in the form of thrust vectors as inputs and outputs actionable commands. It distributes these control inputs to the appropriate system, e.g. thrusters, pitch, yaw, etc. These systems have control loops that manage their actuators through a dedicated PID controller. Each PID controller activates its respective actuator, and feedback from the sensors is used to update the PID controller and adjust its output appropriately to reach the desired actuator state.

=== Model Predictive Control (MPC)
@mpc is the process of applying a reference trajectory to a real-world system. It has been used in many domains over the last few decades.


=== PID Controller
// TODO: add math and explanation of a PID controller


=== Guidance Controller Diagram
#figure(
  image("../../assets/control_architecture.png", width: 80%),
  caption: [A typical architecture for ],
) <pos_vel_chart>


#include "../../code/guidance_controller_diagram.typ"


