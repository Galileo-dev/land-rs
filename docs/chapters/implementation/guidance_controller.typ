== Guidance Controller
Once we have a suitable trajectory, we must implement a controller to follow this trajectory. A guidance controller is used within the vehicle's system alongside the trajectory optimisation algorithm, serving as a central controller that takes in the computed fuel-optimal trajectory in the form of thrust vectors as inputs and outputs actionable commands. It distributes these control inputs to the appropriate system, e.g. thrusters, pitch, yaw, etc. These systems have control loops that manage their actuators through a dedicated PID controller. Each PID controller activates its respective actuator, and feedback from the sensors is used to update the PID controller so it can adjust its output appropriately to reach t, a certain angle, thrust, etc.he desired actuator state.


=== PID Controller
// TODO: add math and explaination of a PID controller


=== Guidance Controller Diagram
#include "../../code/guidance_controller_diagram.typ"


